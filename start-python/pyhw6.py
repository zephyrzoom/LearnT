#coding=utf-8
#author=zeek

class Student(object):

    def __init__(self, name, age, score):
        self.name = name
        self.age = age
        self.score = score

    def get_name(self):
        return str(self.name)

    def get_age(self):
        return int(self.age)

    def get_score(self):
        return max(self.score)


class DictClass(object):
    def __init__(self, arg):
        self.arg = arg

    def del_dict(self, key):
        del self.arg[key]

    def get_dict(self, key):
        if self.arg.has_key(key):
            return self.arg[key]
        else:
            return 'not found'

    def get_key(self):
        return self.arg.keys()

    def update_dict(self, update):
        self.arg.update(update)
        return self.arg

if __name__ == '__main__':
    zm = Student('zhangming',20,[69,88,100])
    print zm.get_name()
    print zm.get_age()
    print zm.get_score()

    dc = DictClass({'a':1,'b':2,'c':3})
    print dc.get_key()
    print dc.get_dict('a')
    print dc.get_dict('b')
    print dc.get_dict('c')
    dc.del_dict('c')
    print dc.get_key()
    print dc.update_dict({'d':4})
