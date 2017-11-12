#coding=utf-8

import sys

###############exe1.1
info = 'abc'
#info[2] = 'd' # error

print info.replace('c', 'd')

a = '1'
b = 2
#print a + b #error
print int(a) + b

################exe1.2
a = 'abcd'
print a[3]
print a[-1]

a = 'jay'
b = 'python'
print 'my name is %s, i love %s.' % (a, b)

##################exe1.3
print 'my name is %(a)s, i love %(b)s.' % {'a':'pyer', 'b':'apple'}
print 'my name is {a}, i love {b}.'.format(b = 'apple', a = 'pyer')

# write to file with 500 numbers
f = open('info.txt', 'w')
writer = []
for i in range(500):
    writer.append(str(i))
writer = ' '.join(writer)
f.write(writer)
f.close()

#################exe8
s = 'i,am,lilei'
print s[2:4]
print s[-8:-6]
print s.split(',')[1]

print bool('2012' == 2012)

f = open('test.txt', 'r')
message = f.read()
print message
print len(message.decode('utf-8'))
print message.replace('\n', '')
w = message.replace('2012', '2013')
print w
print message[len(message)/2-3:len(message)/2+2]
print message[-2:]
message = message[:11]
print message
f.close()
f = open('test.txt', 'w')
f.write(w)
f.close()

a = 'zhongwenbiancheng'
b = a
c = a
a = 'pythonbiancheng'
b = u'%s' %a
d = 'zhongwenbiancheng'
e = a
c = b
b2 = a.replace('zhong', 'zhong')
print sys.getrefcount('zhongwenbiancheng')
print sys.getrefcount('pythonbiancheng')

a = 'zifupinjie1'
b = 'zifupinjie2'
print ''.join([a, b])
print '%s%s' % (a, b)
print '{0}{1}'.format(a, b)
print '%(a)s%(b)s' % {'a':a, 'b':b}
c = ','.join([a, b])
print c
print len(c)
print c[6]

import string
print string.digits
print string.ascii_lowercase
print string.punctuation
print string.letters
print ''.join([string.digits, string.ascii_lowercase, string.punctuation, string.letters])

a = 'i,am,a,boy,in,china'
print a[7:10], a[14:]
s = a.split(',')
print s[3], s[-1]
print a.find('i')
print a.find('i', a.find('china'))
print a.count(',')

__console__ = sys.stdout
f = open('string.txt', 'w')
sys.stdout = f
help(string)
f.close()
sys.stdout = __console__

###################################exe9
a = [1,2,3,4,5,333,11,44]
print a[3:6]

a = [1,2,3]
b = [4,5,6]
print a + b
a.extend(b)
print a

a = [1,99,33,44,55,22]
a.insert(1, 2)
a.append((11,33,54))
print a
del a[-1]
a.insert(4, 101)
print a

print [odd for odd in range(1, 101) if (odd > 20) & (odd & 1 == 0)]

"""
exe10
"""
a = (1,2,3)
a = list(a)
a[-1] = 4
print tuple(a)

print ['%s love python' % x for x in range(1,11) ]
print [(a,b) for a in range(3) if a & 1 == 0 for b in range(3) if b & 1 == 0]


"""
exe11
"""
# change tuple
a = (1,2,3)
a = list(a)
a[0] = 5
print tuple(a)

a = (1,2,3)
a = set(a)
a.remove(1)
a.add(5)
print tuple(a)

a = set(['a','b','c'])
a.add('jay')
print a

b = set(['b','e','f','g'])
a.update(b)
print a

print a | b

"""
exe12
"""
ainfo = {'ab':'liming','ac':20}
ainfo['sex']='man'
ainfo['age']=20
print ainfo

ainfo = {'ab':'liming','ac':20}
ainfo.update({'sex':'man','age':20})
print ainfo

print ainfo.keys()
print ainfo.values()
print ainfo['ab']
print ainfo.get('ab')
del ainfo['ac']
print ainfo
ainfo.pop('sex')
print ainfo

"""
exe13
"""
a = [11,22,24,29,30,32]
a.append(28)
print a
a.insert(4, 57)
print a
a[a.index(11)] = 6
print a
a.sort()
print a

b = [1,2,3,4,5]
b.extend([6,7,8])
print b
b = [1,2,3,4,5]
print b+[6,7,8]

b = [1,2,3,4,5]
b.reverse()
print b[:2]
b = [1,2,3,4,5]
print b[-1:-3:-1]

if 2 in b:
    print '2 in list b'
else:
    print '2 not in list b'

b = [23,45,22,44,25,66,78]
o = filter(lambda x:x & 1 == 1, b)
print o
o = ['the content %s' % x for x in b[:2]]
print o
o = [x+2 for x in b]
print o

print [x*11 for x in range(1,4)]
print range(11,34,11)

a = (1,4,5,6,7)
if 4 in a:
    print '4 in a'
else:
    print '4 not in a'
b = list(a)
b[2] = 8
print tuple(b)

