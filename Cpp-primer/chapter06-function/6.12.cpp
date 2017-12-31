/*
 * @author 707<707472783@qq.com>
 * This program uses pointer parameters exchange two value
 * by reference.
 */
#include <iostream>
using std::cout;
using std::endl;
void exchange(int &pi1, int &pi2)
{
    pi2 = pi1 + pi2 - (pi1 = pi2);
}
int main()
{
    int i1 = 7;
    int i2 = 0;
    exchange(i1, i2);
    cout << i1 << " " << i2 << endl;
    return 0;
}
