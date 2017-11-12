/*
 * @author 707<707472783@qq.com>
 * This program defines NoDefault class
 * and C class.
 */
class NoDefault {
    public:
        NoDefault(int n) {}
};

class C {
    private:
        NoDefault nd;
    public:
        C(): nd(707) {}
};
