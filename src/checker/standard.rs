//忽略空行和末尾空白字符
use super::checker::CheckResult;
use crate::types::result::JudgeStatus;
use crate::utils;
use std::str::Lines;

pub fn next_not_empty_line<'a>(lines: &'a mut Lines) -> &'a str {
    loop {
        match lines.next() {
            None => return "",
            Some(mut s) => {
                s = s.trim_end();
                if s != "" {
                    return s;
                }
            }
        }
    }
}

pub fn get_std_and_process_output(
    std_output_path: &str,
    process_output_path: &str,
) -> Result<(String, String), CheckResult> {
    Ok((
        utils::read_file(std_output_path)
            .map_err(|e| (JudgeStatus::SystemError, format!("{}", e)))?,
        utils::read_file(process_output_path)
            .map_err(|e| (JudgeStatus::SystemError, format!("{}", e)))?,
    ))
}

pub fn standard_checker(std_output: &str, process_output: &str) -> CheckResult {
    let mut std_lines = std_output.lines();
    let mut process_lines = process_output.lines();
    let mut line_count = 0;
    loop {
        let std_line = next_not_empty_line(&mut std_lines);
        let process_line = next_not_empty_line(&mut process_lines);
        if std_line == "" && process_line == "" {
            break;
        }
        line_count += 1;
        let suffix_trimed_std_line = std_line.trim_end();
        let suffix_trimed_process_line = process_line.trim_end();
        if suffix_trimed_std_line != suffix_trimed_process_line {
            return (
                JudgeStatus::WrongAnswer,
                format!(
                    "different at {}th line, expected `{}`, but found `{}`",
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
            standard_checker(std_output.as_str(), process_output.as_str())
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
