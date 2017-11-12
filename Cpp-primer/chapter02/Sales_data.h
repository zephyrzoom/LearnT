#ifndef SALES_DATA_H
#define SALES_DATA_H
#include <string>
struct Sales_data {
    std::string isbn;
    double price = 0;
    double sum_price = 0;
    std::string name;
    int amount = 0;
};
#endif
