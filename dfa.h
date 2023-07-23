#include "list.h"

typedef struct DFAState *DFAState;

DFAState new_DFAState();

void DFAState_addTransition(DFAState src, DFAState dst, char c);
DFAState DFAState_getTransition(DFAState state, char c);
int DFAState_hasTransition(DFAState state, char c);

void DFAState_setTag(DFAState state, List tag);
List DFAState_getTag(DFAState state);

void DFAState_setSuccess(DFAState state, char success);
char DFAState_getSuccess(DFAState state);
