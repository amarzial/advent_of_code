import re

f = open("input.txt")

content = f.readlines();

f.close()

class Bot:
	def __init__(self, id):
		self.upper = 0
		self.lower = 0
		self.full = False
		self.ID = id

	def give(self, value):
		if self.lower == 0:
			self.lower = value
		elif value >= self.lower:
			self.upper = value
		else:
			self.upper = self.lower
			self.lower = value
		if self.lower != 0 and self.upper != 0:
			self.full = True
		if self.lower == 17 and self.upper == 61:
			print ("Hello! Number " + str(self.ID) + " reported match")

	def popLower(self):
		ret = self.lower
		self.lower = 0
		self.full = False
		return ret

	def popUpper(self):
		ret = self.upper
		self.upper = 0
		self.full = False
		return ret

	def isFull(self):
		return self.full

	def __repr__(self):
		return ("(" + str(self.upper) + ", " + str(self.lower) + ")")

bots = {}
outs = {}


for line in content:
	res = re.match("value (\d+) goes to bot (\d+)", line)
	if res is not None:
		if not int(res.group(2)) in bots:
			bots[int(res.group(2))] = Bot(int(res.group(2)))
		bots[int(res.group(2))].give(int(res.group(1)))

working = True
current = 0
while working:
	current += 1
	print ("Pass: " + str(current))
	working = False
	for line in content:
		res = re.match("bot (\d+) gives low to (bot|output) (\d+) and high to (bot|output) (\d+)", line)
		if res is not None:
			if int(res.group(1)) in bots and bots[int(res.group(1))].isFull():
				if res.group(2) == "bot":
					if not int(res.group(3)) in bots:
						bots[int(res.group(3))] = Bot(int(res.group(3)))
					bots[int(res.group(3))].give(bots[int(res.group(1))].popLower())
				else:
					if not int(res.group(3)) in outs:
						outs[int(res.group(3))] = []
					outs[int(res.group(3))].append(bots[int(res.group(1))].popLower())
				if res.group(4) == "bot":
					if not int(res.group(5)) in bots:
						bots[int(res.group(5))] = Bot(int(res.group(5)))
					bots[int(res.group(5))].give(bots[int(res.group(1))].popUpper())
				else:
					if not int(res.group(5)) in outs:
						outs[int(res.group(5))] = []
					outs[int(res.group(5))].append(bots[int(res.group(1))].popLower())
				working = True

print(sum(outs[0]) * sum(outs[1]) * sum(outs[2]))