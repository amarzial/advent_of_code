import itertools

def paper(l,w,h):
	aree = [a*b for a,b in itertools.permutations((l,w,h),2)]
	return sum(aree + [min(aree)])

def ribbon(l,w,h):
	perimetro = min([(a+b)*2 for a,b in itertools.permutations((l,w,h),2)])
	return perimetro + l*w*h

carta = 0
for line in open('read.txt', 'r'):
	if not line == '\n':
		args = line.split('x')
		print args
		carta +=ribbon(int(args[0]),int(args[1]),int(args[2]))

print carta
