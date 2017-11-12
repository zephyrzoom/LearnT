/*
 * @author 707<707472783@qq.com>
 * This program defines the class book.
 */
class Book {
    string name;
    int page;
    double price;
    Book(string n, int p, double pr):
        name(n), page(p), price(pr) {}
    Book(string n):
        Book(n, 0, 0) {}
    Book(string n, int p):
        Book(n, p, 0) {}
    Book(string n, int pr):
        Book(n, 0, pr) {}
};
