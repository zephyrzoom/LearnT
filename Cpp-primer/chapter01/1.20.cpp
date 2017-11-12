/*
 * @author 707<707472783@qq.com>
 * This program read some mark of book saling and print to standard output.
 */

#include <iostream>
#include "Sales_item.h"
int main()
{
    Sales_item book;
    while (std::cin >> book)
    {
        std::cout << book << std::endl;
    }
    return 0;
}
