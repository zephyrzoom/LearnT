/*
 * @author 707<707472783@qq.com>
 * This program tests whether the two same word with the uppercase come out.
 */
#include <iostream>
using std::cin;
using std::cout;
using std::endl;
using std::string;
int main()
{
    string word;
    string tmp;
    bool isSame = false;
    cout << "enter some words:" << endl;
    while (cin >> word) {
        if (word == tmp) {
            if (isupper(word[0])) {
                isSame = true;
                cout << "the same is:" << word << endl;
                break;
            } else {
                continue;
            }
        } else {
            tmp = word;
        }
    }
    if (!isSame) {
        cout << "the same not come." << endl;
    }
    
    return 0;
}
