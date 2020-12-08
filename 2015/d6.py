import re 
grid = [[0 for x in range(1000)] for x in range(1000)]

def count():
	cnt = 0
	for i in range(1000):
		for j in range(1000):
			if grid[i][j] == 1:
				cnt +=1
	return cnt

def count2():
	cnt = 0
	for i in range(1000):
		for j in range(1000):
			cnt += grid[i][j]
	return cnt
	
def change(start,end,action,do):
	for i in range(start[0], end[0]+1):
		for j in range(start[1], end[1]+1):
			if action == 'turn':
				put = 0
				if do == 'on':
					put = 1
				grid[i][j] = put
			elif action == 'toggle':
				put = 0
				if grid[i][j] == 0:
					put = 1
				grid[i][j] = put
			
def change2(start,end,action,do):
	for i in range(start[0], end[0]+1):
		for j in range(start[1], end[1]+1):
			if action == 'turn':
				put = -1
				if do == 'on':
					put = 1
				grid[i][j] += put
			elif action == 'toggle':
				grid[i][j] += 2		
			if grid[i][j] < 0:
				grid[i][j] = 0
def process(command):
	action = re.search('(\w+) ', command).group(1)
	pattern = ''
	
	if action == 'turn':
		pattern = 'turn \w+ (\d+),(\d+)' 
		state = re.search('turn (\w+) \d',command).group(1)
	elif action == 'toggle':
		pattern = 'toggle (\d+),(\d+)'
		state = ''
	startpoint = re.search(pattern,command)
	start_x = int(startpoint.group(1))
	start_y = int(startpoint.group(2))
	endpoint = re.search('through (\d+),(\d+)',command)
	end_x = int(endpoint.group(1))
	end_y = int(endpoint.group(2))
	change2((start_x,start_y), (end_x, end_y), action, state)

for line in open('d6_input1.txt', 'r'):
	process(line)

print count2()

