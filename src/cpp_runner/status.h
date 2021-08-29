#ifndef STATUS_H
#define STATUS_H


//用户代码运行结果的状态
enum Status {
    OK = 0,
    TIME_LIMIT_EXCEEDED = 2,
    MEMORY_LIMIT_EXCEEDED = 3,
    OUTPUT_LIMIT_EXCEEDED = 4,
    RUNTIME_ERROR = 5,
    SYSTEM_ERROR = 6,

    //下面都属于 system error
    ARGS_ERROR = 20,

    OPEN_INPUT_FILE_FAIL = 30,
    OPEN_OUTPUT_FILE_FAIL = 31,
    OPEN_ERROR_FILE_FAIL = 32,

    RESOURCE_INIT_ERROR = 40,
    SECCOMP_INIT_ERROR = 41,
    RUN_COMMAND_FAIL = 42
};

std::string get_info(Status status) {
    std::string info = "";
    switch (status)
    {
    case OK:
        info = "Ok";
        break;
    case TIME_LIMIT_EXCEEDED:
        info = "时间超限";
        break;
    case MEMORY_LIMIT_EXCEEDED:
        info = "内存超限";
        break;
    case OUTPUT_LIMIT_EXCEEDED:
        info = "输出超限";
        break;
    case RUNTIME_ERROR:
        info = "运行出错";
        break;
    case SYSTEM_ERROR:
        info = "系统错误";
        break;
    case ARGS_ERROR:
        info = "参数错误";
        break;
    case OPEN_INPUT_FILE_FAIL:
        info = "无法打开输入文件";
        break;
    case OPEN_OUTPUT_FILE_FAIL:
        info = "无法打开输出文件";
        break;
    case OPEN_ERROR_FILE_FAIL:
        info = "无法错误记录文件";
        break;
    case RESOURCE_INIT_ERROR:
        info = "资源限制错误";
        break;
    case SECCOMP_INIT_ERROR:
        info = "权限限制错误";
        break;
    case RUN_COMMAND_FAIL:
        info = "运行代码失败";
        break;
    default:
        break;
    }
    return info;
}
#endif
