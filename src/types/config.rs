use lazy_static::lazy_static;
use std::collections::HashMap;
use std::path::PathBuf;

pub type CompileConfigMap = HashMap<&'static str, CompileConfig>;
pub type SpjCompileConfigMap = HashMap<&'static str, SpjCompileConfig>;

lazy_static! {
    pub static ref BASE_PATH: PathBuf =
        PathBuf::from(std::env::var("BASE_PATH").expect("缺少环境变量 : `BASE_PATH` !!!"));
    pub static ref RUNNER_PATH: PathBuf =
        PathBuf::from(std::env::var("BASE_PATH").unwrap()).join("src/cpp_runner/runner");
    pub static ref TMP_DIR_PATH: PathBuf =
        PathBuf::from(std::env::var("BASE_PATH").unwrap()).join("src/tmp");
    pub static ref TEST_CASE_DIR: PathBuf =
        PathBuf::from(std::env::var("TEST_CASE_DIR").expect("缺少环境变量 : `TEST_CASE_DIR` !!!"));
    pub static ref CPU_CORES_COUNT: usize = {
        let x = std::process::Command::new("grep").args(["core id", "/proc/cpuinfo"]).output().expect("获取 CPU 核心数失败!!!");
        std::cmp::max((String::from_utf8(x.stdout).unwrap().lines().count()) / 2, 1)
    };
}

use crate::checker::checker::Checker;
use serde::{Deserialize, Serialize};

pub struct CompileConfig {
    pub src_name: &'static str,
    pub exe_name: &'static str,
    pub compile_command: &'static str,
    pub run_command: &'static str,
    pub seccomp_rule: &'static str,
    pub resource_rule: i8,
}

