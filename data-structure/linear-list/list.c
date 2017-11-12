#include <stdio.h>
#include <stdlib.h>

#define TRUE 1
#define FALSE 0
#define OK 1
#define ERROR 0
#define INFEASIBLE -1
#define OVERFLOW -2

// function return status 
typedef int Status;
typedef int ElemType;

// SqList 
#define LIST_INIT_SIZE 100
#define LIST_INCREMENT 10

typedef struct {
	ElemType* elem;
	int length;
	int listsize;
} SqList;

// function declare
Status InitList_Sq(SqList* L);
Status ListInsert_Sq(SqList* L, int i, ElemType e);
Status ListDelete_Sq(SqList* L, int i, ElemType* e);
int LocationElem_Sq(SqList *L, ElemType e, Status (*compare)(ElemType, ElemType));
Status compare(ElemType e1, ElemType e2);
void MergeList_Sq(SqList La, SqList Lb, SqList* Lc);

// main function
int main() {
	SqList L;
	SqList La;
	SqList Lb;
	SqList Lc;
	int del = 0;
	int pos = 0;
	int i = 0;
	int elems = 0;
	ElemType input = 0;
/*	InitList_Sq(&L);
	printf("L.length = %d, L.listsize = %d.\n", L.length, L.listsize);
	printf("how many element you want to input:");
	scanf("%d",&elems);
	for (i = 0; i < elems; i++) {
		printf("%d:", i+1);
		scanf("%d", &input);
		ListInsert_Sq(&L, i+1, input);
	}
	printf("elements in the SqList:\n");
	for (i = 1; i <= L.length; i++) {
		printf("%d:%d\n", i, L.elem[i-1]);
	}
	printf("choose which you want to delete:");
	scanf("%d", &pos);
	ListDelete_Sq(&L, pos, &del);
	printf("after delete, elements in the SqList:\n");
	for (i = 1; i <= L.length; i++) {
		printf("%d:%d\n", i, L.elem[i-1]);
	}
	printf("input an element ,you can get it's position:");
	scanf("%d", &input);
	printf("location is %d\n", LocationElem_Sq(&L, input, compare));
*/	
	InitList_Sq(&La);
	InitList_Sq(&Lb);
	InitList_Sq(&Lc);
	printf("create La SqList, input elements amount:");
	scanf("%d", &elems);
	for (i = 0; i < elems; i++) {
		printf("%d:", i+1);
		scanf("%d", &input);
		ListInsert_Sq(&La, i+1, input);
	}
	printf("create Lb SqList, input elements amount:");
	scanf("%d", &elems);
	for (i = 0; i < elems; i++) {
		printf("%d:", i+1);
		scanf("%d", &input);
		ListInsert_Sq(&Lb, i+1, input);
	}
	printf("elements in the La SqList:\n");
	for (i = 1; i <= La.length; i++) {
		printf("%d:%d\n", i, La.elem[i-1]);
	}
	printf("elements in the Lb SqList:\n");
	for (i = 1; i <= Lb.length; i++) {
		printf("%d:%d\n", i, Lb.elem[i-1]);
	}
	MergeList_Sq(La, Lb, &Lc);
	printf("elements in the Lc SqList:\n");
	for (i = 1; i <= Lc.length; i++) {
		printf("%d:%d\n", i, Lc.elem[i-1]);
	}
	
	/*ListInsert_Sq(&L, 1, 8);
	ListInsert_Sq(&L, 2, 9);
	printf("%d", L.elem[0]);
	ListDelete_Sq(&L, 1, &del);
	printf("getDel: %d\n", del);
	printf("deleted: %d\n", L.elem[0]);
	printf("length: %d\n", L.length);
	for (i = 0; i < 10; i++) {
		ListInsert_Sq(&L, i+2, 10+i);
	}
	for (i = 1; i <= L.length; i++) {
		printf("%d: %d\n", i, L.elem[i-1]);
	}
	printf("10 is in the location:%d\n", LocationElem_Sq(&L, 17, compare));	
	*/
}

// SqList initialize
Status InitList_Sq(SqList* L) {
	L->elem = (int *)malloc(LIST_INIT_SIZE * sizeof(ElemType));
	if(!L->elem) {
		exit(OVERFLOW);
	}
	L->length = 0;
	L->listsize = LIST_INIT_SIZE;
	return OK;
}

// SqList insert
Status ListInsert_Sq(SqList* L, int i, ElemType e) {
	int* q = &(L->elem[i-1]);
	int* p;
	if (i < 1 || i > L->length + 1 ) {
		return ERROR;
	}
	if (L->length >= L->listsize) {
		int* newbase = (int*)realloc(L->elem, (L->listsize + LIST_INCREMENT) * sizeof(ElemType));
		if (!newbase) {
			exit(OVERFLOW);
		}
		L->elem = newbase;
		L->listsize += LIST_INCREMENT;
	}
	
	for(p = &(L->elem[L->length - 1]); p >= q; --p) {
		*(p+1) = *p;
	}
	*q = e;
	++L->length;
	return OK;

}

// SqList delete
Status ListDelete_Sq(SqList* L, int i, ElemType* e) {
	int *p;
	int *q;
	if ((i < 1) || (i > L->length)) {
		return ERROR;
	}
	p = &(L->elem[i-1]);
	*e = *p++;
	q = L->elem + L->length - 1;
	for (; p <= q; ++p) {
		*(p-1) = *p;
	}
	--L->length;
	return OK;
}

// locate SqList element
int LocationElem_Sq(SqList *L, int e, Status (*compare)(ElemType, ElemType)) {
	int i = 1;
	ElemType* p = L->elem;
	while (i <= L->length && !(*compare)(*p++, e)) {
		++i;
	}
	if (i <= L->length) {
		return i;
	} else {
		return 0;
	}	
}

// compare element function
Status compare(ElemType e1, ElemType e2) {
	if (e1 == e2) {
		return TRUE;
	} else {
		return FALSE;
	}
}

// merge two list
void MergeList_Sq(SqList La, SqList Lb, SqList* Lc) {
	ElemType* pa = La.elem;
	ElemType* pb = Lb.elem;
	Lc->listsize = Lc->length = La.length + Lb.length;
	ElemType* pc = Lc->elem = (ElemType*)malloc(Lc->listsize * sizeof(ElemType));	
	if (!Lc->elem) {
		exit(OVERFLOW);
	}
	ElemType* pa_last = La.elem + La.length - 1;
	ElemType* pb_last = Lb.elem + Lb.length - 1;
	while (pa <= pa_last && pb <= pb_last) {
		if (*pa <= *pb) {
			*pc++ = *pa++;
		} else {
			*pc++ = *pb++;
		}
	}
	while (pa <= pa_last) {
		*pc++ = *pa++;
	}
	while (pb <= pb_last) {
		*pc++ = *pb++;
	}
}	
