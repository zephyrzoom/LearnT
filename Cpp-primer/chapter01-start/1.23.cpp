/*
 * @author 707<707472783@qq.com>
 * This program reads several sale orders and calculate the same ISBN orders.
 * Assume there are 3 kinds of books.
 */

#include <iostream>
#include "Sales_item.h"
int main()
{
    Sales_item book;
    Sales_item kind1;
    Sales_item kind2;
    Sales_item kind3;
    // empty of the kinds.
    int empty = 3;
    std::cout << "Enter book info:" << std::endl;
    while (std::cin >> book)
    {
        std::cout << "Enter book info:" << std::endl;
        if (book.isbn() == kind1.isbn()) {
            kind1 = book + kind1;
        } else if (book.isbn() == kind2.isbn()) {
            kind2 = book + kind2;
        } else if (book.isbn() == kind3.isbn()) {
            kind3 = book + kind3;
        } else {
            if (empty == 3) {
                kind1 = book;
                empty = 2;
            } else if (empty == 2) {
                kind2 = book;
                empty = 1;
            } else {
                kind3 = book;
            }
        }

    }
    std::cout << kind1 << std::endl
              << kind2 << std::endl
              << kind3 << std::endl;
    return 0;
}
