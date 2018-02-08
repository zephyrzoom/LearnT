#coding=utf-8
#author=zeek

import os
import logging
import urllib

class Fileinfo:
    def __init__(self, filename):
        self.filename = filename

    def __enter__(self):
        self.f = open(self.filename, 'r')
        try:
            c = self.f.read()
        except Exception, e:
            print 'open error'
        else:
            return c
        finally:
            self.f.close()

    def __exit__(self, exc_type, exc_value, exc_tb):

        logger = logging.getLogger()
        hdlr = logging.FileHandler('pyhw12.log')
        formatter = logging.Formatter('%(asctime)s %(message)s')
        hdlr.setFormatter(formatter)
        logger.addHandler(hdlr)
        logger.setLevel(logging.NOTSET)
        logger.debug(self.f.name)

def get_page(listindex):
    info = ['http://www.qq.com', 'http://www.baidu.com', 'www.taobao.com']
    r = urllib.urlopen(info[listindex])
    try:
        c = r.read()
    except Exception, e:
        logger = logging.getLogger()
        hdlr = logging.FileHandler('pyhw12_2.log')
        formatter = logging.Formatter('%(asctime)s-%(message)s')
        hdlr.setFormatter(formatter)
        logger.addHandler(hdlr)
        logger.setLevel(logging.NOTSET)
        logger.debug(e)
    else:
        return c
    finally:
        r.close()

def get_urlcontent(url):


if __name__ == '__main__':
    with Fileinfo('554') as f:
        print f
    print get_page(2)
