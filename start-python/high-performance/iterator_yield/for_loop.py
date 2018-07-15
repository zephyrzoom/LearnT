# python 循环
for i in object:
    do_work(i)
# 等同于
object_iterator = iter(object)
while True:
    try:
        i = object_iterator.next()
        do_work(i)
    except StopIteration:
        break

def do_work(i):
    pass