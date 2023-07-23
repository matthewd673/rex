#include "list.h"
#include "dfa.h"

#define EPSILON     0
#define ALPHABET    128

typedef struct NFAState *NFAState;

NFAState new_NFAState();
void free_NFAState(NFAState state);

void NFAState_addTransition(NFAState src, NFAState dst, char c);
List NFAState_getTransitions(NFAState state, char c);

void NFAState_setSuccess(NFAState state, char success);
char NFAState_getSuccess(NFAState state);

List NFAtoDFA(NFAState entry);
