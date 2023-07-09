#include "dfa.h"

typedef struct RegEx *RegEx;

RegEx compile(char *expression);

int match(RegEx re, char *str);