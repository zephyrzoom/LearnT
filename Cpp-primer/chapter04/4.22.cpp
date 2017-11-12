/*
 * @author 707<707472783@qq.com>
 * This program divide the score.
 */
#include <iostream>
using std::string;
int main()
{
    int grade;
    string finalgrade;
    finalgrade = (grade > 90) ? "high pass"
        : (grade < 60) ? "fail"
        : (grade < 75) ? "low pass" 
        : "pass";

    if (grade > 90) {
        finalgrade = "high pass";
    } else if (grade < 60) {
        finalgrade = "fail";
    } else if (grade < 75) {
        finalgrade = "low pass";
    } else {
        finalgrade = "pass";
    }

    return 0;
}
