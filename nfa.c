#include <stdlib.h>
#include <stdio.h>
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

void free_NFAState(NFAState state) {
    for (int i = 0; i < ALPHABET; i++) {
        free_List(state->trans[i]);
    }
    free(state->trans);
    free(state);
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

// convert() helpers
// these are reset when conversion begins
List dfaStates;
List taggedNfaStates;

void convert(DFAState state) {
    // add all possible epsilon destinations to this tag
    // for every NFAState in tag...
    Node tagged = List_getHead(DFAState_getTag(state));
    while (tagged != NULL) {
        NFAState tagState = List_getObject(tagged);
        // for every epsilon transition, add destinations directly to tag
        Node dst = List_getHead(NFAState_getTransitions(tagState, EPSILON));
        while (dst != NULL) {
            List_addUnique(DFAState_getTag(state), List_getObject(dst));
            List_addUnique(taggedNfaStates, List_getObject(dst)); // keep track
            dst = List_getNext(dst);
        }
        tagged = List_getNext(tagged);
    }

    // mark this state as a success if any tagged states are successes
    tagged = List_getHead(DFAState_getTag(state));
    while (tagged != NULL) {
        // found a success, mark and stop
        if (NFAState_getSuccess(List_getObject(tagged))) {
            DFAState_setSuccess(state, 1);
            break;
        }

        tagged = List_getNext(tagged);
    }

    // if this state's tags match an existing one, stop
    Node existing = List_getHead(dfaStates);
    while (existing != NULL) {
        if (List_equals(
                DFAState_getTag(state),
                DFAState_getTag(List_getObject(existing))
            )) {
            DFAState_manuallyFreeTag(state);
            free_DFAState(state);
            return;
        }
        existing = List_getNext(existing);
    }

    // log as an existing state
    List_add(dfaStates, state);

    // create all transitions
    List *destinations = (List *)malloc(sizeof(List) * ALPHABET);
    // for every letter in ALPHABET (except EPSILON)...
    for (int i = 1; i < ALPHABET; i++) {
        // for every state in tag that transitions on that letter...
        tagged = List_getHead(DFAState_getTag(state));
        while (tagged != NULL) {
            NFAState tagState = List_getObject(tagged);
            // if there are no transitions skip this state
            if (List_empty(NFAState_getTransitions(tagState, i))) {
                tagged = List_getNext(tagged);
                continue;
            }

            // if this transition hasn't been taken yet
            if (destinations[i] == NULL) {
                destinations[i] = new_List();
            }
            // keep track of all destinations
            Node dst = List_getHead(NFAState_getTransitions(tagState, i));
            while (dst != NULL) {
                List_addUnique(destinations[i], List_getObject(dst));
                List_addUnique(taggedNfaStates, List_getObject(dst)); // keep track
                dst = List_getNext(dst);
            }

            tagged = List_getNext(tagged);
        }

        // skip if no transitions
        if (destinations[i] == NULL) {
            continue;
        }

        // TODO: this eps logic is copied from beginning of function, simplify
        // add all possible epsilon destinations to destination list
        // for every NFAState in destination...
        Node dests = List_getHead(destinations[i]);
        while (dests != NULL) {
            NFAState destState = List_getObject(dests);
            // for every epsilon transition, add destinations directly to tag
            Node dst = List_getHead(NFAState_getTransitions(destState, EPSILON));
            while (dst != NULL) {
                List_addUnique(destinations[i], List_getObject(dst));
                List_addUnique(taggedNfaStates, List_getObject(dst)); // keep track
                dst = List_getNext(dst);
            }
            dests = List_getNext(dests);
        }

        // check if this destination list matches an existing DFAState's tag
        // if it does, that DFAState will be set as the destination
        DFAState dst;
        int dstUnique = 1;
        int checked = 0;
        Node existing = List_getHead(dfaStates);
        while (existing != NULL) {
            DFAState existingState = List_getObject(existing);
            checked++;
            // if we found a match
            if (List_equals(destinations[i], DFAState_getTag(existingState))) {
                free_List(destinations[i]);

                dst = existingState;
                dstUnique = 0;
                break;
            }

            existing = List_getNext(existing);
        }
        // no match was found, create a new state and transition to it
        if (dstUnique) {
            dst = new_DFAState();
            DFAState_manuallyFreeTag(dst);
            DFAState_setTag(dst, destinations[i]);

            // continue building...
            convert(dst);
        }
        // transition to destination state
        DFAState_addTransition(state, dst, i);
    }

    free(destinations);
}

void freeAllTags() {
    // free all NFAStates that are tagged
    Node current = List_getHead(taggedNfaStates);
    while (current != NULL) {
        free_NFAState(List_getObject(current));
        current = List_getNext(current);
    }

    // free all tag lists
    current = List_getHead(dfaStates);
    while (current != NULL) {
        free_List(DFAState_getTag(List_getObject(current)));
        current = List_getNext(current);
    }

    free_List(taggedNfaStates);
}

List NFAtoDFA(NFAState entry) {
    dfaStates = new_List();
    taggedNfaStates = new_List();

    // create DFA entry tagged with NFA entry
    DFAState dfaEntry = new_DFAState();
    List_addUnique(DFAState_getTag(dfaEntry), entry);
    List_addUnique(taggedNfaStates, entry); // keep track
    // List_add(dfaStates, dfaEntry);
    convert(dfaEntry); // start the process...

    freeAllTags();

    return dfaStates;
}
