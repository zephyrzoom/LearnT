/*
 * @author 707<707472783@qq.com>
 * This program reads two string then compare them 
 * and output the result.
 * if not equal, output the bigger.
 */

#include <iostream>
using std::cin;
using std::cout;
using std::endl;
using std::string;
int main()
{
    string s1;
    string s2;
    cout << "first string:";
    cin >> s1;
    cout << "second string:";
    cin >> s2;
    if (s1 != s2) {
        if (s1 > s2) {
            cout << "the bigger is: " << s1 << endl;
        } else {
            cout << "the bigger is: " << s2 << endl;
        }
    } else {
        cout << "they are equal." << endl;
    }

    if (s1.size() != s2.size()) {
        if (s1.size() > s2.size()) {
            cout << "the longer is: " << s1 << endl;
        } else {
            cout << "the longer is: " << s2 << endl;
        }
    } else {
        cout << "they are the same length." << endl;
    }
    return 0;
}
