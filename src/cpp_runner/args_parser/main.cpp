//
//  main.cpp
//  CmdParser
//
//  Created by liupeng on 2020/11/2.
//

#include <iostream>
#include "parser.hpp"

void printData(Parser::Arguments& args){
    switch (args.type) {
        case Parser::ArgType::INT:
            printf("integers:");
            for(int i = 0;i< args.data.integers.size();i++){
                printf("%d ", args.data.integers[i]);
            }
            printf("\n");
            break;
        case Parser::ArgType::STRING:
            printf("strings: \n");
            for(int i = 0;i<args.data.strings.size();i++){
                printf("%s\n", args.data.strings[i].c_str());
            }
            break;
        case Parser::ArgType::FLOAT:
            printf("floats: ");
            for(int i = 0;i< args.data.floats.size();i++){
                printf("%f ", args.data.floats[i]);
            }
            printf("\n");
            break;
            
        default:
            break;
    }
}

int main(int argc, const char * argv[]) {
    // insert code here...

    Parser p;
    p.add("--integer", "-i", Parser::ArgType::INT, 0, "integer");
    p.add("--floats", "-f", Parser::ArgType::FLOAT, 0, "floats");
    p.add("--strings", "-s", Parser::STRING, 0, "sss");
    p.add("--type", "-t", Parser::STRING, 0, "type");
    
    p.parse(argc, argv);
    Parser::Arguments args;
    
    printf("parsed parameters: \n");
    p.getArguments("--type", args);
    switch (args.data.strings[0][0]){
        case 'I':
            args = p["integer"];
            printData(args);
            break;
        case 'S':
            args = p["strings"];
            printData(args);
            break;
        case 'F':
            args = p["floats"];
            printData(args);
            break;
    }
    
    return 0;
}
