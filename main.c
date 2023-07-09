#include <stdio.h>
#include "regex.h"

int main(int argc, char *argv[]) {
    printf("trex - tiny regular expression engine\n");
    RegEx re = compile("abc");
    printf("Match 'abc'?\t%d\n", match(re, "abc"));
    printf("Match 'ab'?\t%d\n", match(re, "ab"));
}