/*
 * 计算一元多项式
 */
#include <stdio.h>
#define MAXSIZE 100
int main() {
    int x;
    int n;
    int a[MAXSIZE];
    int ai;
    int i;
    int result = 0;
    printf("input x:");
    scanf("%d", &x);
    printf("input n:");
    scanf("%d", &n);
    printf("input a0...an\n");
    for (i = 0; i < n; i++) {
	printf("a%d:", i);
	scanf("%d", &a[i]);
    }

    for (i = n-1; i >= 0; i--) {
	result = x * result + a[i];
    }
    printf("the result is %d\n", result);
    
    return 0;
}
