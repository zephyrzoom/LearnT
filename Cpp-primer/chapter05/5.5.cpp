/*
 * @author 707<707472783@qq.com>
 * This program translate number to alphabeta.
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
    if (grade < 60) {
        lettergrade = scores[0];
    } else if (grade < 70) {
        lettergrade = scores[1];
    } else if (grade < 80) {
        lettergrade = scores[2];
    } else if (grade < 90) {
        lettergrade = scores[3];
    } else if (grade < 100) {
        lettergrade = scores[4];
    } else {
        lettergrade = scores[5];
    }
    return 0;
}
