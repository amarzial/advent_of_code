import random
import operator
from tqdm import tqdm

paks = [int(x[:-1]) for x in open('d24_input1.txt', 'r').readlines()]

groups = 4

#paks = [1,2,3,4,5,7,8,9,10,11]
valids = set([])
weight = int(sum(paks)/groups)
l = list(paks)
for c in tqdm(range(200000)):
	random.shuffle(l)
	s = 0
	for i in range(len(l)):
		s += l[i]
		if s > weight:
			break
		elif s == weight:
			valids.add(tuple(l[:i+1]))

answer = min(map(sum,valids))
size = min(map(len,valids))

res = list(valids)
res.sort(key = lambda k: len(k), reverse = True)
ok = []
for v in res:
	if sum(v) == answer and len(v) == size:
		ok.append(v)

def prod(l):
	p = 1
	for v in l:
		p *= v
	return p

print min([prod(var) for var in ok])
