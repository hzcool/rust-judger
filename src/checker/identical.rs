// 忽略行末尾空格和结尾空行

use super::checker::CheckResult;
use super::standard::get_std_and_process_output;
use crate::types::result::JudgeStatus;

fn identical_check(std_output: &str, process_output: &str) -> CheckResult {
    let mut std_lines = std_output.trim_end().lines();
    let mut process_lines = process_output.trim_end().lines();
    let mut line_count = 0;
    loop {
        let std_line_opt = std_lines.next();
        let process_line_opt = process_lines.next();
        if std_line_opt.is_none() && process_line_opt.is_none() {
            break;
        }
        line_count += 1;
        let suffix_trimed_std_line = std_line_opt.unwrap_or("").trim_end();
        let suffix_trimed_process_line = process_line_opt.unwrap_or("").trim_end();
        if suffix_trimed_std_line != suffix_trimed_process_line {
            return (
                JudgeStatus::WrongAnswer,
                format!(
                    "different at {}th line, expected `{}`, but found `{}` ",
                    line_count, suffix_trimed_std_line, suffix_trimed_process_line
                ),
            );
        }
    }
    (JudgeStatus::Accepted, format!("Ok {} lines", line_count))
}

pub fn check(std_output_path: &str, process_output_path: &str) -> CheckResult {
    match get_std_and_process_output(std_output_path, process_output_path) {
        Err(e) => e,
        Ok((std_output, process_output)) => {
            identical_check(std_output.as_str(), process_output.as_str())
        }
    }
}

mod tests {
    use super::check;
    #[test]
    fn test_check() {
        let std_path = "/home/hzcool/Code/Rust/rust-judger/src/1.txt";
        let process_path = "/home/hzcool/Code/Rust/rust-judger/src/2.txt";
        let res = check(std_path, process_path);
        println!("{:?} {}", res.0, res.1);
    }
}
