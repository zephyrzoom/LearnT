/*
 * @author 707<707472783@qq.com>
 * This program validate ur reset function
 * by reference type.
 */
#include <iostream>
using std::cout;
using std::endl;
void reset(int &i)
{
    i = 707;
}
int main()
{
    int i = 1;
    reset(i);
    cout << i << endl;
    return 0;
}
