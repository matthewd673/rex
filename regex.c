#include <stdlib.h>
#include "nfa.h"
#include "dfa.h"
#include "regex.h"
#include "stringifier.h"

struct RegEx {
    DFAState entry;
    List dfaStates;
};

typedef struct NFAModule *NFAModule;
struct NFAModule {
    NFAState head;
    NFAState tail;
    char code;
};

RegEx new_RegEx(DFAState entry, List dfaStates) {
    RegEx new = (RegEx)malloc(sizeof(struct RegEx));
    if (new == NULL) {
        return NULL;
    }

    new->entry = entry;
    new->dfaStates = dfaStates;
    return new;
}

void free_RegEx(RegEx re) {
    Node current = List_getHead(re->dfaStates);
    while (current != NULL) {
        free_DFAState(List_getObject(current));
        current = List_getNext(current);
    }
    free_List(re->dfaStates);
    free(re);
}

NFAModule new_NFAModule() {
    NFAModule new = (NFAModule)malloc(sizeof(struct NFAModule));

    new->head = new_NFAState();
    new->tail = new_NFAState();
    new->code = 0;

    return new;
}

void free_NFAModule(NFAModule module) {
    free(module);
}

// parse() helpers
// these are reset when compilation begins
int toki;
char tok;
int escape;
int eat(char *expr) {
    if (tok != 0) {
        toki++;
        tok = expr[toki];
    }
    return tok != 0;
}

/*
 * parse(char *expr, int depth)
 *   char *expr: the expression being parsed
 *   int depth:  the current depth of the parse() calls
 */
NFAModule parse(char *expr, int depth) {
    // create dummy NFA state to start
    NFAModule module = new_NFAModule();
    NFAState p = module->head;
    List branches = new_List();

    while (eat(expr)) {
        // escape
        if (!escape && tok == '\\') {
            escape = 1;
        }
        // union
        else if (!escape && tok == '|') {
            List_add(branches, p); // add pointer state to branches
            p = module->head; // reset pointer back to head
        }
        // kleene closure
        else if (!escape && tok == '*') {
            // make current pointer a branch
            // however, the head should only be a branch if there are no others
            if (p != module->head || List_empty(branches)) {
                List_add(branches, p);
            }
            // create an eps transition from every branch to the head
            Node current = List_getHead(branches);
            while (current != NULL) {
                NFAState_addTransition(
                    List_getObject(current),
                    module->head,
                    EPSILON
                );
                current = List_getNext(current);
            }
            // mark head as success
            NFAState_setSuccess(module->head, 1);
        }
        // open group
        else if (!escape && tok == '(') {
            NFAModule innerModule = parse(expr, depth + 1);
            // if inner module failed, we fail
            if (innerModule->code) {
                module->code = innerModule->code;
                free_NFAModule(innerModule);
                return module;
            }
            // eps transition to inner module's head from current branch
            // then set current branch to inner module's tail
            NFAState_addTransition(p, innerModule->head, EPSILON);
            p = innerModule->tail;
            free_NFAModule(innerModule);
        }
        // close group
        else if (!escape && tok == ')') {
            // you can't close the top level group, return with an error
            if (depth < 1) {;
                module->code = 1;
                return module;
            }
            // break out of loop and return to caller using logic below
            break;
        }
        // concatenation
        else {
            escape = 0; // stop escaping
            // add new state after pointer and move pointer to it
            NFAState new = new_NFAState();
            NFAState_addTransition(p, new, tok);
            p = new;
        }
    }

    // no more input, close top-level "group" (same logic as for ')' above)
    // make current pointer a branch
    // however, the head should only be a branch if there are no others
    if (p != module->head || List_empty(branches)) {
        List_add(branches, p);
    }
    // connect all branches to tail
    Node current = List_getHead(branches);
    while (current != NULL) {
        NFAState_addTransition(
            List_getObject(current),
            module->tail,
            EPSILON
        );
        current = List_getNext(current);
    }
    // mark tail as success if at top level
    if (depth == 0) {
        NFAState_setSuccess(module->tail, 1);
    }

    // cleanup branches
    free_List(branches);

    return module;
}

RegEx compile(char *expr) {
    toki = -1;
    tok = -1;
    escape = 0;

    // try parse string into NFA
    NFAModule module = parse(expr, 0);
    if (module->code) {
        free_NFAModule(module);
        return NULL;
    }

    // convert NFA to DFA
    List dfaStates = NFAtoDFA(module->head);

    // first object in list of dfastates is the entrypoint
    Node entryNode = List_getHead(dfaStates);
    DFAState entry;
    if (entryNode != NULL) {
        entry = List_getObject(entryNode);
    }

    free_NFAModule(module);

    return new_RegEx(entry, dfaStates);
}

int match(RegEx re, char *str) {
    DFAState state = re->entry;

    // consume every character in string
    int i = 0;
    while (str[i] != 0) {
        // if state cannot transition on this character, fail
        if (!DFAState_hasTransition(state, str[i])) {
            return 0;
        }

        // move to appropriate state and continue reading input
        state = DFAState_getTransition(state, str[i]);
        i++;
    }

    // reached end of string, are we in a success state?
    return DFAState_getSuccess(state);
}

DFAState RegEx_getEntry(RegEx re) {
    return re->entry;
}
