/*
 * @author 707<707472783@qq.com>
 * This program read two books with the same ISBN, then output their sum.
 */

#include <iostream>
#include "Sales_item.h"
int main()
{
    Sales_item book1;
    Sales_item book2;
    std::cout << "Book one:";
    std::cin >> book1;
    std::cout << "Book two:";
    std::cin >> book2;
    std::cout << book1 + book2 << std::endl;
    return 0;
}
