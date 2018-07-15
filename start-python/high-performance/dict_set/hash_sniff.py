def index_sequence(key, mask=0b111, PERTURB_SHIFT=5):
    perturb = hash(key)
    i = perturb & mask
    yield i
    while True:
        i = ((i << 2) + i + perturb + 1)
        perturb >>= PERTURB_SHIFT
        yield i & mask



class City(str):
    def __hash__(self):
        return ord(self[0])
        
# We create a dictionary where we assign arbitrary values to cities
data = {
    City("Rome"): 4,
    City("San Francisco"): 3,
    City("New York"): 5,
    City("Barcelona"): 2,
}
print(index_sequence('abc'))
print(index_sequence('aaa'))
print(index_sequence('ab1'))
print(index_sequence('aaa2'))
print(index_sequence('abc4'))
print(index_sequence('aaa3'))