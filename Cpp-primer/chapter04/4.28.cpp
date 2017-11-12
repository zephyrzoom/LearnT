/*
 * @author 707<707472783@qq.com>
 * This program outputs the inner type size.
 */
#include <iostream>
using std::cout;
using std::endl;
int main()
{
    cout << "char:" << sizeof(char) << endl;
    cout << "short:" << sizeof(short) << endl;
    cout << "int:" << sizeof(int) << endl;
    cout << "long:" << sizeof(long) << endl;
    cout << "long long:" << sizeof(long long) << endl;
    cout << "float:" << sizeof(float) << endl;
    cout << "double:" << sizeof(double) << endl;
    return 0;
}
