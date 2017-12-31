/*
 * @author 707<707472783@qq.com>
 * This program counts vowel include uppercase and counts "ff fl fi".
 */
#include <iostream>
using std::cin;
int main()
{
    char letter;
    int cnt = 0;
    bool isF = false;
    int ff = 0;
    int fl = 0;
    int fi = 0;
    switch (letter) {
        case 'a':
        case 'e':
        case 'i': {
                      if (isF) {
                          ++fi;
                          break;
                      }
                  }
        case 'o':
        case 'u':
        case 'A':
        case 'E':
        case 'I':
        case 'O':
        case 'U': ++cnt; break;
        case 'f': {
                      if (isF) {
                          ++ff;
                      } else {
                          isF = true;
                      }
                  }
                  break;
        case 'l': {
                      if (isF) {
                          ++fl;
                          break;
                      }
                  }
        default: break;
    }
    return 0;
}
