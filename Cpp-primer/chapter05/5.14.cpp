/*
 * @author 707<707472783@qq.com>
 * This program count the continous same word
 * from the input string.
 */
#include <iostream>
#include <vector>
using std::cout;
using std::cin;
using std::endl;
using std::string;
using std::vector;
int main()
{
    string word;
    vector<string> words;
    vector<int> counts;
    while (cin >> word) {
        if (words.empty()) {
            words.push_back(word);
            counts.push_back(1);
        } else {
            if (word == words[counts.size()-1]) {
                ++counts[counts.size()-1];
            } else {
                words.push_back(word);
                counts.push_back(1);
            }
        }
    }
    
    for (auto i = 0, j = 0; i != words.size() && j != counts.size(); i++, j++) {
        cout << words[i] << " " << counts[i] << endl;
    }
    return 0;
}
