/*
 * @author 707<707472783@qq.com>
 * This program outputs the arguments.
 */
#include <iostream>
using std::endl;
using std::cout;
int main(int argc, char **argv)
{
    for(size_t i = 0; i < argc; i++) {
        cout << argv[i] << endl;
    }
    return 0;
}
