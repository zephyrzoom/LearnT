#coding=utf-8
#author=zeek


import os

def func_file(filename):
    if os.path.isdir(filename):
        return 'not a file'
    if os.path.isfile(filename):
        f = open(filename, 'r')
        try:
            c = f.read()
        except IOError:
            print 'io error'
        else:
            return c
        finally:
            f.close()
    else:
        return 'not exist the file'

import urllib
import logging

def func_url(urllist):
    for url in urllist:
        try:
            print urllib.urlopen(url).read()
        except IOError:
            logging.basicConfig(level=logging.DEBUG,
                format='%(asctime)s %(filename)s[line:%(lineno)d] %(levelname)s %(message)s',
                datefmt='%a, %d %b %Y %H:%M:%S',
                filename='pyhw10.log',
                filemode='w')
            logging.info('io error')

def func_domain(domainlist):
    for url in domainlist:
        try:
            print os.system('ping %s' % url)
        except IOError:
            print 'io error'

if __name__ == '__main__':
    # how to make f access in finally
    print func_file('except_file')
    func_url(['http://www.baidu','http://www.qq.com','http://www.taobao.com'])
    func_domain(['www.baidu.com'])
