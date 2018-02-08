#coding=utf-8

##########################
########exe1
##########################
def max(*num):
    max = num[0]
    for i in num:
        if i > max:
            max = i
    return max
print max(1,2,31,4,5)

###########################
def max_len(*strs):
    max = strs[0]
    for i in strs:
        if len(i) > len(max):
            max = i
    return max
print max_len('sdf','vdfe','iiiiii','fuccccck')

#############################
def get_doc(module):
    return help(module)
import string
get_doc('string')

################################
def get_text(f):
    files = open(f, 'r')
    content = files.read()
    files.close()
    return content

print get_text('info.txt')

######################################
import glob
def get_dir(folder):
    return glob.glob(folder)

print get_dir('*')


#########################
############exe2
########################

def get_num(num):
    odd = []
    for i in num:
        if isinstance(i, int):
            if i & 1 == 0:
                odd.append(i)
        else:
            return 'error'
    return odd

num = [1,2,3,4,5]
assert get_num(num) == [2,4]
num = ['1',2,3]
assert get_num(num) == 'error'

###################################
import urllib
def get_page(url):
    f = urllib.urlopen(url)
    return f.read().decode('utf-8')

print get_page('http://www.baidu.com')


##################################

def max_of_lists(*lists):
    tmp = []
    for i in lists:
        tmp.extend(i)
    sorted(tmp)
    return tmp[-1]

assert max_of_lists([2,4,5],[1,3,9]) == 9


###############################
import os
import glob

def get_dir(f):
    f = '%s*' % f
    files = glob.glob(f)
    if files == []:
        return 'No dirs'
    else:
        return files

print get_dir('./')
