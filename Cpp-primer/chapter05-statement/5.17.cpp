/*
 * @author 707<707472783@qq.com>
 * This program is test whether one vector
 * include the other vector.
 */
#include <iostream>
#include <vector>
using std::vector;
using std::cout;
using std::endl;
int main()
{
    vector<int> v1 = {0, 1, 1, 2};
    vector<int> v2 = {0, 1, 1, 2, 4, 5, 8};
    bool is = true;
    if (v1.size() < v2.size()) {
        for (auto i = v1.cbegin(), j = v2.cbegin(); i != v1.cend(); i++, j++) {
            if (*i != *j) {
                is = false;
            }
        }
    } else {
        for (auto i = v1.cbegin(), j = v2.cbegin(); j != v2.cend(); i++, j++) {
            if (*i != *j) {
                is = false;
            }
        }
    }
    cout << is << endl;
    return 0;
}
