import sys
start = sys.argv[1]
if len(sys.argv) > 2:
	times = int(sys.argv[2])
else:
	times = 1
def parse(start):
	tmp = start[0]
	var = ''
	l = []
	for w in start:
		if w == tmp:
			var += w
		else:
			tmp = w
			l.append(var)
			var = w
	l.append(var)
	return result(l)

def result(parsed):
	res = ''
	for group in parsed:
		tmp = str(len(group)) + group[0]
		res += tmp
	return res
	
tmp = start
for i in range(times):
	tmp = parse(tmp)

print(len(tmp))