pub struct SpjCompileConfig {
    pub src_name: &'static str,
    pub exe_name: &'static str,
    pub compile_command: &'static str,
    pub run_command: &'static str,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TestCase {
    pub id: i32,
    pub input_name: String,
    pub output_name: String,
    pub process_output_path: Option<String>,
    pub max_output_size: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SpjConfig {
    pub spj_lang: String,
    pub spj_src: String,
    pub run_command: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JudgeConfig {
    pub lang: String, //语言

    pub src: String, //源码

    pub max_cpu_time: i32, //最大cpu时间

    pub max_memory: i32, //最大内存

    pub io_dir: String, // 测试用例文件夹

    pub test_cases: Vec<TestCase>, //测试用例

    pub checker: Option<Checker>, // 检查器

    pub spj_config: Option<SpjConfig>, // special judger

    pub seccomp_rule: Option<String>, //  权限规则

    pub resource_rule: Option<i8>, // 资源限制规则

    pub run_command: Option<String>, //生成的运行规则
}

// const RUN_CMD_FMT: &str = "{} -c \"{}\" -t {} -m {} -o {} -I {} -O {} -r {}";

const C99: CompileConfig = CompileConfig {
    src_name: "main.c",
    exe_name: "main",
    compile_command: "/usr/bin/gcc -std=c99 -O2 -lm -w -fmax-errors=3 {src_path} -o {exe_path}",
    run_command: "{exe_path}",
    seccomp_rule: "c_cpp",
    resource_rule: 7,
};
const C11: CompileConfig = CompileConfig {
    src_name: "main.c",
    exe_name: "main",
    compile_command: "/usr/bin/gcc -std=c11 -O2 -lm -w -fmax-errors=3 {src_path} -o {exe_path}",
    run_command: "{exe_path}",
    seccomp_rule: "c_cpp",
    resource_rule: 7,
};
const CPP11: CompileConfig = CompileConfig {
    src_name: "main.cpp",
    exe_name: "main",
    compile_command: "/usr/bin/g++ -std=c++11 -O2 -lm -w -fmax-errors=3 {src_path} -o {exe_path}",
    run_command: "{exe_path}",
    seccomp_rule: "c_cpp",
    resource_rule: 7,
};
const CPP14: CompileConfig = CompileConfig {
    src_name: "main.cpp",
    exe_name: "main",
    compile_command: "/usr/bin/g++ -std=c++14 -O2 -lm -w -fmax-errors=3 {src_path} -o {exe_path}",
    run_command: "{exe_path}",
    seccomp_rule: "c_cpp",
    resource_rule: 7,
};
const CPP17: CompileConfig = CompileConfig {
    src_name: "main.cpp",
    exe_name: "main",
    compile_command: "/usr/bin/g++ -std=c++17 -O2 -lm -w -fmax-errors=3 {src_path} -o {exe_path}",
    run_command: "{exe_path}",
    seccomp_rule: "c_cpp",
    resource_rule: 7,
};
const PYTHON2: CompileConfig = CompileConfig {
    src_name: "solution.py",
    exe_name: "solution.pyc",
    compile_command: "/usr/bin/python2 -m py_compile {src_path}",
    run_command: "/usr/bin/python2 {exe_path}",
    seccomp_rule: "general",
    resource_rule: 7,
};
const PYTHON3: CompileConfig = CompileConfig {
    src_name: "solution.py",
    exe_name: "__pycache__/solution.cpython-38.pyc",
    compile_command: "/usr/bin/python3 -m py_compile {src_path}",
    run_command: "/usr/bin/python3 {exe_path}",
    seccomp_rule: "general",
    resource_rule: 7,
};
const JAVA: CompileConfig = CompileConfig {
    src_name: "Main.java",
    exe_name: "Main",
    compile_command: "/usr/bin/javac {src_path} -d {exe_dir} -encoding UTF8",
    run_command: "/usr/bin/java -cp {exe_dir} -Xss1M -Xms16M -Xmx{max_memory}k -Djava.security.manager -Dfile.encoding=UTF-8 -Djava.security.policy==/etc/java_policy -Djava.awt.headless=true Main",
    seccomp_rule: "none",
    resource_rule: 5,
};

pub fn make_compile_config_map() -> CompileConfigMap {
    let mut mp: CompileConfigMap = HashMap::new();
    mp.insert("C", C11);
    mp.insert("C99", C99);
    mp.insert("C11", C11);
    mp.insert("C++", CPP11);
    mp.insert("C++11", CPP11);
    mp.insert("C++14", CPP14);
    mp.insert("C++17", CPP17);
    mp.insert("Python", PYTHON3);
    mp.insert("Python2", PYTHON2);
    mp.insert("Python3", PYTHON3);
    mp.insert("Java", JAVA);
    mp
}

pub fn make_spj_compile_config_map() -> SpjCompileConfigMap {
    let mut mp: SpjCompileConfigMap = HashMap::new();
    mp.insert("C", SPJ_C);
    mp.insert("C++", SPJ_CPP);
    mp.insert("Python", SPJ_PYTHON);
    mp
}

const SPJ_C: SpjCompileConfig = SpjCompileConfig {
    src_name: "spj_main.c",
    exe_name: "spj_main",
    compile_command:
    "/usr/bin/gcc -std=c11 -O2 -lm -w -fmax-errors=3 {spj_src_path} -o {spj_exe_path}",
    run_command: "{spj_exe_path}",
    //每个特判指令对每个样例需要接三个参数 -i {input_path} -o {output_path} -p {process_output_path}
};

const SPJ_CPP: SpjCompileConfig = SpjCompileConfig {
    src_name: "spj_main.cpp",
    exe_name: "spj_main",
    compile_command:
    "/usr/bin/g++ -std=c++17 -O2 -lm -w -fmax-errors=3 {spj_src_path} -o {spj_exe_path}",
    run_command: "{spj_exe_path}",
    //每个特判指令对每个样例需要接三个参数 -i {input_path} -o {output_path} -p {process_output_path}
};

const SPJ_PYTHON: SpjCompileConfig = SpjCompileConfig {
    src_name: "spj_solution.py",
    exe_name: "__pycache__/spj_solution.cpython-38.pyc",
    compile_command: "/usr/bin/python3 -m py_compile {spj_src_path}",
    run_command: "/usr/bin/python3 {spj_exe_path}",
    //每个特判指令对每个样例需要接三个参数 -i {input_path}  -o {output_path} -p {process_output_path}
};
