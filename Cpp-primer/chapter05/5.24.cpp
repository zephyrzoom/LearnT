/*
 * @author 707<707472783@qq.com>
 * This program input two integers
 * throw exception when the second is 0.
 */
#include <iostream>
#include <stdexcept>
using std::cin;
using std::cout;
using std::endl;
using std::runtime_error;
int main()
{
    int i1;
    int i2;
    cout << "Enter two integers:" << endl;
    cin >> i1 >> i2;
    if (i2 == 0) {
        throw runtime_error("can not be 0");
    }
    cout << i1 / i2 << endl;
    return 0;
}
