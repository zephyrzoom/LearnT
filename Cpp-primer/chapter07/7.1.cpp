/*
 * @author 707<707472783@qq.com>
 * This program creates the Sales_data struct opnion my view.
 */
#include <iostream>
using std::cin;
using std::cout;
using std::endl;
struct Sales_data {
    std::string isbn;
    double price = 0;
    std::string name;
    int amount = 0;
    std::string order_no;
};

int main()
{
    Sales_data total;
    if (read(cin, total)) {
        Sales_data trans;
        while (read(cin, trans)) {
            if (total.isbn() == trans.isbn())
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
