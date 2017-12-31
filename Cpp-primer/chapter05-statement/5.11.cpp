/*
 * @author 707<707472783@qq.com>
 * This program counts vowel include uppercase and counts space tab return.
 */
#include <iostream>
using std::cin;
int main()
{
    char letter;
    int cnt = 0;
    int space = 0;
    int tab = 0;
    int ret = 0;
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
        case ' ': ++space; break;
        case '\t': ++tab; break;
        case '\n': ++ret; break;
        default: break;
    }
    return 0;
}
