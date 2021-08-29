#ifndef RESULT_H
#define RESULT_H

#include <string>
#include "status.h"

std::string json_result(Status status, int cpu_time = 0, int memory = 0) {
    return "{\"status\":" + std::to_string(status) + " , " + 
        "\"info\":" + "\"" + get_info(status) + "\" , " +
        "\"cpu_time\":" + std::to_string(cpu_time) + " , " +
        "\"memory\":" + std::to_string(memory) + "}" ; 
}
#endif