import time

timestart = time.time()

containers = sorted([int(x[:-1]) for x in open('d17_input1.txt', 'r').readlines()], reverse=True)

liters = 150
depth = 0
combs = {}
def fill(capacity,leftcont):
	global combs
	global depth
	depth += 1
	count = 0
	for i in range(len(leftcont)):
		if leftcont[i] == capacity:
			count += 1
			if depth in combs:
				combs[depth] += 1
			else:
				combs[depth] = 1
		elif leftcont[i] < capacity:
			count += fill(capacity-leftcont[i],leftcont[i+1:])
	depth -= 1
	return count
	
print (time.time()-timestart)*1000
print 'Part1: ' + str(fill(liters,containers))
print 'Part2: ' + str(combs[min(combs)])

