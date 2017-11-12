/*
 * @author 707<707472783@qq.com>
 * This program changes pointer and the object of prointer pointed.
 */
#include <iostream>
int main()
{
    int val = 1;
    int *p = &val;
    std::cout << p << " " << *p << std::endl;
    *p = 2;
    p++;
    std::cout << p << " " << val << std::endl;
    return 0;
}
