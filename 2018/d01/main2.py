with open("input.txt") as f:
	freq = 0
	unique = set([freq])
	data = f.readlines()
	found = False
	while not found:
		for line in data:
			val = int(line[:-1])
			freq += val
			if (freq in unique):
				print("first recurring frequency: {}".format(freq))
				found = True
				break
			unique.add(freq)
