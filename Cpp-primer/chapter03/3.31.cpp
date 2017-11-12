/*
 * @author 707<707472783@qq.com>
 * This program define a 10 integer array
 * set every elementary is its subscript.
 */
#include <iostream>
using std::cout;
using std::endl;
int main()
{
    int n[10];
    for (size_t i = 0; i < 10; i++) {
        n[i] = i;
    }
    for (auto i : n) {
        cout << i << endl;
    }
    return 0;
}
