/*
 * @author 707<707472783@qq.com>
 * This program recursively ouput vector.
 */
#include <iostream>
#include <vector>
#include <cassert>
using std::vector;
using std::cout;
using std::endl;
using std::cerr;
void print_vector(const vector<int> &v, vector<int>::iterator beg, vector<int>::iterator end)
{
#ifndef NDEBUG
    assert(*beg < 3);
    cerr << "Eroor: " << __FILE__
        << " : in function " << __func__
        << " at line " << __LINE__ << endl
        << "    Compile on " << __DATE__
        << " at " << __TIME__ << endl;
#endif
    if (beg == end) {
        return;
    } 
    cout << *beg++ << endl;
    print_vector(v, beg, end);
}
int main()
{
    vector<int> v = {1, 2, 3, 4, 5};
    print_vector(v, v.begin(), v.end());
    return 0;
}
