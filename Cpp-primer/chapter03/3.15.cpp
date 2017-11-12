/*
 * @author 707<707472783@qq.com>
 * This program read some string and store in a vecotr.
 */
#include <iostream>
#include <vector>
using std::cin;
using std::cout;
using std::endl;
using std::vector;
using std::string;
int main()
{
    string s;
    vector<string> ss;
    while (cin >> s) {
        ss.push_back(s);
    }
    for (auto c : ss) {
        cout << c << " ";
    }
    cout << endl;
    return 0;
}
