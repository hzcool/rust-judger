#ifndef RUNNER_H
#define RUNNER_H

#include "args_parser/parser.hpp"
class Runner{
private:
    char* run_command;
    int max_cpu_time;
    int max_memory;
    int max_output_size;
    char* input_path; // 测试数据输入文件路径
    char* output_path; //结果输出的文件路径
    char* seccomp_rule;
    int resource_rule;
    Parser p;
    void child_process();

public:
    void run();
    Runner(int argc, const char* argv[]);
};

#endif