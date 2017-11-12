/*
 * @author 707<707472783@qq.com>
 * This program reads some words
 * and store in a vector then
 * change the chars to uppercase.
 */

#include <iostream>
#include <vector>
using std::string;
using std::vector;
using std::cin;
using std::cout;
using std::endl;
int main()
{
    string word;
    vector<string> vw;
    while (cin >> word) {
        vw.push_back(word);
    }
    for (auto &w : vw) {
        for (auto &s : w) {
            s = toupper(s);
        }
    }
    for (auto w : vw) {
        cout << w << endl;
    }
    return 0;
}
