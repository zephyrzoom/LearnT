/*
 * @author 707<707472783@qq.com>
 * This program defines the fact function.
 */
#include "Chapter6.h"
int fact(int n) {
    int ret = 1;
    for (int i = 1; i <= n; i++) {
        ret *= i;
    }
    return ret;
}
