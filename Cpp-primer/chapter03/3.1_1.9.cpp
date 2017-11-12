/*
 * @author 707<707472783@qq.com>
 * This program is add from 50 to 100.
 */

#include <iostream>
using std::cout;
using std::endl;
int main()
{
    int val = 50;
    int sum = 0;
    while (val <= 100)
    {
        sum += val;
        ++val;
    }
    cout << "The sum from 50 to 100 is " << sum << endl;
    return 0;
}
