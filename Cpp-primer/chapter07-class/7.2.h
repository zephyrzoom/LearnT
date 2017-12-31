/*
 * @author 707<707472783@qq.com>
 * This program define the class Sales_data.
 */
#include <iostream>
using std::string;
struct Sales_data {
    string isbn;
    double price = 0;
    double sum_price = 0;
    string name;
    int amount = 0;
    Sales_data& combine(const Sales_data &rhs);
    string get_isbn() const;
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