setinfo = set('acbdfem')
finfo = set('sabcdef')
setinfo.add('abc')
print setinfo

setinfo.remove('m')
print setinfo

print setinfo | finfo

stu1 = {'name':'liming','age':25,'score':{'Chinese':80,'Math':75,'English':85}}
stu2 = {'name':'zhangqiang','age':23,'score':{'Chinese':75,'Math':82,'English':78}}

print stu1, stu2
stu1['score']['python']=60
stu2['score']['python']=80
print stu1,stu2
stu2['score']['Math']=89
print stu2
stu1.pop('age')
print stu1
print sorted(stu2['score'].iteritems(), key = lambda x:x[1])
print stu1.pop('city', 'beijing')

"""
exe17
"""
i = 1
while i < 11:
    print i,
    i += 1

i = 0
while True:
    i += 1
    print i,
    if i > 9:
        break

for i in range(10):
    if i & 1 == 0:
        continue
    print i,

a = [1,2,3,4,5,6]
for i in a:
    if i == 8:
        print 'find'
        break
else:
    print 'not find'

i = 0
while i < 6:
    a[i] += 1
    i += 1
print a

"""
exe18
"""
a = 'aAsmr3idd4bgs7Dlsf9eAF'
b = []
for i in a:
    if i.islower():
        b.append(i.upper())
    else:
        b.append(i.lower())
print ''.join(b)
# a.swapcase()


b = []
for i in a:
    if i.isdigit():
        b.append(i)
print ''.join(b)
# ''.join([b for b in a if b.isdigit()])

b = a.lower()
c = {}
for i in b:
    if i.isalpha():
        if c.has_key(i):
            c[i] += 1
        else:
            c[i] = 1
print c
# dict([(x, a.count(x)) for x in set(a)])

print ''.join(set(a))
# alist = list(a)
# slist = list(set(alist))
# slist.sort(key=alist.index)
# ''.join(slist)

b = list(a)
b.reverse()
print ''.join(b)
# a[::-1]

b = []
for i in a:
    if not i.isdigit():
        b.append(i)
b.sort(key=str.upper)
print b

c = 'aidD9'
c = list(c)
for i in c:
    for j in a:
        if i == j:
            break
    else:
        print 'not match'
        break
else:
    print 'match'

# search = 'boy'
# u = set(a)
# u.update(list(search))
# len(set(a)) == len(u)


b = 'aAs'
c = 'mr3'
d = 'dd4'
e = '9eA'
c = list(''.join([b,c,d,e]))
for i in c:
    for j in a:
        if i == j:
            break
    else:
        print 'not match'
        break
else:
    print 'match'

c = {}
for i in a:
    if c.has_key(i):
        c[i] += 1
    else:
        c[i] = 1
c = sorted(c.items(), key=lambda x:x[1])
print c[-1][0]

poet = """
The Zen of Python, by Tim Peters

Beautiful is better than ugly.
Explicit is better than implicit.
Simple is better than complex.
Complex is better than complicated.
Flat is better than nested.
Sparse is better than dense.
Readability counts.
Special cases aren't special enough to break the rules.
Although practicality beats purity.
Errors should never pass silently.
Unless explicitly silenced.
In the face of ambiguity, refuse the temptation to guess.
There should be one-- and preferably only one --obvious way to do it.
Although that way may not be obvious at first unless you're Dutch.
Now is better than never.
Although never is often better than *right* now.
If the implementation is hard to explain, it's a bad idea.
If the implementation is easy to explain, it may be a good idea.
Namespaces are one honking great idea -- let's do more of those!
"""

be = poet.count('be')
isc = poet.count('is')
than = poet.count('than')
print be, isc, than

fileinfo = 102324123499123
mb = fileinfo >> 20
kb = fileinfo >> 10
print str(mb) + 'mb'
print str(kb) + 'kb'

a = [1,2,3,6,8,9,10,14,17]
a = [str(x) for x in a]
print ''.join(a)
# str(a)[1:-1].replace(',', '')

"""
exe19
"""
ainfo = {'b':'python','a':'haha','c':'hehe','f':'xiaoming'}
s = sorted(ainfo.items(), key=lambda x:x[1])
print s

for i in s:
    print i[0]

s = dict(s)
print s.keys()

for k,v in s.items():
    if v == 'haha':
        print 'find', k
        break
else:
    print 'not find'

"""
exe20
"""
def get_max(a=123,b=234,c=444):
    lst = [a,b,c]
    lst.sort()
    return lst[-1]
print get_max()

def ainfo(**arg):
    lst = []
    for k,v in arg.items():
        lst.append(v)
    return sorted(lst)
print ainfo(x=88,y=22,z=44)

def cinfo(**arg):
    arg = sorted(arg,key=lambda x:x[0])
    a = '%saa%s'%(arg[0],arg[1])
    b = '%saa%s'%(arg[1],arg[1])
    c = '%saa%s'%(arg[2],arg[1])
    return tuple([a,b,c])
print cinfo(x=88,y=22,z=44)
