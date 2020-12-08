
sample = []

for l in open('d23_input1.txt', 'r').readlines():
	if l[0] == 'j':
		if 'jmp' in l:
			x = [l[0:3], '', l[4:-1]]
		else:
			x = [l[0:3], l[4], l[7:-1]]
	else:
		x = [l[0:3], l[4]]
	sample.append(x)

actions = {'hlf': 'r/2', 'tpl': 'r*3', 'inc': 'r+1'}

a = 0
b = 0
i = 0

def process(z,x,c):
	a = z
	b = x
	i = c
	do = sample[i]
	action = do[0]
	if action in actions:
		val = eval(actions[action].replace('r', do[1]))
		if do[1] == 'a':
			a = val
		elif do[1] == 'b':
			b = val
		i += 1
	else:
		offset = int(do[2])
		if action == 'jie':
			if eval(do[1] + '%2 == 0'):
				i += offset
			else:
				i += 1
		elif action == 'jio':
			if eval(do[1] + ' == 1'):
				i += offset
			else:
				i += 1
		else:
			i += offset
	return (a,b,i)

while i < len(sample):
	a,b,i = process(a,b,i)
	print(a,b)


