/*
 * @author 707<707472783@qq.com>
 * This program define the class Sales_data.
 */
#include <iostream>
using std::string;
using std::istream;
using std::ostream;
struct Sales_data;
istream &read(istream&, Sales_data&);
struct Sales_data {
    string isbn;
    double price = 0;
    double sum_price = 0;
    int amount = 0;
    Sales_data& combine(const Sales_data &rhs);
    string get_isbn() const;
    Sales_data() = default;
    Sales_data(const string &s): isbn(s) { }
    Sales_data(const string &s, double p, int a):
        isbn(s), price(p), amount(a), sum_price(p*a) { }
    Sales_data(istream &is)
    {
        read(is, *this);
    }
};

Sales_data& Sales_data::combine(const Sales_data &rhs)
{
    sum_price += rhs.sum_price;
    amount += rhs.amount;
    return *this;
}

string Sales_data::get_isbn() const
{
    return isbn;
}

Sales_data add(const Sales_data &l, const Sales_data &r)
{
    Sales_data sum = l;
    sum.combine(r);
    return sum;
}

istream &read(istream &is, Sales_data &item)
{
    is >> item.isbn >> item.price >> item.amount;
    item.sum_price = item.price * item.amount;
    return is;
}

ostream &print(ostream &os, const Sales_data &item)
{
    os << item.get_isbn() << " " << item.price << " "
        << item.sum_price << " " << item.amount;
    return os;
}
