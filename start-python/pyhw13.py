#coding=utf-8

import threading
import urllib

info = [1,2,3,4,55,233]

def get_info(i):
    print i

ts = []

for i in info:
    t = threading.Thread(target=get_info,args=[i])
    ts.append(t)

for i in ts:
    i.start()

for i in ts:
    i.join()

print 'the end'


urlinfo = ['http://www.sohu.com','http://www.163.com','http://www.sina.com']

def get_url(url):
    f = urllib.urlopen(url)
    print f.getcode()
    print f.read()

ts = []
for i in urlinfo:
    t = threading.Thread(target=get_url,args=[i])
    ts.append(t)

for i in ts:
    i.start()

for i in ts:
    i.join()
