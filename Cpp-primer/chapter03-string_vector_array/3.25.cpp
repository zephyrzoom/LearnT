/*
 * @author 707<707472783@qq.com>
 * This program is for dividing the grade
 * to 11 scores.
 */
#include <iostream>
#include <vector>
using std::cout;
using std::endl;
using std::cin;
using std::vector;
int main()
{
    vector<unsigned> scores(11, 0);
    unsigned grade;
    while (cin >> grade) {
        if (grade <= 100) {
            auto it = scores.begin();
            (*(it + grade/10))++;
        }
    }
    for (auto it = scores.cbegin(); it != scores.cend(); it++) {
        cout << *it << " ";
    }
    cout << endl;
    return 0;
}
