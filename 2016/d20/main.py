import copy

class Range:
	def __init__(self, begin, end):
		self.begin = begin
		self.end = end


	def __lt__(self, other):
		if self.begin == other.begin:
			return self.end < other.end
		return self.begin < other.begin

	def __repr__(self):
		return "range: {} - {}".format(self.begin, self.end)

	def joinable(self, other):
		if (self.end + 1)< other.begin:
			return False
		return True

	def __iadd__(self, other):
		self.begin = min(self.begin, other.begin)
		self.end = max(self.end, other.end)
		return self

f = open("input.txt")

l = []
for line in f.readlines():
	b, e = line[:-1].split("-")
	b = int(b)
	e = int(e)
	l.append(Range(b, e))

l.sort()
r = copy.copy(l[0])
for i in range(1, len(l)):
	elem = l[i]
	if (r.joinable(elem)):
		r += elem
	else:
		print("First valid address is: ", r.end + 1)
		break

r = copy.copy(l[0])
allowed = 0
for i in range(1, len(l)):
	elem = l[i]
	if (r.joinable(elem)):
		r += elem
	else:
		allowed += (elem.begin - (r.end + 1))
		r = copy.copy(elem)

print("allowed addresses: ", allowed)
