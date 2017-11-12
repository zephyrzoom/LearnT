/*
 * @author 707<707472783@qq.com>
 * This program defines print function.
 */
#include <iostream>
using std::cout;
using std::endl;
void print(const int &i)
{
    cout << i << endl;
}

void print(const int *const p, const int n)
{
    for (size_t i = 0; i != n; i++) {
        cout << p[i] << " ";
    }
    cout << endl;
}
int main()
{
    int i = 707;
    int j[3] = {7, 0, 7};
    print(i);
    print(j, 3);
    return 0;
}
