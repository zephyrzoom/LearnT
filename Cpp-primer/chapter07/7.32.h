/*
 * @author 707<707472783@qq.com>
 * This header defines class Screen
 * and Window_mgr.
 */
#include <iostream>
#include <vector>
using std::vector;
using std::string;
using std::ostream;
class Screen {
    private:
        int x = 0;
        int y = 0;
        int curser_x = 0;
        int curser_y = 0;
        string content;
        void do_display(ostream &os) const {os << content;}

    public:
        Screen() = default;
        Screen(int xx, int yy):
            x(xx), y(yy), content(" ") {}
        Screen(int xx, int yy, char ct):
            x(xx), y(yy), content(yy * x + xx, ct) {}
        char get() const { return content[curser_y * x + curser_x]; }
        char get(int cx, int cy) const;
        Screen &move(int cx, int cy);
        Screen &set(char c);
        Screen &display(ostream &os)
        { do_display(os); return *this; }
        const Screen &display(ostream &os) const
        { do_display(os); return *this; }

    friend void Window_mgr::clear(Index);
};

class Window_mgr {
    public:
        using Index = vector<Screen>::size_type;
        void clear(Index);

    private:
        vector<Screen> screens {Screen(70, 7, ' ')};
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

inline Screen &Screen::set(char c)
{
    content[curser_y * x + curser_x] = c;
    return *this;
}

void Window_mgr::clear(Index i)
{
    Screen &s = screens[i];
    s.content = string(s.x * s.y, ' ');
}
