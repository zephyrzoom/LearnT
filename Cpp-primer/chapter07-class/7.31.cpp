/*
 * @author 707<707472783@qq.com>
 * This program define class X and class Y.
 */
class Y;
class X {
    private:
        Y *y;
};

class Y {
    private:
        X x;
};
int main()
{
    return 0;
}
