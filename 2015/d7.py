import re

wires = {}
depth = 0
def simply(expr,res):
	global wires
	
	if res == 'j':
		pass
	ops = re.findall('[a-z]+', expr)
	for op in ops:
		expr = expr.replace(op,'$$["' + op + '"]')
	expr = expr.replace('$$','wires')
	try:	
		x = (eval(expr) & 0xffff)
		return x
	except:
		return None
	
commands = {}

f = open('d7_input1.txt', 'r')

r = f.read()
r = r.replace('AND','&').replace('OR','|').replace('LSHIFT','<<').replace('RSHIFT','>>').replace('NOT','~')
for command in r.split('\n'):
	if command != '':
		commands[command[command.index('->')+2:].strip()] = (command[:command.index('->')].strip())

while len(wires) != len(commands):
	if len(wires) == 10:
		pass
	for key in commands.keys():
		r = simply(commands[key],key)
		if r != None:
			wires[key] = r
print wires
print wires['a']
