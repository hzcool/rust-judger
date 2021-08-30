#include "rule.h"
#include <stdio.h>
#include <seccomp.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <sys/resource.h>
#include <fcntl.h>
#include <errno.h>


bool c_cpp_seccomp_rules_init(char* exe_path) {
   int syscalls_whitelist[] = {SCMP_SYS(read), SCMP_SYS(fstat),
                                SCMP_SYS(mmap), SCMP_SYS(mprotect),
                                SCMP_SYS(munmap), SCMP_SYS(uname),
                                SCMP_SYS(arch_prctl), SCMP_SYS(brk),
                                SCMP_SYS(access), SCMP_SYS(exit_group),
                                SCMP_SYS(close), SCMP_SYS(readlink),
                                SCMP_SYS(sysinfo), SCMP_SYS(write),
                                SCMP_SYS(writev), SCMP_SYS(lseek),
                                SCMP_SYS(clock_gettime), SCMP_SYS(pread64),
                                };

    int syscalls_whitelist_length = sizeof(syscalls_whitelist) / sizeof(int);
    scmp_filter_ctx ctx = NULL;
    // load seccomp rules
    ctx = seccomp_init(SCMP_ACT_KILL);
    if (!ctx) {
        return false;
    }
    for (int i = 0; i < syscalls_whitelist_length; i++) {
        if (seccomp_rule_add(ctx, SCMP_ACT_ALLOW, syscalls_whitelist[i], 0) != 0) {
            return false;
        }
    }
//     add extra rule for execve
     if (seccomp_rule_add(ctx, SCMP_ACT_ALLOW, SCMP_SYS(execve), 0, SCMP_A0(SCMP_CMP_EQ, (scmp_datum_t)exe_path)) != 0) {
         return false;
     }
    
    // do not allow "w" and "rw"
    if (seccomp_rule_add(ctx, SCMP_ACT_ALLOW, SCMP_SYS(open), 1, SCMP_CMP(1, SCMP_CMP_MASKED_EQ, O_WRONLY | O_RDWR, 0)) != 0) {
        return false;
    }
    if (seccomp_rule_add(ctx, SCMP_ACT_ALLOW, SCMP_SYS(openat), 1, SCMP_CMP(2, SCMP_CMP_MASKED_EQ, O_WRONLY | O_RDWR, 0)) != 0) {
        return false;
    }
    
    if (seccomp_load(ctx) != 0) {
        return false;
    }
    seccomp_release(ctx);
    return true;
}   

bool general_rules_init(char* exe_path) {

    int syscalls_blacklist[] = {SCMP_SYS(clone),
                                SCMP_SYS(fork), SCMP_SYS(vfork),
                                SCMP_SYS(kill),
#ifdef __NR_execveat
                                SCMP_SYS(execveat)
#endif
                               };
    int syscalls_blacklist_length = sizeof(syscalls_blacklist) / sizeof(int);
    scmp_filter_ctx ctx = nullptr;
    // load seccomp rules
    ctx = seccomp_init(SCMP_ACT_ALLOW);
    if (!ctx) {
        return false;
    }
    for (int i = 0; i < syscalls_blacklist_length; i++) {
        if (seccomp_rule_add(ctx, SCMP_ACT_KILL, syscalls_blacklist[i], 0) != 0) {
            return false;
        }
    }
    // use SCMP_ACT_KILL for socket, python will be killed immediately
    if (seccomp_rule_add(ctx, SCMP_ACT_ERRNO(EACCES), SCMP_SYS(socket), 0) != 0) {
        return false;
    }
    // add extra rule for execve
    if (seccomp_rule_add(ctx, SCMP_ACT_KILL, SCMP_SYS(execve), 1, SCMP_A0(SCMP_CMP_NE, (scmp_datum_t)(exe_path))) != 0) {
        return false;
    }

    // do not allow "w" and "rw" using open
    if (seccomp_rule_add(ctx, SCMP_ACT_KILL, SCMP_SYS(open), 1, SCMP_CMP(1, SCMP_CMP_MASKED_EQ, O_WRONLY, O_WRONLY)) != 0) {
        return false;
    }
    if (seccomp_rule_add(ctx, SCMP_ACT_KILL, SCMP_SYS(open), 1, SCMP_CMP(1, SCMP_CMP_MASKED_EQ, O_RDWR, O_RDWR)) != 0) {
        return false;
    }
    // do not allow "w" and "rw" using openat
    if (seccomp_rule_add(ctx, SCMP_ACT_KILL, SCMP_SYS(openat), 1, SCMP_CMP(2, SCMP_CMP_MASKED_EQ, O_WRONLY, O_WRONLY)) != 0) {
        return false;
    }
    if (seccomp_rule_add(ctx, SCMP_ACT_KILL, SCMP_SYS(openat), 1, SCMP_CMP(2, SCMP_CMP_MASKED_EQ, O_RDWR, O_RDWR)) != 0) {
        return false;
    }    

    if (seccomp_load(ctx) != 0) {
        return false;
    }
    seccomp_release(ctx);
    return true;
}

bool resource_init(int max_cpu_time, int max_memory, int max_output_size) {
    
    if(max_cpu_time != -1) {
        rlim_t cpu_time = max_cpu_time / 1000 + 1;
        if(setrlimit(RLIMIT_CPU, new rlimit{cpu_time, cpu_time})) {
            return false;
        } 
    }

    if(max_memory != -1) {
        rlim_t memory = max_memory;
        if(setrlimit(RLIMIT_AS,new rlimit{memory, memory})) {
            return false;
        }

        if(setrlimit(RLIMIT_STACK,new rlimit{memory, memory})) {
            return false;
        }
    }

    if(max_output_size != -1) {
        rlim_t output_size = max_output_size;
        if(setrlimit(RLIMIT_FSIZE, new rlimit{output_size, output_size})) {
            return false;
        }
    }
    return true;
}