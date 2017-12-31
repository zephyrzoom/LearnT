/*
 * @author 707<707472783@qq.com>
 * This program declaration 4 functions
 * and a vector pointing to the functions.
 */
#include <vector>
using std::vector;
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
inline int div(int a, int b)
{
    return a / b;
}

int main()
{
    vf.push_back(add);
    vf.push_back(sub);
    vf.push_back(mul);
    vf.push_back(div);
    return 0;
}
