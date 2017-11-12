/*
 * @author 707<707472783@qq.com>
 * This program is test whether the element is odd
 * in the vector and then double the odd.
 */
#include <iostream>
#include <vector>
using std::cout;
using std::endl;
using std::vector;
int main()
{
    vector<int> vi = {1, 2, 3, 4, 5};
    for (auto &i : vi) {
        i % 2 == 0 ? : i *= 2;
    }

    for (auto i : vi) {
        cout << i << " ";
    }
    cout << endl;
    return 0;
}
