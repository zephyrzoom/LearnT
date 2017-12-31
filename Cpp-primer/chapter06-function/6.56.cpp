/*
 * @author 707<707472783@qq.com>
 * This program declaration 4 functions
 * and a vector pointing to the functions
 * then output the result.
 */
#include <iostream>
#include <vector>
using std::vector;
using std::cout;
using std::endl;
int f(int, int);
vector<decltype(f)*> vf;

inline int add(int a, int b)
{
    return a + b;
}
inline int sub(int a, int b)
{
    return a - b;
}
inline int mul(int a, int b)
{
    return a * b;
}
inline int divi(int a, int b)
{
    return a / b;
}

int main()
{
    vf.push_back(add);
    vf.push_back(sub);
    vf.push_back(mul);
    vf.push_back(divi);
    
    for (auto i : vf) {
        cout << i(8, 4) << endl;
    }
    return 0;
}
