### 编译: ```g++ -O3 -lm main.cpp rule.cpp runner.cpp args_parser/parser.cpp -o runner -lseccomp -lpthread```

### 命令行参数
```C++
p.add("--run_command", "-c", Parser::ArgType::STRING, 0, "运行指令， 非空");
p.add("--max_cpu_time", "-t", Parser::ArgType::INT, 0, "运行时间上限, 单位毫秒， 默认 20 秒 ");
p.add("--max_memory", "-m", Parser::ArgType::INT, 0, "运行内存上限，单位字节, 默认 1024 MB");
p.add("--max_output_size", "-o", Parser::ArgType::INT, 0, "输出文件大小上限，单位字节， 默认 64MB");
p.add("--input_path", "-I", Parser::ArgType::STRING, 0, "标准输入文件路径, 可空");
p.add("--output_path", "-O", Parser::ArgType::STRING, 0, "输出文件路径, 非空");
p.add("--seccomp_rule", "-r", Parser::ArgType::STRING, 0, "选项[c_cpp, general, none], 默认c_cpp");
p.add("--resource_rule", "-s", Parser::ArgType::INT, 0, "位状态 `b[0]b[1]b[2]`，b[0]是否进行时间限制，b[1]是否进行内存限制，b[2]是否进行输出文件大小限制，默认 `7`");
```

#### cmd example: ```./runner -c "./test/a.out" -t 1000 -m 134217728 -o 1048576 -I ./test/test.in -O ./test/test.out -r c_cpp```
