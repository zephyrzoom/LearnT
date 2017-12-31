/*
 * @author 707<707472783@qq.com>
 * 这个程序从文件按行读取存入vector。
 */
#include <fstream>
#include <string>
#include <vector>
#include <iostream>
std::vector<std::string> storeToVector()
{
    std::string fileName = "8.4.cpp";
    std::ifstream in(fileName);
    std::vector<std::string> content;
    if (in) {
        std::string line;
        while (getline(in, line)) {
            content.push_back(line);
        }
    }
    return content;
}

int main(int argc, char const *argv[]) {
    std::vector<std::string> v = storeToVector();
    for (auto i : v) {
        std::cout << i << std::endl;
    }
    return 0;
}
