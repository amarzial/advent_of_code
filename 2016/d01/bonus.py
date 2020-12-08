f = open("input.txt", "r")

moves = f.read().split(", ")
dirs = [[1, 0], [0, 1], [-1, 0], [0, -1]]
store = dict()

direction = 0
pos = [0, 0]
for move in moves:
	if move[0] == 'R':
		direction += 1
	elif move[0] == 'L':
		direction -= 1
	direction %= 4
	val = int(move[1:])
	tmp = [0, 0]
	for i in range(val):
		tmp[0] = pos[0] + dirs[direction][0] * (i + 1)
		tmp[1] = pos[1] + dirs[direction][1] * (i + 1)
		res = str(tmp)
		if res in store:
			print (abs(tmp[0]) + abs(tmp[1]))
			exit()
		else:
			store[res] = 1
	pos[0] += dirs[direction][0] * val
	pos[1] += dirs[direction][1] * val
