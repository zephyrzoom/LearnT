/*
 * @author 707<707472783@qq.com>
 * This program calculate factorial.
 */
#include <iostream>
using std::cin;
using std::cout;
using std::endl;
int fact(int n) {
    int ret = 1;
    for (int i = 1; i <= n; i++) {
        ret *= i;
    }
    return ret;
}
int main()
{
    int n;
    cin >> n;
    cout << fact(n) << endl;
    return 0;
}
