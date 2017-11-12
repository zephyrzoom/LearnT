#coding=utf-8
#author=zeek

from time import time
import datetime
import logging
import os
import random
import pickle

def douzhang(dirpwd='.'):
    files = os.listdir(dirpwd)
    extend = []
    for i in files:
        if os.path.isfile(i):
            extend.append(i.split('.')[1])
    extend = set(extend)
    extend = list(extend)
    return extend

def xulie(dirname, info):
    file_name = random.randint(100,999)
    file_path = dirname+str(file_name)
    with open(file_path, 'wb') as f:
        pickle.dump(info,f)


if __name__ == '__main__':
    print time()
    print datetime.date.today()
    print datetime.timedelta(days=30)
    last_month = datetime.date.today() - datetime.timedelta(days=30)
    print last_month
#    os.system('ping www.baidu.com')
    print douzhang()
    xulie('./',time())
