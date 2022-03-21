use crate::types::result::JudgeStatus;
use serde::{Deserialize, Serialize};
use serde_json::Value as Json;
pub type CheckResult = (JudgeStatus, String);

const MAX_LINE_CHARS_SHOW: usize = 128;
pub fn line_show(line: &str) -> String {
    match line.len() < MAX_LINE_CHARS_SHOW {
        true => line.to_string(),
        false => format!("{}...", &line[0..MAX_LINE_CHARS_SHOW]),
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Checker {
    typ: String,
    mode: Option<i32>,
    epsilon: Option<f64>,
}

impl Default for Checker {
    fn default() -> Self {
        Self {
            typ: "standard".to_string(),
            mode: None,
            epsilon: None,
        }
    }
}

impl Checker {
    pub fn from_json(json_opt: Option<Json>) -> Result<Self, String> {
        match json_opt {
            None => Ok(Checker::default()),
            Some(json) => serde_json::from_value(json).map_err(|e| format!("{}", e)),
        }
    }
    pub fn check(&self, std_output_path: &str, process_output_path: &str) -> CheckResult {
        match self.typ.as_str() {
            "standard" => super::standard::check(std_output_path, process_output_path),
            "floats" => super::floats::check(
                std_output_path,
                process_output_path,
                self.mode.unwrap_or(0),
                self.epsilon.unwrap_or(1e-9),
            ),
            "ignore_whitespaces" => {
                super::ignore_whitespaces::check(std_output_path, process_output_path)
            }

            "identical" => super::identical::check(std_output_path, process_output_path),
            "unordered" => super::unordered::check(std_output_path, process_output_path),
            _ => (
                JudgeStatus::SystemError,
                format!("没有类型为 `{}` 的 checker", self.typ),
            ),
        }
    }
}
