#include <stdlib.h>
#include <stdio.h>
#include "dfa.h"

#define TRANS_CT    256

struct DFAState {
    DFAState *trans;
    char success;
};

DFAState new_DFAState() {
    DFAState new = (DFAState)malloc(sizeof(struct DFAState));
    if (new == NULL) {
        return NULL;
    }

    // TODO: should be mapping of lists
    new->trans = (DFAState *)malloc(sizeof(DFAState) * TRANS_CT);

    return new;
}

void DFAState_addTransition(DFAState src, DFAState dst, char c) {
    src->trans[c] = dst;
}

DFAState DFAState_getTransition(DFAState state, char c) {
    return state->trans[c];
}

void DFAState_setSuccess(DFAState state, char success) {
    state->success = success;
}

char DFAState_getSuccess(DFAState state) {
    return state->success;
}

void printTransitions(DFAState state) {
    printf("[");
    int printedTrans = 0; // how many transitions were actually printed?
    for (int i = 0; i < TRANS_CT; i++) {
        // don't print null transitions
        if (state->trans[i] == NULL) {
            continue;
        }

        if (printedTrans > 0) {
            printf(",");
        }
        printf("{\"character\":%d,\"states\":[\"%p\"]}", i, state->trans[i]);
        printedTrans++;
    }
    printf("]");
}

void DFAState_print(DFAState state, int depth) {
    printf("{\"states\":[");
    // print this and its transitions
    printf("{");
    printf("\"id\":\"%p\",\"transitions\":", state);
    printTransitions(state);
    printf("}");
    // TODO: print other states
    printf("]}\n");
}