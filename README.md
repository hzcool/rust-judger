## judger 使用说明

***

### 1.安装

***

### 2. ```api``` 接口

+ ping  
  url ```http://127.0.0.1:8080/ping```  
  method ```POST```  
  head ```"content-type", "application/json"```  
  data: ```空```

返回结果: 待定
***

+ judge  
  url ```http://127.0.0.1:8080/judge```  
  method ```POST```  
  head ```"content-type", "application/json"```  
  data: ```json``` 格式
    ```yaml
  lang :  待测评的文件使用的编程语言 #可选项["C99", "C"(默认C11), "C11", "C++"(默认C++11), "C++11", "C++14", "C++17", Python2, Python3, Java]
  src :  测评代码 
  max_cpu_time : 最大 cpu 时间限制 #单位毫秒
  max_memory : 最大使用内存限制 #单位字节
  io_dir: 测试输入输出文件夹
  test_cases : 所有测试样例 数组类型 # 包含若干 test_case
      - #下面描述每个测试用例
          id :  测试用例编号
          input_name : 输入文件名 
          output_name : 输出文件名
          max_output_size : 最大输出文件大 #可空 
  checker : 检查器   # json 对象，可空，默认为 "standard" 类型检测器
      typ : 检查器类型 #可选["standard", "ignore_whitespaces", "identical", "floats", "unordered"]
      mode : 模式  #当 typ 为 "floats" 时起作用， 值可选 0 (绝对误差)或 1 (相对)，可空，默认为 0
      epsilon : 误差范围 #当 typ 为 "floats" 时起作用， 可空，默认为 1e-9
  spj_config : 特判配置 # json 格式, 可空。 非空时 checker 不起作用
      spj_lang : 特判文件语言 #可选项["C++"(默认C++11), "Python(默认Python3)"]
      spj_src : 特判代码  
      seccomp_rule : 代码系统调用权限控制 #可选项["c_cpp", "general", "none"]。 建议为空, 由测评机根据语言决定
   ```
    1. example1
  ```json
  {
    "lang" : "C++",
    "src" : "#include <bits/stdc++.h> \nusing namespace std;\nint main() { \nint a, b; \ncin >> a >> b;\ncout << a + b << endl;\nreturn 0;}"
    "max_cpu_time" : 1000,
    "max_memory" : 2684435456,
    "io_dir" : "",
    "test_cases" : [
        {
          "id" : 1,
          "input_path" : "1.in",
          "output_path" : "1.out"
        }
    ]
  }
  ```
    2. example2 -- special judge
  ```json
  {
    "lang" : "C++",
    "src" : "",
    "max_cpu_time" : 1000,
    "max_memory" : 2684435456,
    "io_dir": "",
    "test_cases" : [
        {
          "id" : 1,
          "input_name" : "",
          "output_name" : ""
        }
    ],
    "spj_config" : {
      "spj_lang" : "C++",
      "spj_src": ""
  }
  }
   ```

  


  
