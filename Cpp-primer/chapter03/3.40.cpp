/*
 * @author 707<707472783@qq.com>
 * This program define two char arrays
 * then another array store the joined array.
 */
#include <iostream>
#include <cstring>
using std::cout;
using std::endl;
int main()
{
    char c1[] = "hello ";
    char c2[] = "world";
    char c3[12];
    strcpy(c3, strcat(c1, c2));
    cout << c3 << endl;
    return 0;
}
