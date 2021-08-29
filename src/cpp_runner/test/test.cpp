#include <bits/stdc++.h>

using namespace std;
int main() {
    char s[]="this      is th#e best way to spend   time for reedaf   ";
    char* ans[100];
    int k = 0;
    char* output = strtok(s, " \t\n");
    while(output != NULL) {
        ans[k++] = output;
        output = strtok(NULL, " \t\n");
    } 
    for(int i = 0; i < k; i++)
        cout << ans[i] << " " << strlen(ans[i]) <<"\n";
    return 0;
}