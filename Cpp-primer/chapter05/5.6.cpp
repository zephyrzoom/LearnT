/*
 * @author 707<707472783@qq.com>
 * This program is translate number to alphabeta by ? ; statement.
 */
#include <iostream>
#include <vector>
using std::vector;
using std::string;
int main()
{
    int grade;
    const vector<string> scores = {"F", "D", "C", "B", "A", "A++"};
    string lettergrade;
    grade < 60 ? lettergrade = scores[0]
        : grade < 70 ? lettergrade = scores[1]
        : grade < 80 ? lettergrade = scores[2]
        : grade < 90 ? lettergrade = scores[3]
        : grade < 100 ? lettergrade = scores[4]
        : lettergrade = scores[5];
    return 0;
}
