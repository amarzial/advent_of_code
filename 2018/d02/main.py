def getcount(word):
	double = False
	triple = False
	for c in word:
		cnt = word.count(c)
		if cnt == 2:
			double = True
		elif cnt == 3:
			triple =` True
	return (double, triple)

with open("input.txt") as f:
	double = 0
	triple = 0
	for word in f.readlines():
		d, t = getcount(word)
		print(d, t)
		double += 1 if d else 0
		triple += 1 if t else 0
	print (double * triple)
