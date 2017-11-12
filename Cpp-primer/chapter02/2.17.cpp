/*
 * @author 707<707472783@qq.com>
 * This program tests reference.
 */
#include <iostream>
int main()
{
    int i = 0, &r1 = i;
    i = 5;
    r1 = 10;
    std::cout << i << " " << r1 << std::endl;
    return 0;
}
