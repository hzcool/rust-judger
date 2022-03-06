## judger 使用说明

***

### 1.快速搭建

+ 先下载docker和docker-compose， 版本不要太旧即可，可自行尝试安装
+ git clone 该项目到你的某个文件夹内(实际上只要有```docker-compose.yml``` 单个文件即可实现安装)
+ 进入到 ```docker-compose.yml``` 所在目录 ，输入启动命令```docker-compose up -d```即可成功运行。
+ 测试运行，输入 ```curl -H "Content-Type:application/json" -H "ACCESS_TOKEN:123456" -X POST http://0.0.0.0:7777/ping```,
  返回语言的配置信息说明启动成功。

***

### 2. ```api``` 接口

+ ping  
  url ```http://127.0.0.1:8080/ping```  
  method ```POST```  
  head ```[{"content-type", "application/json"}, {"ACCESS_TOKEN", "123456"}]```  
  data: ```空```

返回结果:

```yaml
1. 编译选项: [ C , C++(默认C++11) , C++14 , C++17, Python2 , Python3 , Java ]

2. 编译器版本:
  C: gcc11.1.0,

  C++: g++11.1.0,

  Python2: Python2.7.18

  Python3: Python3.8.10

  Java: Java16.0.2
```

***

+ judge  
  url ```http://127.0.0.1:8080/judge```  
  method ```POST```  
  head ```[{"content-type", "application/json"}, {"ACCESS_TOKEN", "123456"}]```  
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
      spj_lang : 特判文件语言 #可选项["C"(默认C11), "C++"(默认C++17), "Python(默认Python3)"]
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

***

### 3. checker 说明

1. type 可选项为 ["standard", "ignore_whitespaces",  "identical", "floats", "unordered"]，默认为 standard。

```yaml
standard: 忽略空行和末尾空白字符
ignore_whitespaces: 忽略所有的空白字符
identical: 忽略行末的空白字符
floats: 浮点比较， mode = 0(默认)为绝对误差， mode = 1为相对误差。  误差上限为 episilon(默认为1e-9)。 对于比较文件的数字部分会采用对应的误差，非数字部分会进行一致性比较。
unordered: 将测试代码输出文件的所有单词升序排列后然后与标准文件(同样升序)进行一致性比较。
```

2. 对于 special-judge, 该项不起作用。

***

### 4. spj_config 说明

1. json 中该项不为空时采用特判。 特判文件语言可选项["C"(默认C11), "C++"(默认C++17), "Python(默认Python3)"]