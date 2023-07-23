#include "dfa.h"

typedef struct RegEx *RegEx;

RegEx compile(char *expr);

int match(RegEx re, char *str);

DFAState RegEx_getEntry(RegEx re);
