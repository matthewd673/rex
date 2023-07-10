#include <stdlib.h>
#include "list.h"

struct List {
    int count;
    Node head;
    Node tail;
};

struct Node {
    void *obj;
    Node next;
};

List new_List() {
    List new = (List)malloc(sizeof(struct List));
    if (new == NULL) {
        return NULL;
    }

    new->count = 0;
    new->head = NULL;
    new->tail = NULL;

    return new;
}

Node new_Node(void *obj) {
    Node new = (Node)malloc(sizeof(struct Node));
    if (new == NULL) {
        return NULL;
    }

    new->obj = obj;
    new->next = NULL;

    return new;
}

void List_add(List list, void *obj) {
    // list is empty, create head & tail
    if (list->tail == NULL) { // (head == NULL) == (tail == NULL)
        list->head = new_Node(obj);
        list->tail = list->head;
        list->count = 1;
        return;
    }

    // list is not empty, append
    list->tail->next = new_Node(obj);
    list->tail = list->tail->next;
    list->count++;
}

Node List_getHead(List list) {
    return list->head;
}

void *List_getObject(Node node) {
    return node->obj;
}

Node List_getNext(Node node) {
    return node->next;
}