#include <stdio.h>
#include "regex.h"

int main(int argc, char *argv[]) {
    printf("trex - tiny regular expression engine\n");
    RegEx re = compile("abc");
    // printf("Match 'abc'?\t%d\n", match(re, "abc"));
    // printf("Match 'ab'?\t%d\n", match(re, "ab"));
    // printf("Match 'abcd'?\t%d\n", match(re, "abcd"));
    // printf("Match '0abc'?\t%d\n", match(re, "0abc"));
    // printf("Match 'ganbddc\t%d\n", match(re, "ganbddc"));
}