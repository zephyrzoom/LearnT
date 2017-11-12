/**
 * @author 707 <707472783@qq.com>
 *
 * This program is computing tax with Horner's Rule.
 *
 */
#include <stdio.h>
int main(void)
{
    printf("input x:");
    int x = 0;
    scanf("%d", &x);
    int value = ((((3*x + 2)*x - 5)*x - 1)*x + 7)*x - 6;
    printf("value of the formula is %d\n", value);
}