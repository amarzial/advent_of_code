f = open('d8_input1.txt','rb')
l = f.readlines()

def encode(text):
	text = text.replace('\n','')
	newstr = '"'
	for w in text:
		if w == '"':
			newstr += '\\"'
		elif w == '\\':
			newstr += '\\\\'
		else:
			newstr += w
	newstr += '"'
	print newstr
	return newstr

code = sum(map(len,l))-len(l)
strval = sum([len(encode(x)) for x in l])
print code
print strval
print strval - code
f.close()
