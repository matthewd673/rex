#include <stdio.h>
#include "regex.h"
#include "stringifier.h"

int main(int argc, char *argv[]) {
    printf("rex - tiny regular expression engine\n");
    RegEx re = compile("(ab|a)*");

    stringify_DFA(RegEx_getEntry(re));

    printf("Match 'aba'?\t\t%d\n", match(re, "aba"));
    printf("Match 'no'?\t\t%d\n", match(re, "no"));
    printf("Match 'bababa'?\t\t%d\n", match(re, "bababa"));
    printf("Match 'abababa'?\t%d\n", match(re, "abababa"));
    printf("Match ''?\t\t%d\n", match(re, ""));
    printf("Match 'axxbxaba'?\t%d\n", match(re, "axxbxaba"));
    printf("Match 'aaaab'?\t\t%d\n", match(re, "aaaab"));

    free_RegEx(re);
}
