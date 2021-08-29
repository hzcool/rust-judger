use super::result::{JudgeResult, JudgeStatus};
use actix_web::{http::StatusCode, ResponseError};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Response {
    pub compile_info: String,
    //编译信息
    pub case_count: u16,
    //用例总数
    pub pass_count: u16,
    //通过的数量
    pub time: u32,
    //同时
    pub memory: u32,
    //内存消耗
    pub total_time: u32,
    //所有样例总用时
    pub status: JudgeStatus,
    //结果
    pub error: String,
    //出现的错误
    pub results: Vec<JudgeResult>, //所有的样例结果
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let v = serde_json::json!(self);
        write!(f, "{}", v.to_string())
    }
}

impl ResponseError for Response {
    fn status_code(&self) -> StatusCode {
        match self.status {
            JudgeStatus::BadRequst => StatusCode::BAD_REQUEST,
            _ => StatusCode::OK,
        }
    }
}

pub fn compile_error_response(info: String) -> Response {
    let mut res = Response::default();
    res.compile_info = info;
    res.status = JudgeStatus::CompileError;
    res
}

pub fn system_error_response(info: String) -> Response {
    let mut res = Response::default();
    res.error = info;
    res.status = JudgeStatus::SystemError;
    res
}

pub fn bad_request_response(info: String) -> Response {
    let mut res = Response::default();
    res.error = info;
    res.status = JudgeStatus::BadRequst;
    res
}


