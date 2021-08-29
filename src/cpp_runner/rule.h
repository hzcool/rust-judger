#ifndef RULE_H
#define RULE_H

// seccomp 权限控制
bool c_cpp_seccomp_rules_init(char* exe_path);
bool general_rules_init(char* exe_path);


//资源控制
bool resource_init(int max_cpu_time, int max_memory,int max_output_size);

#endif