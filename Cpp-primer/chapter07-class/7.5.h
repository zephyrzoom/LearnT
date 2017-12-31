/*
 * @author 707<707472783@qq.com>
 * This program define a persion class.
 */
#include <iostream>
using std::string;
struct Person {
    string name;
    string address;
    string get_name() const;
    string get_address() const;
};

string Person::get_name() const
{
    return name;
}

string Person::get_address() const
{
    return address;
}
