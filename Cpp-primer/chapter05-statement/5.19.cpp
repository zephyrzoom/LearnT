/*
 * @author 707<707472783@qq.com>
 * This program picks up the shorter string.
 */
#include <iostream>
using std::string;
using std::cin;
using std::cout;
using std::endl;
int main()
{
    string quit;
    do {
        string s1;
        string s2;
        cout << "enter two strings:" << endl;
        cin >> s1 >> s2;
        if (s1.size() < s2.size()) {
            cout << s1 << endl;
        } else {
            cout << s2 << endl;
        }
        cout << "quit? (yes or no)" << endl;
        cin >> quit;
    } while (!quit.empty() && quit[0] != 'y');
    return 0;
}
