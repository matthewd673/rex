#include <stdlib.h>
#include "nfa.h"
#include "regex.h"

struct RegEx {
    NFAState entry;
};

typedef struct NFAModule *NFAModule;
struct NFAModule {
    List exits;
    NFAState head;
    NFAState tail;
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

int toki = -1;
char tok = -1;
int escape = 0;
int eat(char *expr) {
    if (tok != 0) {
        toki++;
        tok = expr[toki];
    }
    return tok != 0;
}

NFAState parse(char *expr) {
    // create dummy NFA state to start
    NFAState head = new_NFAState();
    NFAState p = head;

    while (eat(expr)) {
        // escape
        if (!escape && tok == '\\') {
            escape = 1;
        }
        // union
        else if (!escape && tok == '|') {
            p = head; // reset pointer back to head
        }
        // kleene closure
        else if (!escape && tok == '*') {
            // TODO
        }
        // TODO: parentheses
        // concatenation
        else {
            escape = 0; // stop escaping
            // add new state after pointer and move pointer to it
            NFAState new = new_NFAState();
            NFAState_addTransition(p, new, tok);
            NFAState_setSuccess(p, 0);
            p = new;
            NFAState_setSuccess(p, 1);
        }
    }

    return head;
}

RegEx compile(char *expr) {
    RegEx this = new_RegEx(parse(expr));
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