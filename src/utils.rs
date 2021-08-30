use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::process;

pub fn read_file(src: &str) -> std::io::Result<String> {
    let file = File::open(src)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn write_to_file(path: &Path, content: &str) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    Ok(file.write_all(content.as_bytes())?)
}

pub fn run_cmd(cmd: &str) -> std::io::Result<process::Output> {
    process::Command::new("sh").arg("-c").arg(cmd).output()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_file() {
        let x = read_file("/home/hzcool/Code/Rust/rust-judger/src/1.txt").unwrap();
        assert_eq!("1 2 3", x);
    }
}
