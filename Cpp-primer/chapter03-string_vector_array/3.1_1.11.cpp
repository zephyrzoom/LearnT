/*
 * @author 707<707472783@qq.com>
 * This program is let user input two numbers and print the numbers between them.
 */

#include <iostream>
using std::cout;
using std::cin;
using std::endl;
int main()
{
    int little = 0;
    int big = 0;
    cout << "Enter a little number: ";
    cin >> little;
    cout << "Enter a big number: ";
    cin >> big;
    while (little <= big)
    {
        cout << little << " ";
        little++;
    }
    cout << endl;
    return 0;
}
