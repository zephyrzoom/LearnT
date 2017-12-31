/*
 * @author 707<707472783@qq.com>
 * This program is reading a line and output.
 * Then read a word and output.
 */

#include <iostream>
using std::string;
using std::cout;
using std::cin;
using std::endl;
int main()
{
    string s;
    getline(cin, s);
    cout << s << endl;
    string word;
    cin >> word;
    cout << word << endl;
    return 0;
}
