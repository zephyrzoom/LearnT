#coding=utf-8

"""
已知字符串：

info = 'test,&nbsp;url("http://www.baidu.com")&,dddddd "="" <svg></svg><path></path><img src="http://www.baidu.com">ininnnin<img src="http://www.dd.com">'

要求完成下面2个小功能：
1.1 关闭[img]标签
1.2 将url()中的["]转为[']

最后结果字符串：

"test,&nbsp;url('http://www.baidu.com')&,dddddd "="" <svg></svg><path></path><img src="http://www.baidu.com"></img>ininnnin<img src="http://www.dd.com"></img>"
"""

import re

info = 'test,&nbsp;url("http://www.baidu.com")&,dddddd "="" <svg></svg><path></path><img src="http://www.baidu.com">ininnnin<img src="http://www.dd.com">'
print re.findall(r'<img.*>()', info)
print re.sub(r'<img.*>()', '</img>', info)

