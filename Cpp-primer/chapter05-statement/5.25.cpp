/*
 * @author 707<707472783@qq.com>
 * This program input two integers
 * catch the 0 exception and 
 * let user re input.
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
    while (cin >> i1 >> i2) {
        try {
            if (i2 == 0) {
                throw runtime_error("can not be 0");
            } else {
                cout << i1 / i2 << endl;
            }
        } catch (runtime_error err) {
            cout << err.what()
                << "\nTry again? Enter y or n" << endl;
            char c;
            cin >> c;
            if (!cin || c == 'n') {
                break;
            }
        }
    }
    return 0;
}
