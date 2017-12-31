/*
 * @author 707<707472783@qq.com>
 * This program test the string whether
 * include uppercase.
 * And another function change the string
 * to lowercase.
 */
#include <iostream>
using std::string;
using std::cout;
using std::endl;
bool hasUp(const string &s)
{
    for (auto i : s) {
        if (isupper(i)) {
            return true;
        }
    }
    return false;
}

void changeLower(string &s)
{
    for (auto &i : s) {
        if (islower(i)) {
            i = toupper(i);
        }
    }
}
int main()
{
    cout << hasUp("abcd") << endl;
    cout << hasUp("dfA") << endl;
    string s = "abcDe";
    changeLower(s);
    cout << s << endl;
    return 0;
}
