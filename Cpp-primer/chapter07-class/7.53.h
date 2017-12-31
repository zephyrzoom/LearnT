/*
 * @author 707<707472783@qq.com>
 * This header defines Debug class
 * with constexpr.
 */
class Debug {
    public:
        constexpr Debug(bool s, bool i, bool o):
            syn(s), io(i), other(o) {}
        constexpr Debug(bool b = true):
            syn(b), io(b), other(b) {}
        
    private:
        bool syn;
        bool io;
        bool other;
};
