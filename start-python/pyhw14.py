#coding=utf-8

import threading

mlock = threading.Lock()
count = 500

def withdraw():
    global count
    for i in xrange(0,100):
        mlock.acquire()
        count+=1
        mlock.release()
        print count

for i in xrange(0,10):
    t = threading.Thread(target=withdraw)
    t.start()
