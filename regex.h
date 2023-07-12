#include "nfa.h"

typedef struct RegEx *RegEx;

RegEx compile(char *expression);

int match(RegEx re, char *str);

NFAState RegEx_getEntry(RegEx re);