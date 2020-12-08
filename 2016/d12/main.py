
class mem:
	def __init__(self, a = 0, b = 0, c = 0, d = 0):
		self.registers = [a, b, c, d]
		self.pc = 0

	def cpy(self, a, b):
		if a.isdigit():
			get = int(a)
		elif ord(a) >= ord('a') and ord(a) <= ord('d'):
			get = self.registers[ord(a) - ord('a')]
		self.registers[ord(b) - ord('a')] = get
		self.pc += 1

	def inc(self, reg):
		self.registers[ord(reg) - ord('a')] += 1
		self.pc += 1

	def dec(self, reg):
		self.registers[ord(reg) - ord('a')] -= 1
		self.pc += 1

	def jnz(self, a, b):
		if a.isdigit():
			get = int(a)
		elif ord(a) >= ord('a') and ord(a) <= ord('d'):
			get = self.registers[ord(a) - ord('a')]
		if get != 0:
			self.pc += int(b)
		else:
			self.pc += 1

	def __str__(self):
		return "Registers:\na: " + str(self.registers[0]) + "\nb :" + str(self.registers[1]) + \
		"\nc :" + str(self.registers[2]) + "\nd :" + str(self.registers[3])

def parse(line, process):
	objs = line.split()

	if "cpy" == objs[0]:
		process.cpy(objs[1], objs[2])
	elif "inc" == objs[0]:
		process.inc(objs[1])
	elif "dec" == objs[0]:
		process.dec(objs[1])
	elif "jnz" == objs[0]:
		process.jnz(objs[1], objs[2])


def main(a, b, c, d):
	f = open("input.txt", 'r')
	process = mem(a, b, c, d)
	inst = f.readlines()
	codeSize = len(inst)

	while process.pc < codeSize:
		parse(inst[process.pc].rstrip(), process)
	print(process)
	f.close()

print("main:\n")
main(0, 0, 0, 0)
print("bonus:\n")
main(0, 0, 1, 0)