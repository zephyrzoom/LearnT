/*
 * @author 707<707472783@qq.com>
 * This program output first para
 * and change them to uppercase.
 */

#include <iostream>
#include <vector>
using std::cin;
using std::cout;
using std::string;
using std::vector;
using std::endl;
int main()
{
    string l;
    vector<string> vs;
    while (getline(cin, l)) {
        vs.push_back(l);
    }
    auto i = vs.begin();
    if (i != vs.end()) {
        for (auto ii = i->begin(); ii != i->end(); ii++) {
            *ii = toupper(*ii);
        }
    }
    for (auto i = vs.cbegin(); i != vs.cend(); i++) {
        cout << *i << endl;
    }
    return 0;
}
