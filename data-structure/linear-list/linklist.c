#include <stdio.h>
#include<stdlib.h>

#define OK 1
#define ERROR 0

/*
 * some type define used. 
 */
typedef int Status;
typedef int ElemType;
typedef struct LNode {
	ElemType data;
	struct LNode *next;
} LNode, *LinkList;

/*
 * funciton declaration
 */
void Traverse(LinkList L);
void CreateList_L(LinkList* L, int n);
Status GetElem_L(LinkList L, int i, ElemType *e);
Status ListInsert_L(LinkList L, int i, ElemType e); 
Status ListDelete_L(LinkList L, int i, ElemType *e);
void MergeList_L(LinkList La, LinkList Lb, LinkList* Lc);

/*
 * main function.
 */
int main() {
	int elems = 0;
	LinkList L = NULL;
	LinkList La = NULL;
	LinkList Lb = NULL;
	LinkList Lc = NULL;
	ElemType e = 0;
	int pos = 0;
/*	printf("create a link list below\nhow many node you want to obtain:");	
	scanf("%d", &elems);
	CreateList_L(&L, elems);
	Traverse(L);	
	printf("which element you want to get, input the position:");
	scanf("%d", &pos);
	GetElem_L(L, pos, &e);
	printf("aha! it's %d\n", e);
	printf("input a insert value:");
	scanf("%d", &e);
	printf("choose a place to insert an element:");
	scanf("%d", &pos);
	ListInsert_L(L, pos, e); 
	Traverse(L);
	printf("which position you want to delete:");
	scanf("%d", &pos);
	ListDelete_L(L, pos, &e);
	Traverse(L);
*/
	printf("create a link list\ninput node amount:");
	scanf("%d", &elems);
	CreateList_L(&La, elems);
	Traverse(La);
	printf("create another link list\ninput node amount:");
	scanf("%d", &elems);
	CreateList_L(&Lb, elems);
	Traverse(Lb);
	MergeList_L(La, Lb, &Lc);
	printf("after merge into Lc:\n");
	Traverse(Lc);
}

/*
 * travese the link list.
 */
void Traverse(LinkList L) {
	LNode* temp = L->next;
	int iterator = 1;
	while (temp != NULL) {
		printf("%d: %d\n", iterator, temp->data);
		iterator++;
		temp = temp->next;
	}
}

/*
 * get certain element from the link list.
 */
Status GetElem_L(LinkList L, int i, ElemType *e) {
	LNode* p = L->next;
	int j = 1;
	while (p && j < i) {
		p = p->next;
		++j;
	}
	if (!p || j > i) {
		return ERROR;
	}
	*e = p->data;
	return OK;
}

/*
 * insert element to link list.
 */
Status ListInsert_L(LinkList L, int i, ElemType e) {
	LNode* p = L;
	LNode* s = NULL;
	int j = 0;
	while (p && j < i - 1) {
		p = p->next;
		++j;
	}
	if (!p || j > i-1) {
		return ERROR;
	}
	s = (LinkList)malloc(sizeof(LNode));
	s->data = e;
	s->next = p->next;
	p->next = s;
	return OK;
}

/*
 * delete element of the link list.
 */
Status ListDelete_L(LinkList L, int i, ElemType *e) {
	LNode* p = L;
	LNode* q = NULL;
	int j = 0;
	while (p->next && j < i - 1) {
		p = p->next;
		++j;
	}
	if (!(p->next) || j > i - 1) {
		return ERROR;
	}
	q = p->next;
	p->next = q->next;
	*e = q->data;
	free(q);
	return OK;
}

/*
 * create a link list.
 */
void CreateList_L(LinkList* L, int n) {
	LNode* p = NULL;
	int i = 0;
	*L = (LinkList)malloc(sizeof(LNode));
	(*L)->next = NULL;
	for (i = n; i > 0; --i) {
		p = (LinkList)malloc(sizeof(LNode));
		printf("%d:", n - i + 1);
		scanf("%d", &p->data);
		p->next = (*L)->next;
		(*L)->next = p;
	}
}

/*
 * merge two link list to one.
 */
void MergeList_L(LinkList La, LinkList Lb, LinkList* Lc) {
	LNode* pa = La->next;
	LNode* pb = Lb->next;
	LNode* pc = *Lc = La;
	while (pa && pb) {
		if (pa->data <= pb->data) {
			pc->next = pa;
			pc = pa;
			pa = pa->next;
		} else {
			pc->next = pb;
			pc = pb; 
			pb = pb->next;
		}
	}
	pc->next = pa ? pa : pb;
	free(Lb);
}
