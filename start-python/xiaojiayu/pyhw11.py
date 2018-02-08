#coding=utf-8
# import logging

# logger = logging.getLogger()

# hdlr = logging.FileHandler('pyhw11.log')

# formatter = logging.Formatter('%(asctime)s %(levelname)s %(message)s')

# hdlr.setFormatter(formatter)

# logger.addHandler(hdlr)

# logger.setLevel(logging.NOTSET)

# logger.debug('aaa bugs')

def with_file(filename):
    with open(filename, 'r') as f:
        return f.read()

def assert_list(listinfo):
    odd = [i for i in listinfo if (i & 1 == 0) & (i < 100)]
    return odd

class MyException(Exception):
    def __init__(self,error,msg):
        self.args = (error,msg)
        self.error = error
        self.msg = msg



if __name__ == '__main__':
    print with_file('pyhw11.log')
    listinfo = [133,88,33,22,44,11,44,55,33,22,11,11,444,66,555]
    print assert_list(listinfo)
    assert type(assert_list(listinfo)) == list
    assert assert_list(listinfo) == [88,22,44,44,22,66]
    try:
        i = raw_input('input a string: ')
        if len(i) < 5:
            print 'The length of input is %d, excepting is at least 5.' % len(i)
        if len(i) > 5:
            raise MyException(i, 'success')
    except MyException as e:
        print str(e)
