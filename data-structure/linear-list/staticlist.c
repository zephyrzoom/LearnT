#include <stdio.h>

#define MAXSIZE 1000

typedef int ElemType;
typedef struct {
	ElemType data;
	int cur;
} component, SLinkList[MAXSIZE];

/*
 * functions declaration.
 */
int LocateElem_SL(SLinkList S, ElemType e);
void difference(SLinkList* space, int* S);
void InitSpace_SL(SLinkList* space);
int Malloc_SL(SLinkList* space);
void Free_SL(SLinkList* space, int k);
void Traverse(SLinkList S);

/*
 * main function.
 */
int main() {
	SLinkList space;
	int S;
	difference(&space, &S);
}

/*
 * traverse the static link list.
 */
void Traverse(SLinkList S) {

}

/*
 * locate the element.
 */
int LocateElem_SL(SLinkList S, ElemType e) {
	int i = S[0].cur;
	while (i && S[i].data != e) {
		i = S[i].cur;
	}
	return i;
}

/*
 * initialize static list.
 */
void InitSpace_SL(SLinkList* space) {
	int i;
	for (i = 0; i < MAXSIZE - 1; ++i) {
		(*space)[i].cur = i + 1;
	}
	(*space)[MAXSIZE - 1].cur = 0;
}

/*
 * generate a head node. 
 */
int Malloc_SL(SLinkList* space) {
	int i = (*space)[0].cur;
	if ((*space)[0].cur) {
		(*space)[0].cur = (*space)[i].cur;
	}
	return i;
}

/*
 * delete the element.
 */
void Free_SL(SLinkList* space, int k) {
	(*space)[k].cur = (*space)[0].cur;
	(*space)[0].cur = k;
}

/*
 * create S static list from the terminal, then create b static list.
 * if input value has stored in S, then delete the value of S, else insert into S. 
 * function is (A-B)U(B-A)
 */
void difference(SLinkList* space, int* S) {
	InitSpace_SL(space);
	*S = Malloc_SL(space);
	int r = *S;
	int m = 0;
	int n = 0;
	int j = 0;
	int i = 0;
	int b = 0;
	int p = 0;
	int k = 0;
	printf("input list S and B's element amount:");
	scanf("%d %d", &m, &n);
	printf("input S elements:\n");
	for (j = 1; j <= m; ++j) {
		i = Malloc_SL(space);
		printf("%d:", j);
		scanf("%d", &(*space)[i].data);
		(*space)[r].cur = i;
		r = i;
	}
	(*space)[r].cur = 0;
	printf("input B elements:\n");
	for (j = 1; j <= n; ++j) {
		printf("%d:", j);
		scanf("%d", &b);
		p = *S;
		k = (*space)[*S].cur;
		while (k != (*space)[r].cur && (*space)[k].data != b) {
			p = k;
			k = (*space)[k].cur;
		}
		if (k == (*space)[r].cur) {
			i = Malloc_SL(space);
			(*space)[i].data = b;
			(*space)[i].cur = (*space)[r].cur;
			(*space)[r].cur = i;
		} else {
			(*space)[p].cur = (*space)[k].cur;
			Free_SL(space, k);
			if (r == k) {
				r = p;
			}
		}
	}
}
