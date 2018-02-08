#coding=utf-8
"""
1 已知字符串:
info = '<a href="http://www.baidu.com">baidu</a>'

用正则模块提取出网址："http://www.baidu.com"和链接文本:"baidu"

2 字符串："one1two2three3four4" 用正则处理，输出 "1234"

3 已知字符串：text = "JGood is a handsome boy, he is cool, clever, and so on..." 查找所有包含'oo'的单词。
"""

import re

info = '<a href="http://www.baidu.com">baidu</a>'
m = re.match(r'.+"(.+?)">(.+)<.+', info)

print m.group(1)
print m.group(2)

s = "one1two2three3four4"
print ''.join(re.split(r'[a-z]+', s))

text = "JGood is a handsome boy, he is cool, clever, and so on..."
m = re.findall(r'(\w*oo\w*)[\s,]', text)
print m

