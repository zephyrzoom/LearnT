/*
 * @author 707<707472783@qq.com>
 * This main cats two string and outputs.
 */
#include <iostream>
using std::cout;
using std::string;
using std::endl;
int main(int argc, char **argv)
{
    string s = argv[1];
    s = s + argv[2];
    cout << s << endl;
    return 0;
}
