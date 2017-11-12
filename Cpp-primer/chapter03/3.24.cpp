/*
 * @author 707<707472783@qq.com>
 * This program reads a set of numbers
 * and add the adjacent numbers 
 * then add the corresponding numbers.
 */
#include <iostream>
#include <vector>
using std::vector;
using std::cin;
using std::cout;
using std::endl;

int main()
{
    cout << "enter numbers:" << endl;
    int nu;
    vector<int> nus;
    while (cin >> nu) {
        nus.push_back(nu);
    }
    cout << "output1:" << endl;
    for (auto i = nus.cbegin(); i < nus.cend(); i+=2) {
        if (i+1 != nus.cend()) {
            cout << *i + *(i+1) << endl;
        } else {
            cout << *i << endl;
        }
    }
    cout << "output2:" << endl;
    for (auto i = nus.cbegin(), j = nus.cend() - 1; i <= j; i++, j--) {
        if (i == j) {
            cout << *i << endl;
        } else {
            cout << *i + *j << endl;
        }
    }
    return 0;
}
