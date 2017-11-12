/*
 * @author 707<707472783@qq.com>
 * This program read some book with the same ISBN, then output their sum.
 */

#include <iostream>
#include "Sales_item.h"
int main()
{
    Sales_item book;
    Sales_item sum;
    std::cout << "Book info:";
    while (std::cin >> book)
    {
        std::cout << "Book info:";
        sum = book + sum;
    }
    std::cout << "sum of all the books is: " << sum << std::endl;
    return 0;
}
