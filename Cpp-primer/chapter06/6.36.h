/*
 * @author 707<707472783@qq.com>
 * This program is a function declaration.
 */
#include <iostream>
using std::string;
string (&strArr getArr())[10];

using strArr = string[10];
strArr &getArr();

auto getArr() -> string(&)[10];

string strArr[] = {"1", "2", "3"};
decltype(strArr) &getArr();
