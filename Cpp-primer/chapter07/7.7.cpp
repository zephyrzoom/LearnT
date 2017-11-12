/*
 * @author 707<707472783@qq.com>
 * This program creates the Sales_data struct opnion my view.
 */
#include <iostream>
#include "7.6.h"
using std::cin;
using std::cout;
using std::endl;
using std::cerr;
int main()
{
    Sales_data total;
    if (read(cin, total)) {
        Sales_data trans;
        while (read(cin, trans)) {
            if (total.get_isbn() == trans.get_isbn())
                total.combine(trans);
            else {
                print(cout, total) << endl;
                total = trans;
            }
        }
        print(cout, total) << endl;
    } else {
        cerr << "No data?!" << endl;
    }
    return 0;
}
