/**
 * @author 707<707472783@qq.com>
 *
 * This program is calculate the loan.
 */
#include <stdio.h>
int main(void)
{
    printf("Enter amount of loan:");
    float loan = 0;
    scanf("%f", &loan);

    printf("Enter interest rate:");
    float rate = 0;
    scanf("%f", &rate);

    printf("Enter monthly payment:");
    float payment = 0;
    scanf("%f", &payment);

    float remain_fst = 0;
    float remain_scd = 0;
    float remain_trd = 0;

    remain_fst = loan * (1 + rate / 100 / 12) - payment;
    remain_scd = remain_fst * (1 + rate / 100 / 12) - payment;
    remain_trd = remain_scd * (1 + rate / 100 / 12) - payment;

    printf("Balance remaining after first payment: $%.2f\n"
           "Balance remaining after second payment: $%.2f\n"
           "Balance remaining after third payment: $%.2f\n",
           remain_fst, remain_scd, remain_trd);
}