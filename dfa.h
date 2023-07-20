typedef struct DFAState *DFAState;

DFAState new_DFAState();

void DFAState_addTransition(DFAState src, DFAState dst, char c);
DFAState DFAState_getTransition(DFAState state, char c);

void DFAState_setSuccess(DFAState state, char success);
char DFAState_getSuccess(DFAState state);

void DFAState_print(DFAState state);
