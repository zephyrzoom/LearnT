/**
 * @author 707 <707472783@qq.com>
 *
 * This program is used to compute volume of a sphere by inputing a radius.
 */
#include <stdio.h>
#include <math.h>
int main(void)
{
    float r = 0;
    printf("input a radius:");
    scanf("%f", &r);

    float v = 0;
    // M_PI come from math.h
    v = 4.0/3 * M_PI * r*r*r;
    printf("volume is %.2f\n", v);
    return 0;
}