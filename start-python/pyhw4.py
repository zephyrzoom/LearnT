#coding=utf-8
#auther=zeek

"""
return max element
"""
def position_match(lst1,lst2,lst3):
    if isinstance(lst1,list) & isinstance(lst2,list) & isinstance(lst3,list):
        lst1.extend(lst2)
        lst1.extend(lst3)
        lst1 = sorted(lst1)
        return lst1[-1]
    else:
        return 'error'

def key_match(lst1=None,lst2=None,lst3=None):
    if isinstance(lst1,list) & isinstance(lst2,list) & isinstance(lst3,list):
        lst1.extend(lst2)
        lst1.extend(lst3)
        lst1 = sorted(lst1)
        return lst1[-1]
    else:
        return 'error'

def tuple_collect_match(*lst):
    tmp = []
    for i in lst:
        if isinstance(i,list):
            tmp.extend(i)
        else:
            return 'error'
    return sorted(tmp)[-1]

def dict_collect_match(**lst):
    tmp = []
    # k is the key of each dict item, get the value by lst[k]
    for k in lst:
        if isinstance(lst[k],list):
            print lst[k]
            tmp.extend(lst[k])
        else:
            return 'error'
    return sorted(tmp)[-1]

if __name__ == '__main__':
    print filter(lambda x:x & 1 == 0, range(1,101))
    assert position_match([1,2,3],[1,5,64],[33,445,22]) == 445
    assert key_match(lst2=[1,2,3],lst1=[1,5,64],lst3=[33,445,22]) == 445
    assert tuple_collect_match([1,2,3],[1,5,64],[33,445,22]) == 445
    assert dict_collect_match(lst1=[1,2,3],lst2=[1,5,64],lst3=[33,445,22]) == 445
