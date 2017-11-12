/*
 * @author 707<707472783@qq.com>
 * This program define a 10 integer array
 * set every elementary is its subscript.
 */
#include <iostream>
#include <vector>
using std::cout;
using std::endl;
using std::vector;
int main()
{
    int n[10] = {};
    for (size_t i = 0; i < 10; i++) {
        n[i] = i;
    }

    int another[10] = {};
    for (size_t i = 0; i < 10; i++) {
        another[i] = n[i];
    }

    for (auto i : n) {
        cout << i << endl;
    }

    cout << "another:" << endl;
    for (auto i : another) {
        cout << i << endl;
    }

    vector<int> vn;
    for (auto i : n) {
        vn.push_back(i);
    }
    cout << "vector:" << endl;
    for (auto i : n) {
        cout << i << endl;
    }
    
    return 0;
}
