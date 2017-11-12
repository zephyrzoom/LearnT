/**
 * @author 707 <707472783@qq.com>
 *
 * This program is compute volume of a sphere.
 */

#include <stdio.h>
#include <math.h>
int main(void)
{
    int r = 10;
    float v = 0;
    // M_PI come from math.h
    v = 4.0/3 * M_PI * r*r*r;
    printf("volume is %.2f\n", v);
    return 0;
}