f = open('d8_input1.txt','rb')
l = f.readlines()

code = sum(map(len,l))-len(l)
strval = sum([len(eval(x)) for x in l])
print code
print strval
print code - strval
f.close()
