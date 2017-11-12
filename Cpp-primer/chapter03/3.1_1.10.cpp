/*
 * @author 707<707472783@qq.com>
 * This program is print the number from 10 to 0.
 */

#include <iostream>
using std::cout;
using std::endl;
int main()
{
    int val = 10;
    while(val >= 0)
    {
        std::cout << val << " ";
        val--;
    }
    cout << endl;
    return 0;
}
