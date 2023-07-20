#include <stdlib.h>
#include "list.h"

struct List {
    Node head;
    Node tail;
    int count;
};

struct Node {
    void *obj;
    Node prev;
    Node next;
    List list;
};

List new_List() {
    List new = (List)malloc(sizeof(struct List));
    if (new == NULL) {
        return NULL;
    }

    new->head = NULL;
    new->tail = NULL;
    new->count = 0;

    return new;
}

List free_List(List list) {
    // move through list freeing nodes (their objects are not freed)
    Node current = list->head;
    while (current != NULL) {
        Node next = current->next;
        free(current);
        current = next;
    }
    free(list);
}

Node new_Node(List list, void *obj) {
    Node new = (Node)malloc(sizeof(struct Node));
    if (new == NULL) {
        return NULL;
    }

    new->obj = obj;
    new->prev = NULL;
    new->next = NULL;
    new->list = list;

    return new;
}

void List_add(List list, void *obj) {
    // list is empty, create head & tail
    if (list->tail == NULL) { // (head == NULL) == (tail == NULL)
        list->head = new_Node(list, obj);
        list->tail = list->head;
        list->count++;
        return;
    }

    // list is not empty, append
    list->tail->next = new_Node(list, obj);
    list->tail->next->prev = list->tail;
    list->tail = list->tail->next;
    list->count++;
}

void List_addUnique(List list, void *obj) {
    if (List_contains(list, obj)) {
        return;
    }

    List_add(list, obj);
}

void List_remove(List list, Node node) {
    if (node->list != list) {
        return;
    }

    list->count--; // TODO: in edge cases this may be inaccurate
    if (node->prev != NULL) {
        node->prev->next = node->next;
    }
    if (node->next != NULL) {
        node->next->prev = node->prev;
    }
}

Node List_getHead(List list) {
    return list->head;
}

void *List_getObject(Node node) {
    return node->obj;
}

Node List_getPrev(Node node) {
    return node->prev;
}

Node List_getNext(Node node) {
    return node->next;
}

int List_contains(List list, void *obj) {
    Node current = list->head;
    while (current != NULL) {
        if (current->obj == obj) {
            return 1;
        }
        current = current->next;
    }
    return 0;
}

int List_empty(List list) {
    return list->count == 0;
}

int List_equals(List a, List b) {
    if (a->count != b->count) {
        return 0;
    }

    Node aNode = a->head;
    Node bNode;
    while (aNode != NULL) {
        // for every node in A, search through B
        bNode = b->head;
        while (bNode != NULL) {
            if (bNode == aNode) {
                break;
            }
            bNode = bNode->next;
        }
        // if we reach the end of B without matching they are different
        if (bNode == NULL) {
            return 0;
        }
        aNode = aNode->next;
    }

    return 1;
}