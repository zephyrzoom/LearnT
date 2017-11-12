/**
 * @author 707 <707472783@qq.com>
 *
 * This program is computing tax.
 *
 */
#include <stdio.h>
int main(void)
{
    printf("Enter an amount:");
    float amount = 0;
    scanf("%f", &amount);
    float tax = 0;
    tax = amount * (1 + 0.05);
    printf("With tax added: $%.2f\n", tax);
}