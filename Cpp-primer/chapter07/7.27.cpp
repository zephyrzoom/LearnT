/*
 * @author 707<707472783@qq.com>
 * This header defines class Screen.
 */
#include <iostream>
#include "7.24.h"
using std::cout;
int main()
{
    Screen myScreen(5, 5, 'X');
    myScreen.move(4, 0).set('#').display(cout);
    cout << "\n";
    myScreen.display(cout);
    cout << "\n";
    return 0;
}
