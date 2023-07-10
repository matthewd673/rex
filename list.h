typedef struct List *List;
typedef struct Node *Node;

List new_List();

void List_add(List list, void *obj);

Node List_getHead(List list);
void *List_getObject(Node node);
Node List_getNext(Node node);