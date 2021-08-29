//
//  parser.hpp
//  CmdParser
//
//  Created by liupeng on 2020/11/2.
//

#ifndef parser_hpp
#define parser_hpp

#include <stdio.h>
#include <vector>
#include <map>
#include <string>
class Parser{
public:
    enum ArgType{
        INT = 0,
        FLOAT,
        STRING,
    };
    struct Data{
        std::vector<int> integers;
        std::vector<float> floats;
        std::vector<std::string> strings;
    };
    struct Arguments{
        ArgType type;
        Data data;
    };
    int add(const char* name, const char* shortName, ArgType type, int max_count, const char* help);
    int remove(const char* name);
    int parse(int argc, const char* argv[]);
    
    void getArguments(const char* name, Arguments& args);
    void* getArguments(const char* name, int& count, ArgType& type);
    Arguments operator [] (const char* name);
private:
    
    typedef struct{
        ArgType type;
        std::string short_name;
        int index;
        int max_count;
        std::string help;
        int vecIndex;
        int real_count;
    } arguments_head;
    std::vector<int> integers;
    std::vector<float> floats;
    std::vector<std::string> strings;
    std::map<std::string, arguments_head> arguments_heads;
    std::map<std::string, std::string> short_name_names;
    
};

#endif /* parser_hpp */
