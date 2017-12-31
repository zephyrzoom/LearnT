/*
 * @author 707<707472783@qq.com>
 * This program is let user input two numbers and print the numbers between them.
 */

#include <iostream>
int main()
{
    int little = 0;
    int big = 0;
    std::cout << "Enter a little number: ";
    std::cin >> little;
    std::cout << "Enter a big number: ";
    std::cin >> big;
    while (little <= big)
    {
        std::cout << little << " ";
        little++;
    }
    std::cout << std::endl;
    return 0;
}
