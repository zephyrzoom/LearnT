def linear_search(needle, array):
    for i, item in enumerate(array):
        if item == needle:
            return i
    return -1

a = range(10000000000)
i = linear_search(99999999, a)
print(i)
#a.index(99999999)