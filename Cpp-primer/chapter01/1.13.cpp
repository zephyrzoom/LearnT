/*
 * @author 707<707472783@qq.com>
 * This program is rewrite the 1.9~1.11 with for statement.
 */

#include <iostream>
int main()
{
    /* 1.9 */
    int sum = 0;
    for (int val = 50; val <= 100; val++)
    {
        sum += val;
    }
    std::cout << "sum from 50 to 100 is " << sum << std::endl;

    /* 1.10 */
    for (int i = 10; i >= 0; i--)
    {
        std::cout << i << std::endl;
    }

    /* 1.11 */
    int little = 0;
    int big = 0;
    std::cout << "Enter a little number: ";
    std::cin >> little;
    std::cout << "Enter a big number: ";
    std::cin >> big;
    for (; little <= big; little++)
    {
        std::cout << little << " ";
    }
    std::cout << std::endl;
}
