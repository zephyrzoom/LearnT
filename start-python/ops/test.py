print([] or None and {})
print('ew')
a=5
b=5
print(a is b)
a=[1,2,3,4]
b=a
print(a is b)
b[0]=0
print(a is b)
print(a)
c=[5,6,7,8]
d=[5,6,7,8]
print(c is d)

s1='boc'
s2='boc'
print(s1 is s2)
c = a.copy()
print(a is c)

d={1:2}
dc=d.copy()
print(dc is d)

s='345345'
print(s[6:0:1])

print(type(range(1,5)))

from psutil import *
print(cpu_times())
print(cpu_count())
print(cpu_count(logical=False))
print(virtual_memory())

import flask