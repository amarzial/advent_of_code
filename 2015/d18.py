

class Grid():
	class Dot():
		def __init__(self,state = 0):
			self.state = state
			self.neighbors = []
			self.newstate = 0
		
		def check(self):
			c = 0
			for i in self.neighbors:
				if i.state == 1:
					c += 1
			if self.state == 1 and 2 <= c <= 3:
				self.newstate = 1
			elif self.state == 0 and c == 3:
				self.newstate = 1
			else:
				self.newstate = 0
			#For part 2 only ~~~~~~~~~~~~~~~~~~~~~~~~	
			if len(self.neighbors) == 3:
				self.newstate = 1
			#~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		def update(self):
			self.state = self.newstate
			
	
	def __init__(self,filename = ''):
		self.width = 0
		self.height = 0
		self.grid = []
		if filename:
			self.load(filename)
		
	def create(self,w,h):
		self.width = w
		self.height = h
		self.grid = [[self.Dot() for x in range(self.width)] for y in range(self.height)]
	
	def load(self, filename):
		self.grid = [[self.Dot(0) if x == '.' else self.Dot(1) for x in row[:-1]] for row in open(filename, 'r')]
		self.width = len(self.grid[0])
		self.height = len(self.grid)
		#For part 2 only ~~~~~~~~~~~~~~~~~~~~~~~~	
		self.grid[0][0].state = 1
		self.grid[0][len(self.grid[0])-1].state = 1
		self.grid[len(self.grid)-1][0].state = 1
		self.grid[len(self.grid)-1][len(self.grid[0])-1].state = 1
		#~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		
	def attach(self):
		
		for i in range(len(self.grid)):
			for j in range(len(self.grid[0])):
				friends = ((-1,-1),(0,-1),(1,-1),(-1,0),(1,0),(-1,1),(0,1),(1,1))
				if i == 0 and j == 0:
					friends = ((1,0),(0,1),(1,1))
				elif i == 0:
					friends = ((0,-1),(1,-1),(1,0),(0,1),(1,1))
				elif j == 0:
					friends = ((-1,0),(1,0),(-1,1),(0,1),(1,1))
				d = self.grid[i][j]
				for c in friends:
					try:
						d.neighbors.append(self.grid[i+c[0]][j+c[1]])	
					except:
						pass
	
	def step(self):
		for row in self.grid:
			for dot in row:
				dot.check()
		for row in self.grid:
			for dot in row:
				dot.update()
		
	def view(self):
		s = ''
		count = 0
		for row in self.grid:
			for dot in row:
				state = dot.state
				if state == 1:
					count += 1
				s += str(state)
			s += '\n'
		print s
		print count
	

grid = Grid('d18_input1.txt')
grid.attach()
for i in range(100):
	grid.step()
grid.view()	

				
