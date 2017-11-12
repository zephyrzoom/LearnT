#coding=utf-8
#author=zeek

'''
1.定义一个func(name)，该函数效果如下。
assert func("lilei") = "Lilei"
assert func("hanmeimei") = "Hanmeimei"
assert func("Hanmeimei") = "Hanmeimei"
'''
def func1(name):
    return name.capitalize()

"""
2.定义一个func(name,callback=None),效果如下。
assert func("lilei") == "Lilei"
assert func("LILEI",callback=string.lower) == "lilei"
assert func("lilei",callback=string.upper) == "LILEI"
"""
import string

def func2(name, callback=None):
    if callback==None:
        return name.capitalize()
    else:
        return callback(name)

"""
3.定义一个func(*kargs),效果如下。

l = func(1,2,3,4,5)
for i in l:
    print i,
#输出 1 2 3 4 5

l = func(5,3,4,5,6)
for i in l:
    print i
#输出 5 3 4 5 6
"""
def func3(*kargs):
    return kargs

"""
4.定义一个func(*kargs)，该函数效果如下。

assert func(222,1111,'xixi','hahahah') == "xixi"
assert func(7,'name','dasere') == 'name'
assert func(1,2,3,4) == None
"""
def func4(*kargs):
    for i in kargs:
        if isinstance(i, str):
            return i
    return None

"""
5.定义一个func(name=None,**kargs),该函数效果如下。

assert func(“lilei”) == "lilei"
assert func("lilei",years=4) == "lilei,years:4"
assert func("lilei",years=10,body_weight=20) == "lilei,years:4,body_weight:20"
"""
def func5(name=None,**kargs):
    # if kargs != None:
    #     for k in kargs:
    #         name = ','.join([name, '%s:%s' % (k, kargs[k])])
    # return name
    lst = ['%s:%s' % (k,v) for k,v in kargs.items()]
    lst.insert(0, name)
    return ','.join(lst)

if __name__ == '__main__':
    assert func1("lilei") == "Lilei"
    assert func1("hanmeimei") == "Hanmeimei"
    assert func1("Hanmeimei") == "Hanmeimei"
    assert func2("lilei") == "Lilei"
    assert func2("LILEI",callback=string.lower) == "lilei"
    assert func2("lilei",callback=string.upper) == "LILEI"
    l = func3(1,2,3,4,5)
    for i in l:
        print i,
    l = func3(5,3,4,5,6)
    for i in l:
        print i,
    assert func4(222,1111,'xixi','hahahah') == "xixi"
    assert func4(7,'name','dasere') == 'name'
    assert func4(1,2,3,4) == None
    assert func5("lilei") == "lilei"
    assert func5("lilei",years=4) == "lilei,years:4"
    print func5("lilei",years=10,body_weight=20)
