/*
 * 求k阶fabonacci数列的第m项
 */
#include <stdio.h>

int fabonacci(int k, int m);

int main() {
    int k = 0;	/* k阶 */
    int m = 0;	/* 第m项 */
    printf("input k:");
    scanf("%d", &k);
    printf("input m:");
    scanf("%d", &m);
    printf("the corresponding fabonacci is %d\n", fabonacci(k, m));
    return 0;
}

/*
 * 求出k阶fabonacci数列的第m项 
 * m从0开始
 */
int fabonacci(int k, int m) {
    int i;
    int result = 0;
    if (m < k - 1) {
	return 0;
    } else if (m == k - 1) {
        return 1;
    } else {	/* 前k项相加 */
        for (i = 0; i < k; i++) {
	    result += fabonacci(k, m - i - 1);
	}
    }
    return result;
}
