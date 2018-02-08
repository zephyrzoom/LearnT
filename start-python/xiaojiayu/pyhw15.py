#coding=utf-8

def test():
    x = yield
    y = yield 'x=%s'%x
    z = yield 'y=%s'%y

t = test()
# print t.next()
# print t.next()
# print t.next()
print t.next()
print t.send('haha1')
print t.send('haha2')


def fibnacci(n):
    x = 1
    y = 1
    yield x
    yield y
    z = x + y
    while z <= n:
        yield z
        x = y
        y = z
        z = x + y

for i in fibnacci(13):
    print i
