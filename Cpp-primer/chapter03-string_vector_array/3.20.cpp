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
    for (decltype(nus.size()) i = 0; i < nus.size(); i+=2) {
        if (i+1 < nus.size()) {
            cout << nus[i] + nus[i+1] << endl;
        } else {
            cout << nus[i] << endl;
        }
    }
    for (decltype(nus.size()) i = 0, j = nus.size() - 1; i < j; i++, j--) {
        cout << nus[i] + nus[j] << endl;
    }
    return 0;
}
