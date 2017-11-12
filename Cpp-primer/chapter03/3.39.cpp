/*
 * @author 707<707472783@qq.com>
 * This program compare two strings and two cstrings.
 */
#include <iostream>
#include <string>
#include <cstring>
using std::cout;
using std::endl;
using std::string;
int main()
{
    string s1 = "abcdefg";
    string s2 = "abcde";
    if (s1 < s2) {
        cout << "s1 less than s2" << endl;
    } else if (s1 > s2) {
        cout << "s1 bigger than s2" << endl;
    } else {
        cout << "equal" << endl;
    }

    const char c1[] = "abcde";
    const char c2[] = "abcde";
    auto cmp = strcmp(c1, c2);
    if (cmp < 0) {
        cout << "c1 less than c2" << endl;
    } else if (cmp > 0) {
        cout << "c1 bigger than c2" << endl;
    } else {
        cout << "equal" << endl;
    }
    return 0;
}
