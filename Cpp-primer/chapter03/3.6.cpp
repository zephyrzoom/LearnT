/*
 * @author 707<707472783@qq.com>
 *This program replace the string by 'X'
 */

#include <iostream>
using std::cin;
using std::cout;
using std::endl;
using std::string;

int main()
{
    string s;
    cin >> s;
    for (auto &c : s) {
       c = 'X'; 
    }
    cout << s << endl;
    return 0;
}
