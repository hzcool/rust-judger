use crate::types::config::{JudgeConfig, TestCase};
use crate::types::response::Response;
use crate::types::result::{JudgeResult, JudgeStatus, SpjJudgeResult};

use std::path::PathBuf;
use std::sync::{mpsc, Arc};


fn run_one_test_case(config: Arc<JudgeConfig>, test_case: &TestCase) -> JudgeResult {
    let program = crate::types::config::RUNNER_PATH.to_str().unwrap();
    let io_dir_path = PathBuf::from(config.io_dir.as_str());
    let input_path = io_dir_path.join(test_case.input_name.as_str());
    let output_path = io_dir_path.join(test_case.output_name.as_str());
    let _args = vec![
        "-c".to_string(),
        format!("{}", config.run_command.as_ref().unwrap()),
        "-t".to_string(),
        format!("{}", config.max_cpu_time),
        "-m".to_string(),
        format!("{}", config.max_memory),
        "-o".to_string(),
        format!("{}", test_case.max_output_size.unwrap_or(-1)),
        "-I".to_string(),
        format!("{}", input_path.to_str().unwrap()),
        "-O".to_string(),
        format!("{}", test_case.process_output_path.as_ref().unwrap()),
        "-r".to_string(),
        format!("{}", config.seccomp_rule.as_ref().unwrap()),
        "-s".to_string(),
        format!("{}", config.resource_rule.unwrap_or(7 as i8)),
    ];


    let args = _args.iter().map(|x| x.as_str()).collect::<Vec<&str>>();
    match std::process::Command::new(program)
        .args(args)
        .output()
    {
        Err(err) => JudgeResult::from_system_err(Some(test_case.id), err.to_string()),
        Ok(output) => {
            let _res = String::from_utf8(output.stdout).unwrap();
            match serde_json::from_str::<JudgeResult>(_res.as_str()) {
                Ok(mut res) => {
                    res.id = Some(test_case.id);
                    if let JudgeStatus::Accepted = res.status {
                        //特判
                        if let Some(spj_config) = &config.spj_config {
                            let args_vec: Vec<&str> = spj_config.run_command.as_ref().unwrap().as_str().split_whitespace().map(|x| x).collect();
                            let args: &[&str] = &[args_vec.as_slice(), &[
                                "-i",
                                input_path.to_str().unwrap(),
                                "-o",
                                output_path.to_str().unwrap(),
                                "-p",
                                test_case.process_output_path.as_ref().unwrap().as_str(),
                            ]].concat();
                            match std::process::Command::new(args[0]).args(&args[1..]).output() {
                                Err(err) => {
                                    res.status = JudgeStatus::SystemError;
                                    res.info = format!("运行特判文件出错, 错误信息 : {}", err.to_string())
                                }
                                Ok(output) => {
                                    if !output.stderr.is_empty() {
                                        res.status = JudgeStatus::SystemError;
                                        res.info = format!("运行特判文件出错, 错误信息 : {}", String::from_utf8(output.stderr).unwrap());
                                    } else {
                                        let _spj_res = String::from_utf8(output.stdout).unwrap();
                                        match serde_json::from_str::<SpjJudgeResult>(_spj_res.as_str()) {
                                            Ok(spj_res) => {
                                                res.status = spj_res.status;
                                                res.info = spj_res.info;
                                            }
                                            Err(err2) => {
                                                res.status = JudgeStatus::SystemError;
                                                res.info = format!("特判信息的输出结果解析失败, 错误信息: {}", err2.to_string());
                                            }
                                        }
                                    }
                                }
                            }
                        } else {
                            let check_result = config.checker.as_ref().unwrap().check(output_path.to_str().unwrap(), test_case.process_output_path.as_ref().unwrap().as_str());
                            res.status = check_result.0;
                            res.info = check_result.1;
                        }
                    }
                    res
                }
                Err(err) => JudgeResult::from_system_err(Some(test_case.id), err.to_string()),
            }
        }
    }
}

pub fn run(config: JudgeConfig) -> Result<Response, Response> {
    let config = Arc::new(config);
    let (tx, rx) = mpsc::channel();
    for i in 0..config.test_cases.len() {
        let conf = config.clone();
        let atx = mpsc::Sender::clone(&tx);
        std::thread::spawn(move || atx.send(run_one_test_case(conf.clone(), &conf.test_cases[i])));
    }
    let mut response = Response::default();
    response.case_count = config.test_cases.len() as u16;
    let mut count = 0usize;
    while count < config.test_cases.len() {
        let res = rx
            .recv()
            .unwrap_or(JudgeResult::system_error_result(format!("系统错误")));
        count += 1;
        response.results.push(res)
    }

    // 总和答案,
    response.results.sort_unstable_by(|x, y| {
        x.id.unwrap_or(-1).cmp(&y.id.unwrap_or(-1))
    });

    for item in &response.results {
        if item.status.get_i32() > response.status.get_i32() {
            response.status = item.status;
        }
        if let JudgeStatus::Accepted = item.status {
            response.pass_count += 1
        }
        if item.cpu_time > response.time {
            response.time = item.cpu_time
        }
        if item.memory > response.memory {
            response.memory = item.memory
        }
        response.total_time += item.cpu_time
    }
    Ok(response)
}
