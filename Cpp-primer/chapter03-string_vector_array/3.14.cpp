/*
 * @author 707<707472783@qq.com>
 * This program reads a set of int and store in a vector.
 */
#include <iostream>
#include <vector>
using std::cin;
using std::cout;
using std::endl;
using std::vector;
int main()
{
    int i;
    vector<int> is;
    while (cin >> i) {
        is.push_back(i);
    }
    for (auto c : is) {
        cout << c << " ";
    }
    cout << endl;
    return 0;
}
