#include <bits/stdc++.h>
#include <unistd.h>

using namespace std;
int main() {
    
    char *args[] = {"a.out", NULL};
    char *envp[] = {NULL};
    execve("a.out", args, envp);
    return 0;
}