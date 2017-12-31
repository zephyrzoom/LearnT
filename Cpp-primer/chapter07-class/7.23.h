/*
 * @author 707<707472783@qq.com>
 * This header defines class Screen.
 */
#include <iostream>
using string;
class Screen {
    private:
        int x = 0;
        int y = 0;
        int curser_x = 0;
        int curser_y = 0;
        string content;

    public:
        Screen() = default;
        Screen(int xx, int yy, string ct):
            x(xx), y(yy), content(ct) {}
        char get() const { return content[curser_y * x + curser_x]; }
        char get(int cx, int cy) const;
        Screen &move(int cx, int cy);
};

inline char Screen::get(int cx, int cy) const
{
    return content[cy * x + cx];
}

inline Screen &Screen::move(int cx, int cy)
{
    curser_x = cx;
    curser_y = cy;
    return *this;
}
