/*
 * @author 707<707472783@qq.com>
 * 这个程序读取字符后输出到标准流。
 */
#include "8.1.h"
#include <iostream>
#include <string>
using std::istream;
using std::cout;
using std::string;
istream& readFromStream(istream& is)
{
    // ctrl+D触发eof
    while (!is.eof()) {
        string word;
        is >> word;
        cout << word;
    }
    is.clear();
    return is;
}
