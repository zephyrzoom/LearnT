/*
 * @author 707<707472783@qq.com>
 * This program count vowel include uppercase.
 */
#include <iostream>
using std::cin;
int main()
{
    char letter;
    int cnt = 0;
    switch (letter) {
        case 'a':
        case 'e':
        case 'i':
        case 'o':
        case 'u':
        case 'A':
        case 'E':
        case 'I':
        case 'O':
        case 'U': ++cnt; break;
        default: break;
    }
    return 0;
}
