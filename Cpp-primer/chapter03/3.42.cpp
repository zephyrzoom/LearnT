/*
 * @author 707<707472783@qq.com>
 * This program copys a vector to an array.
 */
#include <iostream>
#include <vector>
using std::cout;
using std::endl;
using std::vector;
int main()
{
    vector<int> v = { 7, 0, 7 };
    int n[3];
    size_t i = 0;
    for (auto begin = v.cbegin(); begin != v.cend(); begin++) {
        n[i] = *begin;
        i++;
    }
    for (auto i : n) {
        cout << i << " ";
    }
    cout << endl;
    return 0;
}
