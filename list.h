typedef struct List *List;
typedef struct Node *Node;

List new_List();
void free_List();

void List_add(List list, void *obj);
void List_addUnique(List list, void *obj);

Node List_getHead(List list);
void *List_getObject(Node node);
Node List_getPrev(Node node);
Node List_getNext(Node node);
int List_contains(List list, void *obj);
int List_empty(List list);