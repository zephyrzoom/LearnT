/*
 * @author 707<707472783@qq.com>
 * This program exchange two pointers.
 */
#include <iostream>
using std::cout;
using std::cin;
using std::endl;
void exchange(int *&pa, int *&pb)
{
    int *pc = pa;
    pa = pb;
    pb = pc;
}
int main()
{
    int a;
    int b;
    cout << "enter two int:" << endl;
    cin >> a >> b;
    int *pa = &a;
    int *pb = &b;
    exchange(pa, pb);
    cout << *pa << *pb << endl;
    return 0;
}
