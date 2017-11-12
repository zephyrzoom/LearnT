/*
 * @author 707<707472783@qq.com>
 * This program uses parameters, local valuable
 * and static value.
 */
int foo(int a)
{
    int b = a;
    static int c = ++b;
    return c;
}
int main()
{
    int r = foo(2);
    return 0;
}
