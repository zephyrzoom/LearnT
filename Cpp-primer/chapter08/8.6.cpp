/*
 * @author 707<707472783@qq.com>
 * 这个程序修改7.1，改为从文件读写
 */
 #include <iostream>
 #include <fstream>
 using std::cin;
 using std::cout;
 using std::endl;
 using std::cerr;
 struct Sales_data {
     std::string isbn;
     double price = 0;
     std::string name;
     int amount = 0;
     std::string order_no;
 };

 int main()
 {
     Sales_data total;
     std::ifstream fin;
     std::ofstream fout;
     if (read(fin, total)) {
         Sales_data trans;
         while (read(fin, trans)) {
             if (total.isbn() == trans.isbn())
                 total.combine(trans);
             else {
                 print(fout, total) << endl;
                 total = trans;
             }
         }
         print(fout, total) << endl;
     } else {
         cerr << "No data?!" << endl;
     }
     return 0;
 }
