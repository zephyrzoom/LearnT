/*
 * 计算i!*2^i
 */
#include <stdio.h>
#define ARRSIZE 100
int main () {
    int i = 1;
    long sum = 1;
    long tempSum;
    long a[ARRSIZE] = {0};
    while (i < ARRSIZE) {
       	tempSum = sum * i * 2;
	if (tempSum / i / 2 == sum) {
	    a[i-1] = tempSum;
	    sum = tempSum;
	} else {
	    break;
	}
	i++;
    }
    for (i = 0; i < ARRSIZE; i++) {
	printf("a[%d] = %lld\n", i, a[i]);
    }
}
