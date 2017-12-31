/*
 * @author 707<707472783@qq.com>
 * This program is reading a array and output sum of them.
 */

#include <iostream>
int main() 
{
    int val = 0;
    int sum = 0;
    while (std::cin >> val)
    {
        sum += val; 
    }
    std::cout << "Sum of the input is " << sum << std::endl;
}
