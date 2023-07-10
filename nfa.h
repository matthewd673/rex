#include "list.h"

typedef struct NFAState *NFAState;

NFAState new_NFAState();

void NFAState_addTransition(NFAState src, NFAState dst, char c);
List NFAState_getTransitions(NFAState state, char c);

void NFAState_setSuccess(NFAState state, char success);
char NFAState_getSuccess(NFAState state);