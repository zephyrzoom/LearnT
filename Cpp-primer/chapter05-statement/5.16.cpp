/*
 * @author 707<707472783@qq.com>
 * This program uses while and for
 * for the same aim.
 */
#include <iostream>
using std::cin;
int main()
{
    int cnt = 0;
    int i;
    while (cin >> i) {
        cnt++;
    }
    for (;cin >> i;) {
        cnt++;
    }

    int n[] = {1, 2, 3, 4, 5};
    for (auto v : n) {
        v++;
    }
    int j = 0;
    while (j < 5) {
        n[j]++;
        j++;
    }
    return 0;
}
