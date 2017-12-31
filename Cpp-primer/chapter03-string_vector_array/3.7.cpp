/*
 * @author 707<707472783@qq.com>
 * This program is rewrite the last program 
 * replace 'auto' by 'char'.
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
    for (char &c : s) {
       c = 'X'; 
    }
    cout << s << endl;
    return 0;
}
