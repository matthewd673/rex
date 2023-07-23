#include <stdlib.h>
#include <stdio.h>
#include "stringifier.h"
#include "list.h"

/*
    === INTERFACE DEFINITION ===
    {
        "states": [                                 // stringify_NFA
            {                                       // print_NFAState
                "id": {id},
                "success": {success},
                "transitions": [
                    {                               // print_NFAStateTransitions
                        "character": {character},
                        "states": [
                            {id},
                            ...
                        ]
                    },
                    ...
                ]
            },
            ...
        ]
    }
*/

void print_NFAStateTransitions(NFAState state) {
    int printedTransitions = 0;
    for (int i = 0; i < ALPHABET; i++) {
        List transitions = NFAState_getTransitions(state, i);
        // no transitions on this character, skip
        if (List_empty(transitions)) {
            continue;
        }

        // print leading comma if not first in list
        if (printedTransitions > 0) {
            printf(",");
        }

        // begin printing transition
        printf("{\"character\":%d,\"states\":[", i);

        // print transition states
        int printedStates = 0;
        Node current = List_getHead(transitions);
        while (current != NULL) {
            // print leading comma if not first in list
            if (printedStates > 0) {
                printf(",");
            }
            printf("\"%p\"", List_getObject(current));
            current = List_getNext(current);
            printedStates++;
        }
        // close it off
        printf("]}");
        printedTransitions++;
    }
}

void print_NFAState(NFAState state, List visited) {
    // don't print states that have already been traversed
    if (List_contains(visited, state)) {
        return;
    }

    // if another state has been visted (therefore printed), print leading comma
    if (!List_empty(visited)) {
        printf(",");
    }

    // print this state
    printf("{\"id\":\"%p\",\"success\":%s,\"transitions\":[",
        state,
        NFAState_getSuccess(state) ? "true" : "false"
        );
    print_NFAStateTransitions(state);
    printf("]}");

    // mark this state as visited
    List_add(visited, state);

    // recursively print all subsequent states
    for (int i = 0; i < ALPHABET; i++) {
        Node current = List_getHead(NFAState_getTransitions(state, i));
        while (current != NULL) {
            print_NFAState(List_getObject(current), visited);
            current = List_getNext(current);
        }
    }
}

void stringify_NFA(NFAState entry) {
    // begin printing
    printf("{\"states\":[");

    // start printing at entry point
    print_NFAState(entry, new_List());

    // close it off
    printf("]}\n");
}

void print_DFAStateTransitions(DFAState state) {
    int printedTransitions = 0;
    for (int i = 0; i < ALPHABET; i++) {
        // if there is no transition on this character, skip
        if (!DFAState_hasTransition(state, i)) {
            continue;
        }

        DFAState transition = DFAState_getTransition(state, i);

        // print leading comma if not first in list
        if (printedTransitions > 0) {
            printf(",");
        }

        // begin printing transition
        printf("{\"character\":%d,\"state\":", i);

        // print transition state
        printf("\"%p\"", transition);

        // close it off
        printf("}");
        printedTransitions++;
    }
}

void print_DFAStateTag(DFAState state) {
    Node current = List_getHead(DFAState_getTag(state));
    int printedTag = 0;
    while (current != NULL) {
        if (printedTag) {
            printf(",");
        }
        printf("\"%p\"", List_getObject(current));
        printedTag = 1;
        current = List_getNext(current);
    }
}

void print_DFAState(DFAState state, List visited) {
    // don't print states that have already been traversed
    if (List_contains(visited, state)) {
        return;
    }

    // if another state has been visted (therefore printed), print leading comma
    if (!List_empty(visited)) {
        printf(",");
    }

    // print this state
    printf("{\"id\":\"%p\",\"success\":%s,",
        state,
        DFAState_getSuccess(state) ? "true" : "false");

    printf("\"tag\":[");
    print_DFAStateTag(state);
    printf("],");

    printf("\"transitions\":[");
    print_DFAStateTransitions(state);
    printf("]}");

    // mark this state as visited
    List_add(visited, state);

    // recursively print all subsequent states
    for (int i = 0; i < ALPHABET; i++) {
        if (!DFAState_hasTransition(state, i)) {
            continue;
        }
        print_DFAState(DFAState_getTransition(state, i), visited);
    }
}

void stringify_DFA(DFAState entry) {
    // begin printing
    printf("{\"states\":[");

    // start printing at entry point
    print_DFAState(entry, new_List());

    // close it off
    printf("]}\n");
}