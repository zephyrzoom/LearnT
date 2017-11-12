/*
 * @author 707<707472783@qq.com>
 * This program creates the Sales_data struct opnion my view.
 */
#include <iostream>
#include "7.12.h"
using std::cin;
using std::cout;
using std::endl;
using std::cerr;
using std::istream;
int main()
{
    Sales_data total(cin);
    if (!total.get_isbn().empty()) {
        istream &is = cin;
        while (is) {
            Sales_data trans(is);
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
