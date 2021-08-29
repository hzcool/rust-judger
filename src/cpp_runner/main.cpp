#include "runner.h"

// g++ -O3 -lm main.cpp rule.cpp runner.cpp -o runner -lseccomp -lpthread
// ./runner "C++" "./test/a.out" 1000 134217728 1048576 ./test/a.out ./test/test.in ./test/test.out
int main(int argc, const char* argv[]) 
{
    Runner runner = Runner(argc, argv);
    runner.run();
    return 0;
}