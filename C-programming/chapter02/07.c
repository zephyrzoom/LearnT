/**
 * @author 707 <707472783@qq.com>
 *
 * This program is dividing the money into bills.
 */
#include <stdio.h>
int main(void)
{
    printf("Enter a dollar amount:");
    int amount = 0;
    scanf("%d", &amount);
    int d_20 = amount / 20;
    amount %= 20;
    int d_10 = amount / 10;
    amount %= 10;
    int d_5 = amount / 5;
    amount %= 5;
    int d_1 = amount;
    printf("$20 bills: %d\n"
           "$10 bills: %d\n"
           " $5 bills: %d\n"
           " $1 bills: %d\n", d_20, d_10, d_5, d_1);
}