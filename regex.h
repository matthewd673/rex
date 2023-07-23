#include "dfa.h"

typedef struct RegEx *RegEx;

void free_RegEx(RegEx re);

RegEx compile(char *expr);

int match(RegEx re, char *str);

DFAState RegEx_getEntry(RegEx re);
