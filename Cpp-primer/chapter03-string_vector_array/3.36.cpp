/*
 * @author 707<707472783@qq.com>
 * This program is compare two arrays
 * then compare two vectors.
 */
#include <iostream>
#include <vector>
using std::cout;
using std::endl;
using std::vector;
int main()
{
    int a[10] = { 1, 2 };
    int b[10] = { 1, 2, 3 };
    bool equal = true;
    for (int *b1 = a, *b2 = b; b1 < a + 10; b1++, b2++) {
        if (*b1 != *b2) {
            equal = false;
            break;
        }
    }
    cout << equal << endl;

    equal = true;
    vector<int> va(10);
    vector<int> vb(10);
    for (auto b1 = va.cbegin(), b2 = vb.cbegin(); b1 != va.cend(); b1++, b2++) {
        if (*b1 != *b2) {
            equal = false;
            break;
        }
    }
    cout << equal << endl;
    return 0;
}
