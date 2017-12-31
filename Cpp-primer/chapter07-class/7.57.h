/*
 * @author 707<707472783@qq.com>
 * This header defines Account class.
 */
#include <iostream>
using std::string;
class Account {
    private:
        double amount;
        string owner;
        string pwd;
        static double rate;
        static constexpr int period = 12;

    public:
        static double get_rate() { return rate; }
        void add_money() { amount += amount * rate; }
        static void set_rate(double r);
};

void Account set_rate(double r)
{
    rate = r;
}
