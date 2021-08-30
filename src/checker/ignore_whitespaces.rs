// 忽略所有的空格

use super::checker::CheckResult;
use super::standard::get_std_and_process_output;
use crate::types::result::JudgeStatus;

fn ignore_whitespaces_check(std_output: &str, process_output: &str) -> CheckResult {
    let std_words: Vec<&str> = std_output.split_whitespace().collect();
    let process_words: Vec<&str> = process_output.split_whitespace().collect();
    if std_words.len() != process_words.len() {
        return (
            JudgeStatus::WrongAnswer,
            format!("the number of words is wrong"),
        );
    }
    for i in 0..std_words.len() {
        if std_words[i] != process_words[i] {
            return (
                JudgeStatus::WrongAnswer,
                format!(
                    "different at the {}th word, expected `{}`, but found `{}`",
                    i + 1,
                    std_words[i],
                    process_words[i]
                ),
            );
        }
    }
    (
        JudgeStatus::Accepted,
        format!("Ok {} words", std_words.len()),
    )
}

pub fn check(std_output_path: &str, process_output_path: &str) -> CheckResult {
    match get_std_and_process_output(std_output_path, process_output_path) {
        Err(e) => e,
        Ok((std_output, process_output)) => {
            ignore_whitespaces_check(std_output.as_str(), process_output.as_str())
        }
    }
}

#[cfg(test)]
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
