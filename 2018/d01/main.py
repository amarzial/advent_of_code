with open("input.txt") as f:
	freq = 0
	for line in f.readlines():
		val = int(line[:-1])
		freq += val

	print (freq)
