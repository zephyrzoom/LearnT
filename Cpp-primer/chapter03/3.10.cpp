/*
 * @author 707<707472783@qq.com>
 * This program is to remove the punct.
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
    for (decltype(s.size()) i = 0; i < s.size(); i++)
    {
        if (ispunct(s[i])) {
            for (decltype(s.size()) j = i; j < s.size(); j++)
            {
                s[j] = s[j+1];
            }
            i--;
        }
        
    }
    cout << s << endl;
    return 0;
}
