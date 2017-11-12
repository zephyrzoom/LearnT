/*
 * @author 707<707472783@qq.com>
 * This program rewrite the former execises.
 */

#include <iostream>
using std::cout;
using std::cin;
using std::endl;
using std::string;
struct Sales_data {
    string isbn;
    double price = 0;
    double sum_price = 0;
    string name;
    int amount = 0;
};

int main()
{
    // 1.21
    Sales_data book1;
    Sales_data book2;
    cout << "book1:" << endl;
    cin >> book1.isbn >> book1.price >> book1.name >> book1.amount;
    cout << "book2:" << endl;
    cin >> book2.isbn >> book2.price >> book2.name >> book2.amount;
    cout << "sum:" << book1.isbn << " "
                        << book1.price * book1.amount + book2.price * book2.amount << " "
                        << book1.name << " "
                        << book1.amount + book2.amount << endl;

    // 1.22
    Sales_data book;
    Sales_data sum;
    cout << "book:" << endl;
    while (cin >> book.isbn >> book.price >> book.name >> book.amount)
    {
        sum.isbn = book.isbn;
        sum.sum_price += book.price * book.amount;
        sum.name = book.name;
        sum.amount += book.amount;
        cout << "book:" << endl;
    }

    cout << "sum:" << sum.isbn << " "
                        << sum.sum_price << " "
                        << sum.name << " "
                        << sum.amount << endl;

    // 1.24
    Sales_data kind1;
    Sales_data kind2;
    Sales_data kind3;
    int empty = 3;
    cout << "book:" << endl;
    while (cin >> book.isbn >> book.price >> book.name >> book.amount)
    {
        if (book.isbn == kind1.isbn) {
            kind1.sum_price += book.price * book.amount;
            kind1.amount += book.amount;
        } else if (book.isbn == kind2.isbn) {
            kind2.sum_price += book.price * book.amount;
            kind2.amount += book.amount;
        } else if (book.isbn == kind3.isbn) {
            kind3.sum_price += book.price * book.amount;
            kind3.amount += book.amount;
        } else {
            if (empty == 3) {
                kind1.isbn = book.isbn;
                kind1.name = book.name;
                kind1.sum_price = book.price * book.amount;
                kind1.amount = book.amount;
            } else if (empty == 2) {
                kind2.isbn = book.isbn;
                kind2.name = book.name;
                kind2.sum_price = book.price * book.amount;
                kind2.amount = book.amount;
            } else if (empty == 1) {
                kind3.isbn = book.isbn;
                kind3.name = book.name;
                kind3.sum_price = book.price * book.amount;
                kind3.amount = book.amount;
            } else {
                cout << "wrong book isbn" << endl; 
            }
        }
    }
    cout << "kind1:" << kind1.isbn << " "
                     << kind1.sum_price << " "
                     << kind1.name << " "
                     << kind1.amount << endl;
    cout << "kind2:" << kind2.isbn << " "
                     << kind2.sum_price << " "
                     << kind2.name << " "
                     << kind2.amount << endl;
    cout << "kind3:" << kind3.isbn << " "
                     << kind3.sum_price << " "
                     << kind3.name << " "
                     << kind3.amount << endl;

    return 0;
}
