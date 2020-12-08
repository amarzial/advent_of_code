import re
import random

sample = '''Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3'''

sample2 = '''Sugar: capacity 3, durability 0, flavor 0, texture -3, calories 2
Sprinkles: capacity -3, durability 3, flavor 0, texture 0, calories 9
Candy: capacity -1, durability 0, flavor 4, texture 0, calories 1
Chocolate: capacity 0, durability 0, flavor -2, texture 2, calories 8'''

t = re.findall('\w+: capacity (-?\d), \
durability (-?\d), \
flavor (-?\d), \
texture (-?\d), \
calories -?\d',sample2) #calories (-?\d)',sample)

r = [[int(x) for x in l] for l in t]

def comb(l):
	tot = 0
	cmb = []
	for i in range(l,1,-1):
		t = random.randrange(1,102-i-tot)
		cmb.append(t)
		tot += t
	cmb.append(100-tot)
	random.shuffle(cmb)
	return cmb

def calc(spoons, ings):
	t = 1
	for i in range(len(ings[0])):
		n = 0
		for j in range(len(spoons)):
			n += ings[j][i] * spoons[j]
		if n < 0:
			n = 0
		t *= n
	return t

def bestof(spoons, ings):
	val = calc(spoons,ings)
	m = val
	for i in range(len(spoons)):
		up = spoons[i] + 1
		for j in range(len(spoons))[:i] + range(len(spoons))[i+1:]:
			down = spoons[j]-1
			tmp = spoons
			tmp[i] = up
			tmp[j] = down
			clc = calc(tmp,ings)
			if  clc > m:
				m = bestof(tmp,ings)
	return m


points = []			
tries = 500
while tries > 0:
	c = comb(len(r))
	bo = bestof(c,r)
	if bo > 0:
		points.append(bo)
		tries -= 1

print max(points)
	

