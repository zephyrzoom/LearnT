/*
 * @author 707<707472783@qq.com>
 * This program has a list type
 * and the function can sum
 * the amount of parameters.
 */
#include <iostream>
using std::cout;
using std::endl;
using std::initializer_list;
int sum(initializer_list<int> list)
{
    int sum = 0;
    for (const auto &i : list) {
        sum += i;
    }
    return sum;
}
int main()
{
    cout << sum({1, 2, 3, 4, 5}) << endl;
    return 0;
}
