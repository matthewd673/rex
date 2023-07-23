#include <stdlib.h>
#include <stdio.h>
#include "dfa.h"
#include "nfa.h"

struct DFAState {
    DFAState *trans;
    char *hasTrans;
    List tag;
    char success;
};

DFAState new_DFAState() {
    DFAState new = (DFAState)malloc(sizeof(struct DFAState));
    if (new == NULL) {
        return NULL;
    }

    new->trans = (DFAState *)malloc(sizeof(DFAState) * ALPHABET);
    new->hasTrans = (char *)malloc(sizeof(char) * ALPHABET);
    for (int i = 0; i < ALPHABET; i++) {
        new->hasTrans[i] = 0;
    }
    new->tag = new_List();
    new->success = 0; // not success by default

    return new;
}

void free_DFAState(DFAState state) {
    free(state->trans);
    free(state->hasTrans);
    free(state);
}

void DFAState_addTransition(DFAState src, DFAState dst, char c) {
    src->trans[c] = dst;
    src->hasTrans[c] = 1;
}

DFAState DFAState_getTransition(DFAState state, char c) {
    return state->trans[c];
}

int DFAState_hasTransition(DFAState state, char c) {
    return state->hasTrans[c];
}

void DFAState_setTag(DFAState state, List tag) {
    state->tag = tag;
}

List DFAState_getTag(DFAState state) {
    return state->tag;
}

void DFAState_setSuccess(DFAState state, char success) {
    state->success = success;
}

char DFAState_getSuccess(DFAState state) {
    return state->success;
}