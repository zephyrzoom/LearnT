/**
 * @author 707 <707472783@qq.com>
 *
 * This program is computing tax.
 *
 */
#include <stdio.h>
int main(void)
{
    printf("input x:");
    int x = 0;
    scanf("%d", &x);
    int value = 3*x*x*x*x*x + 2*x*x*x*x - 5*x*x*x - x*x + 7*x - 6;
    printf("value of the formula is %d\n", value);
}