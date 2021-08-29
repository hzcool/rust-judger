/*
    1. 忽略所有的空格
    2. 所有的数字满足浮点精度 epsilon 的要求
    3. 非数字要一致
*/
use super::checker::CheckResult;
use super::standard::get_std_and_process_output;
use crate::types::result::JudgeStatus;

// mode = 0
fn verify_absolute(std_float: f64, process_float: f64, epsilon: f64) -> bool {
    (std_float - process_float).abs() <= epsilon
}

// mode = others
fn verify_relative(std_float: f64, process_float: f64, epsilon: f64) -> bool {
    let mut mn = std_float * (1f64 - epsilon);
    let mut mx = std_float * (1f64 + epsilon);
    if mn > mx {
        std::mem::swap(&mut mn, &mut mx);
    }
    mn <= process_float && process_float <= mx
}

fn floats_check(std_output: &str, process_output: &str, mode: i32, epsilon: f64) -> CheckResult {
    let std_words: Vec<&str> = std_output.split_whitespace().collect();
    let process_words: Vec<&str> = process_output.split_whitespace().collect();
    if std_words.len() != process_words.len() {
        return (
            JudgeStatus::WrongAnswer,
            format!("the number of words is wrong"),
        );
    }
    let verify = match mode {
        0 => |a: f64, b: f64, c: f64| verify_absolute(a, b, c),
        _ => |a: f64, b: f64, c: f64| verify_relative(a, b, c),
    };
    for i in 0..std_words.len() {
        if std_words[i] != process_words[i] {
            let std_float_res = std_words[i].parse::<f64>();
            let process_float_res = process_words[i].parse::<f64>();
            if std_float_res.is_err()
                || process_float_res.is_err()
                || !verify(std_float_res.unwrap(), process_float_res.unwrap(), epsilon)
            {
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
    }
    (
        JudgeStatus::Accepted,
        format!("Ok {} words", std_words.len()),
    )
}

pub fn check(
    std_output_path: &str,
    process_output_path: &str,
    mode: i32,
    epsilon: f64,
) -> CheckResult {
    match get_std_and_process_output(std_output_path, process_output_path) {
        Err(e) => e,
        Ok((std_output, process_output)) => {
            floats_check(std_output.as_str(), process_output.as_str(), mode, epsilon)
        }
    }
}

mod tests {
    use super::check;

    #[test]
    fn test_check() {
        let std_path = "/home/hzcool/Code/Rust/rust-judger/src/1.txt";
        let process_path = "/home/hzcool/Code/Rust/rust-judger/src/2.txt";
        let res = check(std_path, process_path, 0, 1e-2);
        println!("{:?} {}", res.0, res.1);
    }
}
