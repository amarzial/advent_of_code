import re
import itertools
def parse(text):
	x = re.search('(\w+) to (\w+) = (\d+)',text)
	a,b,dist = x.group(1),x.group(2),x.group(3)
	return a,b,dist

locations = []
tab = {}
for l in open('d9_input1.txt'):
	t = parse(l)
	tab[t[0] + ' ' +t[1]] = t[2]
	locations.append(t[0])
	locations.append(t[1])

locations = list(set(locations))
distances = []
for path in itertools.permutations(locations):
	dist = 0
	for i in range(len(path)-1):
		try:
			dist += int(tab[path[i] + ' ' + path[i+1]])
		except:
			dist += int(tab[path[i+1] + ' ' + path[i]])
	distances.append(dist)
print min(distances)
print max(distances)

