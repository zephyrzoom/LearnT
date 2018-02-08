#coding=utf-8

import urllib


class Geturl(object):
    def __init__(self, url):
        self.url = url
        self.f = urllib.urlopen(url)
        self.content = self.f.read()

    def get_httpcode(self):
        return self.f.getcode()

    def get_htmlcontent(self):
        return self.content

    def get_linknum(self):
        return len(self.content.split('<a href=')) - 1




class SchoolMember:
    '''Represents any school member.'''
    def __init__(self, name, age):
        self.name = name
        self.age = age
        print '(Initialized SchoolMember: %s)' % self.name

    def tell(self):
        '''Tell my details.'''
        print 'Name:"%s" Age:"%s"' % (self.name, self.age),

class Teacher(SchoolMember):
    '''Represents a teacher.'''
    def __init__(self, name, age, salary):
        SchoolMember.__init__(self, name, age)
        self.salary = salary
        print '(Initialized Teacher: %s)' % self.name

    def tell(self):
        print 'Salary: "%d"' % self.salary

class Student(SchoolMember):
    '''Represents a student.'''
    def __init__(self, name, age, marks):
        SchoolMember.__init__(self, name, age)
        self.marks = marks
        print '(Initialized Student: %s)' % self.name

    def tell(self):
        print 'Marks: "%d"' % self.marks

t = Teacher('Mrs. Shrividya', 40, 30000)
s = Student('Swaroop', 22, 75)

if __name__ == '__main__':
    urlinfo = Geturl('http://www.douyutv.com')
    print urlinfo.get_httpcode()
    print urlinfo.get_htmlcontent()
    print urlinfo.get_linknum()



    members = [t, s]
    for member in members:
        member.tell()
