def piano(azioni):
	start = 0
	for i in azioni:
		if '(' in i:
			start += 1
		elif ')' in i:
			start -= 1
	return start

def character(azioni):
	start = 0
	hop = 0
	for i in azioni:
		hop +=1
		if '(' in i:
			start += 1
		elif ')' in i:
			start -= 1
		if start == -1:
			return hop
	return 'not found'
f = open('read.txt', 'r')
print character(f.read())
