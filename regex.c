#include <stdlib.h>
#include "dfa.h"
#include "regex.h"

struct RegEx {
    DFAState entry;
};

RegEx new_RegEx(DFAState entry) {
    RegEx new = (RegEx)malloc(sizeof(struct RegEx));
    if (new == NULL) {
        return NULL;
    }

    new->entry = entry;
    return new;
}

RegEx compile(char *expression) {
    int i = 0;

    DFAState entry = new_DFAState();
    DFAState lastState = entry;

    // TODO: temp
    while (expression[i] != 0) {
        // create new state and transition to it
        DFAState newState = new_DFAState();
        DFAState_addTransition(lastState, newState, expression[i]);
        lastState = newState;
        i++;
    }
    // mark last node as success
    DFAState_setSuccess(lastState, 1);

    // for debugging:
    DFAState_print(entry, 0);

    return new_RegEx(entry);
}

int match(RegEx re, char *str) {
    int i = 0;
    DFAState current = re->entry;
    while (str[i] != 0) {
        DFAState next = DFAState_getTransition(current, str[i]);
        if (next != NULL) {
            current = next;
        }
        i++;
    }
    return DFAState_getSuccess(current);
}