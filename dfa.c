#include <stdlib.h>
#include "dfa.h"

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
    new->trans = (DFAState *)malloc(sizeof(DFAState) * 256);

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