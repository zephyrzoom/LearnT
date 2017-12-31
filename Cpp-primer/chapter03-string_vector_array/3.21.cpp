/*
 * @author 707<707472783@qq.com>
 * This program is print the size of vector and it's content.
 */
#include <iostream>
#include <vector>
using std::cout;
using std::endl;
using std::vector;
using std::string;
int main()
{
    vector<int> v1;
    vector<int> v2(10);
    vector<int> v3(10, 42);
    vector<int> v4{10};
    vector<int> v5{10, 42};
    vector<string> v6{10};
    vector<string> v7{10, "hi"};

    cout << "v1:" << endl;
    cout << v1.size() << endl;
    for (auto it = v1.cbegin(); it != v1.cend(); it++) {
        cout << *it << endl;
    }

    cout << "v2:" << endl;
    cout << v2.size() << endl;
    for (auto it = v2.cbegin(); it != v2.cend(); it++) {
        cout << *it << endl;
    }

    cout << "v3:" << endl;
    cout << v3.size() << endl;
    for (auto it = v3.cbegin(); it != v3.cend(); it++) {
        cout << *it << endl;
    }

    cout << "v4:" << endl;
    cout << v4.size() << endl;
    for (auto it = v4.cbegin(); it != v4.cend(); it++) {
        cout << *it << endl;
    }

    cout << "v5:" << endl;
    cout << v5.size() << endl;
    for (auto it = v5.cbegin(); it != v5.cend(); it++) {
        cout << *it << endl;
    }

    cout << "v6:" << endl;
    cout << v6.size() << endl;
    for (auto it = v6.cbegin(); it != v6.cend(); it++) {
        cout << *it << endl;
    }

    cout << "v7:" << endl;
    cout << v7.size() << endl;
    for (auto it = v7.cbegin(); it != v7.cend(); it++) {
        cout << *it << endl;
    }
    return 0;
}
