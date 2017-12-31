/*
 * @author 707<707472783@qq.com>
 * This program declaration a function
 * and a vector pointing to the function.
 */
#include <vector>
using std::vector;
int f(int, int);
vector<int (*)(int, int)> vf;
