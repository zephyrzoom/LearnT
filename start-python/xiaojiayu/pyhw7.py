#coding=utf-8
#author=zeek

"""
定义一个列表的操作类：Listinfo

包括的方法:

1 列表元素添加: add_key(keyname)  [keyname:字符串或者整数类型]
2 列表元素取值：get_key(num) [num:整数类型]
3 列表合并：update_list(list)      [list:列表类型]
4 删除并且返回最后一个元素：del_key()
"""
class ListInfo(object):
    def __init__(self, arg):
        self.arg = arg

    def add_key(self, keyname):
        self.arg.append(keyname)

    def get_key(self, num):
        return self.arg[num]

    def update_list(self, lst):
        return self.arg.extend(lst)

    def del_key(self):
        return self.arg.pop()


"""
定义一个集合的操作类：Setinfo

包括的方法:

1 集合元素添加: add_setinfo(keyname)  [keyname:字符串或者整数类型]
2 集合的交集：get_intersection(unioninfo) [unioninfo :集合类型]
3 集合的并集： get_union(unioninfo)[unioninfo :集合类型]
4 集合的差集：del_difference(unioninfo) [unioninfo :集合类型]

set_info =  Setinfo(你要操作的集合)
"""

class Setinfo(object):
    def __init__(self, arg):
        self.arg = arg

    def add_setinfo(self, keyname):
        self.arg.add(keyname)

    def get_intersection(self, unioninfo):
        return self.arg & unioninfo

    def get_union(self, unioninfo):
        return self.arg | unioninfo

    def del_difference(self, unioninfo):
        return self.arg - unioninfo



if __name__ == '__main__':
    list_info = ListInfo([44,222,111,333,454,'sss','333'])
    list_info.add_key('add')
    print list_info.get_key(-1)
    list_info.update_list([1,2,3])
    print list_info.get_key(-1)
    list_info.del_key()
    print list_info.get_key(-1)

    set_info = Setinfo(set(['a','b']))
    set_info.add_setinfo('c')
    print set_info.get_union(set(['d','a']))
    print set_info.get_intersection(set(['d','a']))
    print set_info.del_difference(set(['d','a']))

