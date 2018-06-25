import re
a = 'a人是对方fgrcf'
b = ''
for i in a:
    g = re.match('[a-z]',i)
    if g:
        #print(g[0])
        b = b + '[' +  g[0] + g[0].upper() + ']'
        continue
    x = re.match('[A-Z]',i)
    if x:
        b = b + '[' + g[0].lower() + g[0] + ']'
        continue
    b = b + i
    
print(b)
c = ''
for i in a:
    u = i.upper()
    if ord(u) < 65 or ord(u) > 90:
        c += i
        continue
    l = i.lower()
    c += '[%s%s]' % (l,u)

print(c)