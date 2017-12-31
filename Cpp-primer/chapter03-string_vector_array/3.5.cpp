/*
 * @author 707<707472783@qq.com>
 * This program reads several strings and concatenate them.
 */

#include <iostream>
using std::cin;
using std::cout;
using std::string;
using std::endl;
int main()
{
    string s;
    string sum;
    while (cin >> s)
    {
       sum += s; 
    }
    cout << sum << endl;

    while (cin >> s)
    {
        sum += s + " ";
    }
    cout << sum << endl;
    return 0;
}
