/*
 * @author 707<707472783@qq.com>
 * This program store 10 int vector
 * then double the numbers.
 */

#include <iostream>
#include <vector>
using std::cout;
using std::cin;
using std::endl;
using std::vector;
int main()
{
    vector<int> vi;
    int n;
    cout << "enter 10 numbers:" << endl;
    for (int i = 0; i < 10; i++) {
        cin >> n;
        vi.push_back(n);
    }

    for (auto i = vi.begin(); i != vi.end(); i++) {
        *i = (*i) * (*i);
    }

    cout << "result:" << endl;
    for (auto i = vi.cbegin(); i != vi.cend(); i++) {
        cout << *i << endl;
    }
    return 0;
}
