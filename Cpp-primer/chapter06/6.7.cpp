/*
 * @author 707<707472783@qq.com>
 * This program returns 0 when first call
 * and after returns 1 more than former call.
 */
#include <iostream>
using std::cout;
using std::endl;
int more()
{
    static int origin = 0;
    return origin++;
}
int main()
{
    for (size_t i = 0; i != 10; i++) {
        cout << more() << endl;;
    }
    return 0;
}
