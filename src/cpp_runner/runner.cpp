#include <unistd.h>
#include <cstring>
#include <thread>
#include <sys/wait.h>
#include <sys/resource.h>
#include <iostream>


#include "runner.h"
#include "rule.h"
#include "status.h"
#include "result.h"

char* copy_c_str(const char* s) {
    char* p = new char[strlen(s) + 1];
    strcpy(p, s);
    return p;
};


Runner::Runner(int argc, const char* argv[]) {
    p.add("--run_command", "-c", Parser::ArgType::STRING, 0, "");
    p.add("--max_cpu_time", "-t", Parser::ArgType::INT, 0,"");
    p.add("--max_memory", "-m", Parser::ArgType::INT, 0, "");
    p.add("--max_output_size", "-o", Parser::ArgType::INT, 0, "");
    p.add("--input_path", "-I", Parser::ArgType::STRING, 0, "");
    p.add("--output_path", "-O", Parser::ArgType::STRING, 0, "");
    p.add("--seccomp_rule", "-r", Parser::ArgType::STRING, 0, "");
    p.add("--resource_rule", "-s", Parser::ArgType::INT, 0, "");
    p.parse(argc, argv);


    Parser::Arguments args = p["--run_command"];
    if (args.data.strings.empty()) {
        puts(json_result(ARGS_ERROR).c_str());
        return;
    } else {
        run_command = copy_c_str(args.data.strings[0].c_str());
    }

    args = p["--max_cpu_time"];
    if (args.data.integers.empty()) max_cpu_time = 20000;
    else max_cpu_time = args.data.integers[0];

    args = p["--max_memory"];
    if (args.data.integers.empty()) max_memory = 1024 * 1024 * 1024;
    else max_memory = args.data.integers[0];
     
    args = p["--max_output_size"];
    if (args.data.integers.empty()) max_output_size = 64 * 1024 * 1024;
    else max_output_size = args.data.integers[0];

    args = p["--input_path"];
    if (args.data.strings.empty()) {
        input_path = nullptr;
    } else {
        input_path = copy_c_str(args.data.strings[0].c_str());
    }

    args = p["--output_path"];
    if (args.data.strings.empty()) {
        puts(json_result(ARGS_ERROR).c_str());
        return;
    } else {
        output_path = copy_c_str(args.data.strings[0].c_str());
    }

    args = p["--seccomp_rule"];
    if (args.data.strings.empty()) {
        seccomp_rule = copy_c_str("c_cpp");
    } else {
        seccomp_rule = copy_c_str(args.data.strings[0].c_str());
        if(strcmp(seccomp_rule, "c_cpp") != 0 && strcmp(seccomp_rule, "general") != 0 && strcmp(seccomp_rule, "none") != 0) {
            puts(json_result(ARGS_ERROR).c_str());
            return;
        }
    }

    args = p["--resource_rule"];
    if(args.data.integers.empty()) {
        resource_rule = 7;
    } else {
        resource_rule = args.data.integers[0];
        if (resource_rule > 7) {
            puts(json_result(ARGS_ERROR).c_str());
            return;
        }
    }
}


void Runner::run() {
    pid_t pid = fork();
    if(pid==0) {
        child_process();
    } else {
        // 子进程监视，超过real_time就kill
        std::thread monitor([](pid_t child, int sec) {
           sleep(sec);
           kill(child, SIGXCPU); 
        }, pid, max_cpu_time / 1000 + 1);
        monitor.detach();
        int status;
        struct rusage resource_usage;
        wait4(pid, &status, 0, &resource_usage);
        int cpu_time = std::max((int)(resource_usage.ru_utime.tv_sec * 1000 +
                                resource_usage.ru_utime.tv_usec / 1000)- 2, 0);

        int memory = resource_usage.ru_maxrss * 1024;
        int retStatus = WEXITSTATUS(status);
        if(retStatus < 20) {
            int signal =  WTERMSIG(status);
            if(signal == SIGXFSZ) retStatus = OUTPUT_LIMIT_EXCEEDED;
            else if(signal == SIGXCPU || cpu_time > max_cpu_time) retStatus = TIME_LIMIT_EXCEEDED;
            else if(memory > max_memory) retStatus = MEMORY_LIMIT_EXCEEDED;
            else if(!WIFEXITED(status)) retStatus = RUNTIME_ERROR;
            else retStatus = OK;
        }
        puts(json_result((Status)retStatus, cpu_time, memory).c_str());
    }
}

void Runner::child_process() {
    

    if(!freopen(input_path, "r", stdin)) exit(OPEN_INPUT_FILE_FAIL);
    if(!freopen(output_path, "w", stdout)) exit(OPEN_OUTPUT_FILE_FAIL);
    if(!resource_init( (resource_rule & 1) > 0? max_cpu_time : -1, (resource_rule & 2) > 0 ? max_memory : -1, (resource_rule & 4) > 0 ? max_output_size : -1)) exit(RESOURCE_INIT_ERROR);
    
    
    char* argv[20]; int k = 0;
    const char* spc = " \n\t\r";
    argv[k] = strtok(run_command, spc);
    while(argv[k]) argv[++k] = strtok(NULL, spc); 
    char* envp[] = {NULL};
    
    if(strcmp(seccomp_rule, "none") != 0) {
        if(strcmp(seccomp_rule, "c_cpp") == 0) {
            if(!c_cpp_seccomp_rules_init(argv[0])) exit(SECCOMP_INIT_ERROR);
        } else {
            if(!general_rules_init(argv[0]))  exit(SECCOMP_INIT_ERROR);
        }
    }
    execve(argv[0], argv, envp);
    exit(RUN_COMMAND_FAIL);
}