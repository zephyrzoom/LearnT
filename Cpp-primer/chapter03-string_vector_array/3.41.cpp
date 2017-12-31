/*
 * @author 707<707472783@qq.com>
 * This program init a vector by int array.
 */
#include <iostream>
#include <vector>
#include <iterator>
using std::cout;
using std::endl;
using std::vector;
using std::begin;
using std::end;
int main()
{
    int n[] = { 1, 2, 3, 4 };
    vector<int> v(begin(n), end(n));
    for (auto i : v) {
        cout << i << " ";
    }
    cout << endl;
    return 0;
}
