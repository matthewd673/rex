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
    printf("\nBegin convert on %p\n", state);
    printf("%d DFAStates exist\n", List_getCount(dfaStates));
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
    printf("%d tags after adding epsilon destinations\n",
        List_getCount(DFAState_getTag(state))
    );

    // mark this state as a success if any tagged states are successes
    tagged = List_getHead(DFAState_getTag(state));
    while (tagged != NULL) {
        // found a success, mark and stop
        if (NFAState_getSuccess(List_getObject(tagged))) {
            DFAState_setSuccess(state, 1);
            printf("Tags contain success state so this DFA state is a success\n");
            break;
        }

        tagged = List_getNext(tagged);
    }

    // TODO: block is purely debug
    tagged = List_getHead(DFAState_getTag(state));
    while (tagged != NULL) {
        printf("\t%p is tagged\n", List_getObject(tagged));
        tagged = List_getNext(tagged);
    }

    // if this state's tags match an existing one, stop
    Node existing = List_getHead(dfaStates);
    while (existing != NULL) {
        if (List_equals(
                DFAState_getTag(state),
                DFAState_getTag(List_getObject(existing))
            )) {
            printf("This state's tag matches an existing one, stopping\n");
            return;
        }
        existing = List_getNext(existing);
    }

    printf("This state's tag is unique, it will be added to the master list\n");

    // log as an existing state
    List_add(dfaStates, state);

    printf("Creating transitions\n");
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

            printf("Found transitions from %p on '%d'\n", tagState, i);

            // if this transition hasn't been taken yet
            if (destinations[i] == NULL) {
                destinations[i] = new_List();
            }
            // keep track of all destinations
            Node dst = List_getHead(NFAState_getTransitions(tagState, i));
            while (dst != NULL) {
                List_addUnique(destinations[i], List_getObject(dst));
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
                dst = List_getNext(dst);
            }
            dests = List_getNext(dests);
        }
        printf("%d destinations after adding epsilon destinations\n",
            List_getCount(destinations[i])
        );

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
                printf("Destinations match an existing state's tag\n");
                dst = existingState;
                dstUnique = 0;
                break;
            }

            existing = List_getNext(existing);
        }
        // no match was found, create a new state and transition to it
        if (dstUnique) {
            printf("Destinations are unique (%d states checked), a new state will be created\n", checked);
            dst = new_DFAState();
            DFAState_setTag(dst, destinations[i]);
        }
        // transition to destination state
        printf("Creating transition to destination tag\n");
        DFAState_addTransition(state, dst, i);

        // continue building...
        if (dstUnique) {
            convert(dst);
        }
    }
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
    free_List(taggedNfaStates);

    return dfaStates;
}
