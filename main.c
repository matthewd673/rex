#include <stdio.h>
#include "regex.h"
#include "stringifier.h"

int main(int argc, char *argv[]) {
    printf("rex - tiny regular expression engine\n");
    RegEx re = compile("(ab|a)*");

    stringify_DFA(RegEx_getEntry(re));

    // printf("Match 'abc'?\t%d\n", match(re, "abc"));
    // printf("Match 'ab'?\t%d\n", match(re, "ab"));
    // printf("Match 'abcd'?\t%d\n", match(re, "abcd"));
    // printf("Match '0abc'?\t%d\n", match(re, "0abc"));
    // printf("Match 'ganbddc\t%d\n", match(re, "ganbddc"));
}