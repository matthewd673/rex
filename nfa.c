#include <stdlib.h>

#include "nfa.h"

struct NFAState {
    List *trans;
    char success;
};

NFAState new_NFAState() {
    NFAState new = (NFAState)malloc(sizeof(struct NFAState));
    if (new == NULL) {
        return NULL;
    }

    new->trans = (List *)malloc(sizeof(List) * ALPHABET);
    if (new->trans == NULL) {
        return NULL;
    }
    for (int i = 0; i < ALPHABET; i++) {
        new->trans[i] = new_List();
    }

    new->success = 0; // not success by default

    return new;
}

void NFAState_addTransition(NFAState src, NFAState dst, char c) {
    List_add(src->trans[c], dst);
}

List NFAState_getTransitions(NFAState state, char c) {
    return state->trans[c];
}

void NFAState_setSuccess(NFAState state, char success) {
    state->success = success;
}

char NFAState_getSuccess(NFAState state) {
    return state->success;
}