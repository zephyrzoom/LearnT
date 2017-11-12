/*
 * @author 707<707472783@qq.com>
 * This program test overload f.
 */
#include <iostream>
using std::cout;
using std::endl;
void f()
{
    cout << "empty!" << endl;
}
void f(int i)
{
    cout << i << endl;
}
void f(int i1, int i2)
{
    cout << i1 << " " << i2 << endl;
}
void f(double d1, double d2 = 3.14)
{
    cout << d1 << " " << d2 << endl;
}

int main()
{
    f();
    f(707);
    f(7, 0);
    f(7.07, 7.07);
//    f(7, 7.07);
//    f(7.07, 7);
    return 0;
}
