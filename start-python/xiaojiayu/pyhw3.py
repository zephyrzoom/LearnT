#coding=utf-8

"""
exe4
"""

def get_fundoc(func):
    if hasattr(func, '__call__'):
        if func.__doc__:
            return func.__doc__
    return 'not found'

"""
get sum from 1 to 100 square
@return sum
"""
def get_cjsum():
    sum = 0;
    for i in range(1, 101):
        sum += i*i
    return sum

"""
change list item just affect
in the local function
"""
def list_info(const_list):
    tmp = const_list[:]
    tmp[-1] += 2
    return tmp

"""
get name of a function
"""
def get_funcname(func):
    if hasattr(func, '__call__'):
        return func.__name__
    else:
        return 'func is not a function'

if __name__ == '__main__':
    print get_fundoc(sorted)
    print get_cjsum()
    a = [1,2,3]
    print list_info(a)
    print a
    print get_funcname(a)
    print get_funcname(list)
