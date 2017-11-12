/*
 * @author 707<707472783@qq.com>
 * This program returns the absolute value.
 */
#include <iostream>
using std::cin;
using std::cout;
using std::endl;

double abs(double n)
{
    return n > 0 ? n : -n;
}
int main()
{
    cout << "enter a number:" << endl;
    double n;
    cin >> n;
    cout << "abs is:" << abs(n) << endl;
    return 0;
}


