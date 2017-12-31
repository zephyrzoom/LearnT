/*
 * @author 707<707472783@qq.com>
 * This program define a persion class with explicit.
 */
#include <iostream>
using std::string;
using std::istream;
using std::ostream;
class Person {
    private:
        string name;
        string address;
    public:
        string get_name() const;
        string get_address() const;
        Person() = default;
        Person(string n, address a):
            name(n), address(a) {}
        explicit Person(string n): Person(n, " ") {}
};

string Person::get_name() const
{
    return name;
}

string Person::get_address() const
{
    return address;
}

istream &read(istream &is, Person &p)
{
    is >> p.name >> p.address;
    return is;
}

ostream &print(ostream &os, const Person &p)
{
    os << p.get_name() << " " << p.get_address();
    return os;
}
