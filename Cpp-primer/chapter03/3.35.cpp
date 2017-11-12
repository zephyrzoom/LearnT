/*
 * @author 707<707472783@qq.com>
 * This program sets the array to zero by pointer.
 */

#include <iostream>
using std::cout;
using std::endl;
int main()
{
    int n[] = { 1, 2, 3, 4, 5, 6, 7, 8, 9, 10 };
    for (int *b = n, *e = n + 10; b != e; b++) {
        *b = 0;
    }
    for (auto i : n) {
        cout << i << endl;
    }
    return 0;
}
