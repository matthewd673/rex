#include <stdlib.h>
#include "nfa.h"
#include "regex.h"

struct RegEx {
    NFAState entry;
};

typedef struct NFAModule *NFAModule;
struct NFAModule {
    List exits;
    NFAState entry;
};

RegEx new_RegEx(NFAState entry) {
    RegEx new = (RegEx)malloc(sizeof(struct RegEx));
    if (new == NULL) {
        return NULL;
    }

    new->entry = entry;
    return new;
}

NFAModule new_NFAModule() {
    NFAModule new = (NFAModule)malloc(sizeof(struct NFAModule));

    new->exits = new_List();

    return new;
}

void connectModules(NFAModule previous, NFAModule next) {
    Node current = List_getHead(previous->exits);
    while (current != NULL) {
        NFAState state = (NFAState)List_getObject(current);
        NFAState_addTransition(state, next->entry, EPSILON);
        NFAState_setSuccess(state, 0);  // once connected, exits no longer succeed
        current = List_getNext(current);
    }
}

void subsumeModuleExits(NFAModule this, NFAModule child) {
    Node current = List_getHead(child->exits);
    while (current != NULL) {
        List_add(this->exits, List_getObject(current));
        current = List_getNext(current);
    }
}

NFAModule moduleConcatenation(NFAModule previous, char c) {
    // create new module and connect it to previous
    NFAModule this = new_NFAModule();
    this->entry = new_NFAState();
    connectModules(previous, this);

    // transition from entry to exit on char
    NFAState exit = new_NFAState();
    NFAState_setSuccess(exit, 1); // exit is always assumed to succeed
    NFAState_addTransition(this->entry, exit, c);

    // add exit to module
    List_add(this->exits, exit);

    return this;
}

NFAModule moduleUnion(NFAModule previous, NFAModule left, NFAModule right) {
    // create new module and connect to previous
    NFAModule this = new_NFAModule();
    this->entry = new_NFAState();
    connectModules(previous, this);

    // create epsilon transitions from entry to branches' entries
    NFAState_addTransition(this->entry, left->entry, EPSILON);
    NFAState_addTransition(this->entry, right->entry, EPSILON);

    // make branches' exits our own
    subsumeModuleExits(this, left);
    subsumeModuleExits(this, right);

    return this;
}

// NFAState moduleKClosure(NFAModule closure) {
//     //
// }

RegEx compile(char *expression) {
    int i = 0;

    NFAState entry = new_NFAState();
    NFAState lastState = entry;

    // TODO: temp, bare minimum
    while (expression[i] != EPSILON) {
        // create new state and transition to it
        NFAState newState = new_NFAState();
        NFAState_addTransition(lastState, newState, expression[i]);
        lastState = newState;
        i++;
    }

    RegEx this = new_RegEx(entry);

    return this;
}

int match(RegEx re, char *str) {
    // int i = 0;
    // DFAState current = re->entry;
    // while (str[i] != 0) {
    //     DFAState next = DFAState_getTransition(current, str[i]);
    //     if (next != NULL) {
    //         current = next;
    //     }
    //     i++;
    // }
    // return DFAState_getSuccess(current);
    return 1;
}

NFAState RegEx_getEntry(RegEx re) {
    return re->entry;
}