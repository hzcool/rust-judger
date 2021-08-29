//
//  parser.cpp
//  CmdParser
//
//  Created by liupeng on 2020/11/2.
//
#include <cassert>
#include <cstring>
#include "parser.hpp"
static bool isTotalName(const char* name){
    return name != NULL && strlen(name) >= 3 && name[0] == name[1] && name[1] == '-';
}
static bool isShortName(const char* name){
    return name != NULL and strlen(name) == 2 && name[0] =='-' && (('a' <= name[1] && 'z' >= name[1] ) ||('A' <= name[1] && 'Z' >= name[1] ));
}
int Parser::add(const char *name, const char *shortName, ArgType type, int max_count, const char *help){
    
    assert(name != NULL && (strlen(name) >= 3 && name[0] == name[1] && name[0] == '-'));
    assert(shortName == NULL || strlen(shortName) == 0 || (strlen(shortName) == 2 && shortName[0] == '-' && shortName[1] != '-'));
    arguments_head head;
    head.type = type;
    head.index = -1;
    head.max_count = max_count;
    head.help = help;
    head.real_count = 0;
    if (isShortName(shortName)){
        head.short_name = shortName;
        this->short_name_names[shortName] = name;
    }else{
        head.short_name = "";
    }
    this->arguments_heads[name] = head;
    return 0;
}
int Parser::remove(const char *name){
//    assert(name != NULL);
//    assert(strlen(name) >= 2);
//    assert((name[0] == name[1] && name[0] == '-') || (strlen(name) == 2 && name[0] == '-'));
    if (name == NULL){
        return -1;
    }
    std::string total_name;
    if (isShortName(name)){
        if (short_name_names.find(name) != short_name_names.end()){
            total_name = short_name_names[name];
        }else{
            return -2;
        }
    }else {
        total_name = name;
    }
    if (arguments_heads.find(total_name) != arguments_heads.end()){
        arguments_heads.erase(total_name);
        return 0;
    }
    return -3;
    
}

static void parse_argumenst(char* parameter, Parser::ArgType type, void* data){
    switch (type) {
        case Parser::ArgType::INT:
        {
//            int* d = (int*) data;
            sscanf(parameter, "%d", (int*)data);
            break;
        }
        case Parser::ArgType::STRING:
        {
//            char* d = (char*)data;
            sprintf((char*)data, "%s", parameter);
            break;
            
        }
        case Parser::ArgType::FLOAT:
        {
            sscanf(parameter, "%f", (float*) data);
            break;
        }
        default:
        {
//            char* d = (char*)data;
            sprintf((char*)data, "%s", parameter);
            break;
        }
    }
}

int Parser::parse(int argc, const char *argv[]){
    int i = 1;
    for(;i<argc;i++){
        if (isTotalName(argv[i])){
            auto it = arguments_heads.find(argv[i]);
            assert(it != arguments_heads.end());
            it->second.index = i+1;
        }else if (isShortName(argv[i])){
            
            auto it2 = short_name_names.find(argv[i]);
            assert(it2 != short_name_names.end());
            auto it = arguments_heads.find(it2->second);
            it->second.index = i+1;
        }
    }
    
    for (auto it = arguments_heads.begin(); it != arguments_heads.end(); it++){
        if (it->second.index == -1){
            continue;
        }

        int index = it->second.index;
        char* parameter = (char*)argv[index];
        bool start = true;
        ArgType type = it->second.type;
        it->second.real_count = 0;
        for (; index < argc && parameter[0] != '-'; index++){
            
            if (type == ArgType::INT){
                integers.push_back(0);
                parse_argumenst(parameter, type, &integers.back());
                start && (it->second.vecIndex = (int)integers.size()-1);
            }else if (type == ArgType::FLOAT){
                floats.push_back(0.0);
                parse_argumenst(parameter, type, &floats.back());
                start && (it->second.vecIndex = (int) floats.size()-1);
            }else if (type == ArgType::STRING){
                char data[256];
                parse_argumenst(parameter, type, data);
                strings.push_back(data);
                start && (it->second.vecIndex = (int) strings.size()-1);
            }
            it->second.real_count ++;
            if (start){
                start = false;
            }
            parameter = (char*)argv[index+1];
        }
    }
    return 0;
}
void *Parser::getArguments(const char *name, int &count, ArgType& type){
    std::string total_name = name;
    if (isShortName(name)){
        total_name = short_name_names[name];
    }
    if (arguments_heads.find(total_name) == arguments_heads.end()){
        return NULL;
    }
    
    auto head = arguments_heads[name];
    count = head.real_count;
    type = head.type;
    switch (head.type) {
        case Parser::ArgType::INT:
            return (void*)&integers[head.vecIndex];
        case Parser::ArgType::FLOAT:
            return (void*) &floats[head.vecIndex];
        case Parser::ArgType::STRING:
            return (void*) &strings[head.vecIndex];
        default:
            return NULL;
    }
}

 void Parser::getArguments(const char *name, Parser::Arguments& args){
    int count;
    ArgType type;
    void *data = this->getArguments(name, count, type);
    args.type = type;
    switch (type) {
        case INT :
            args.data.integers.resize(count);
            for(int i = 0;i< count;i++){
                args.data.integers[i] = *((int*)data + i);
            }
//            for(int i = 0;i< count)
            break;
        case STRING:
        default:
            args.data.strings.resize(count);
            for(int i = 0;i< count; i++){
                args.data.strings[i] = *((std::string*) data + i);
            }
            break;
        case FLOAT:
            args.data.floats.resize(count);
            for(int i = 0;i< count;i++){
                args.data.floats[i] = *((float*) data + i);
            }
            break;
        
    }
}

Parser::Arguments Parser::operator[](const char *name){
    Arguments args;
    std::string total_name = name;
    if (name[0] != name[1] && name[0] != '-'){
        total_name = "--" + total_name;
    }
    this->getArguments(total_name.c_str(), args);
    return args;
}
