/*
 * @author 707<707472783@qq.com>
 * This program is for test the content of the value 
 * that before assignment and after assignment.
 */

#include <iostream>
int main()
{
    int i = 0, &r = i;
    auto a = r;
    const int ci = i, &cr = ci;
    auto b = ci;
    auto c = cr;
    auto d = &i;
    auto e = &ci;
    auto &g = ci;
    std::cout << "before:" << a << " " 
                           << b << " " 
                           << c << " " 
                           << d << " " 
                           << e << " " 
                           << g <<std::endl;
    a = 42;
    b = 42;
    c = 42;
    // d is int*
    //d = 42;
    // e is const int*
    //e = 42;
    // g is const
    //g = 42;
    std::cout << "after:" << a << " " 
                          << b << " " 
                          << c << " " 
                          << d << " " 
                          << e << " " 
                          << g <<std::endl;

    return 0;
}
