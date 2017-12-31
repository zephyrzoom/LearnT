/*
 * @author 707<707472783@qq.com>
 * This program compares two integer.
 */
#include <iostream>
using std::cout;
using std::endl;
using std::cin;
int compare(const int &a, const int * const b)
{
    return a > *b ? a : *b;
}
int main()
{
    int a;
    int b;
    cout << "enter two integer:" << endl;
    cin >> a >> b;
    cout << compare(a, &b) << endl;
    return 0;
}
