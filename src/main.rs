use actix_service::Service;
use actix_web::{
    guard, post,
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder,
};

pub mod checker;
pub mod runner;
pub mod types;
pub mod utils;

use types::config::{CompileConfigMap, JudgeConfig, SpjCompileConfigMap};
use types::response::{
    bad_request_response, compile_error_response, system_error_response, Response,
};


struct AppData {
    access_token: String,
    ping_info: String,
}

#[post("/ping")]
async fn ping(data: web::Data<AppData>) -> impl Responder {
    data.ping_info.to_string()
}

#[post("/judge")]
async fn judge(
    data: web::Json<JudgeConfig>,
    compile_config_map: web::Data<CompileConfigMap>,
    spj_compile_config_map: web::Data<SpjCompileConfigMap>,
) -> Result<HttpResponse, Response> {
    let mut run_config = data.into_inner();
    let compile_config = match compile_config_map.as_ref().get(run_config.lang.as_str()) {
        None => {
            return Err(bad_request_response(format!(
                "不支持语言 `{}` ",
                run_config.lang,
            )));
        }
        Some(c) => c,
    };

    let temp_dir = tempfile::tempdir()
        .map_err(|err| system_error_response(format!("新建临时文件夹错误 : {}", err)))?;
    let exe_dir = temp_dir.path().to_path_buf();
    // let exe_dir = std::path::PathBuf::from("/home/hzcool/Code/Rust/rust-judger/src/tmp");
    let src_path = exe_dir.join(compile_config.src_name);
    let exe_path = exe_dir.join(compile_config.exe_name);

    // 写入源代码
    utils::write_to_file(src_path.as_path(), run_config.src.as_str())
        .map_err(|err| system_error_response(format!("写入源代码失败 : {}", err)))?;

    // 构造编译指令
    let mut compile_cmd = String::from(compile_config.compile_command);
    compile_cmd = compile_cmd
        .replace("{exe_dir}", exe_dir.to_str().unwrap())
        .replace("{src_path}", src_path.to_str().unwrap())
        .replace("{exe_path}", exe_path.to_str().unwrap());

    // 编译
    let compile_info = match utils::run_cmd(compile_cmd.as_str()) {
        Err(err) => return Err(system_error_response(format!("编译时出错 : {}", err))),
        Ok(output) => match output.stderr.is_empty() {
            false => {
                return Err(compile_error_response(format!(
                    "{}",
                    String::from_utf8(output.stderr).unwrap()
                )));
            }
            true => String::from_utf8(output.stdout).unwrap(),
        },
    };

    //构造运行指令
    let run_cmd = String::from(compile_config.run_command);
    run_config.run_command = Some(
        match run_config.lang.as_str() {
            "Java" => run_cmd
                .replace("{exe_dir}", exe_dir.to_str().unwrap())
                .replace("{max_memory}", format!("{}", run_config.max_memory / 1024).as_str()),
            _ => run_cmd.replace("{exe_path}", exe_path.to_str().unwrap())
        }
    );

    //special judge 编译checker代码
    if let Some(spj_config) = &mut run_config.spj_config {
        let spj_compile_config = match spj_compile_config_map.as_ref().get(spj_config.spj_lang.as_str()) {
            None => return Err(bad_request_response(format!(
                "不支特特判语言 `{}` ",
                spj_config.spj_lang,
            ))),
            Some(c) => c
        };

        let spj_src_path = exe_dir.join(spj_compile_config.src_name);
        let spj_exe_path = exe_dir.join(spj_compile_config.exe_name);

        // 写入特判代码
        utils::write_to_file(spj_src_path.as_path(), spj_config.spj_src.as_str())
            .map_err(|err| system_error_response(format!("写入特判代码失败 : {}", err)))?;

        // 编译spj_src
        let spj_compile_cmd = String::from(spj_compile_config.compile_command)
            .replace("{spj_src_path}", spj_src_path.to_str().unwrap())
            .replace("{spj_exe_path}", spj_exe_path.to_str().unwrap());
        match utils::run_cmd(spj_compile_cmd.as_str()) {
            Err(err) => return Err(system_error_response(format!("{}", err))),
            Ok(output) => {
                if !output.stderr.is_empty() {
                    return Err(compile_error_response(format!(
                        "编译特判文件失败 : {}",
                        String::from_utf8(output.stderr).unwrap()
                    )));
                }
            }
        }

        // 构造spj运行指令
        spj_config.run_command = Some(
            spj_compile_config.run_command
                .replace("{spj_exe_path}", spj_exe_path.to_str().unwrap()),
        )
    }


    // 代码输出文件
    for t in run_config.test_cases.iter_mut() {
        let process_output_path = exe_dir.join(format!("{}.out", t.id));
        t.process_output_path = Some(process_output_path.to_str().unwrap().to_string());
    }

    //设置 seccomp
    if run_config.seccomp_rule.is_none() {
        run_config.seccomp_rule = Some(compile_config.seccomp_rule.to_string())
    }

    run_config.resource_rule = Some(compile_config.resource_rule);

    // 设置checker
    if run_config.checker.is_none() {
        run_config.checker = Some(checker::checker::Checker::default());
    }


    // run
    let mut res = runner::runner::run(run_config)?;
    res.compile_info = compile_info;
    Ok(HttpResponse::Ok().json(res))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // dotenv::dotenv().expect("Failed to read .env file");
    let app_data = Data::new(AppData {
        access_token: std::env::var("ACCESS_TOKEN").expect("can't get find ACCESS_TOKEN !!!"),
        ping_info: utils::read_file(types::config::BASE_PATH.join("src/ping.txt").to_str().unwrap())?,
    });
    let addr = std::env::var("ADDR").expect("can't find ADDR !!!");

    let server = HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .app_data(Data::new(types::config::make_compile_config_map()))
            .app_data(Data::new(types::config::make_spj_compile_config_map()))
            .wrap_fn(|req, srv| {
                if let Some(token_header) = req.headers().get("ACCESS_TOKEN") {
                    if let Ok(access_token) = token_header.to_str() {
                        let data: &web::Data<AppData> = req.app_data().unwrap();
                        if access_token == data.access_token {
                            return srv.call(req);
                        }
                    }
                }
                Box::pin(async move { Ok(req.into_response(HttpResponse::Unauthorized())) })
            })
            .service(
                web::scope("/")
                    .guard(guard::Header("Content-Type", "application/json"))
                    .service(ping)
                    .service(judge),
            )
    })
        .bind(addr.as_str())?
        .run();
    println!("服务器已启动 , addr : {}", addr);
    server.await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test;

    #[actix_rt::test]
    async fn test_ping() {
        let req = test::TestRequest::with_header("content-type", "application/json").to_http_request();
    }
}
