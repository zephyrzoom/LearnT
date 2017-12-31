/*
 * @author 707<707472783@qq.com>
 * This program count vowel by if else.
 */
#include <iostream>
using std::cin;
int main()
{
    char letter;
    int cnt = 0;
    while (cin >> letter) {
        if (letter == 'a') {
            ++cnt;
        } else if (letter == 'e') {
            ++cnt;
        } else if (letter == 'i') {
            ++cnt;
        } else if (letter == 'o') {
            ++cnt;
        } else if (letter == 'u') {
            ++cnt;
        }
    }
    return 0;
}
