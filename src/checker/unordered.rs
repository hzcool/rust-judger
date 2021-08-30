/*
    1. 忽略所有的空白字符
    2. 按照字典序排好每个词必须一致
*/
use super::checker::CheckResult;
use super::standard::get_std_and_process_output;
use crate::types::result::JudgeStatus;

fn unordered_check(std_output: &str, process_output: &str) -> CheckResult {
    let mut std_words: Vec<String> = std_output
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();
    let mut process_words: Vec<String> = process_output
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();

    if std_words.len() != process_words.len() {
        return (
            JudgeStatus::WrongAnswer,
            format!("the number of words is wrong"),
        );
    }

    std_words.sort();
    process_words.sort();

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
            unordered_check(std_output.as_str(), process_output.as_str())
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
