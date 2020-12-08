import re

sample = '''Sugar: capacity 3, durability 0, flavor 0, texture -3, calories 2
Sprinkles: capacity -3, durability 3, flavor 0, texture 0, calories 9
Candy: capacity -1, durability 0, flavor 4, texture 0, calories 1
Chocolate: capacity 0, durability 0, flavor -2, texture 2, calories 8'''

t = re.findall('\w+: capacity (-?\d), \
durability (-?\d), \
flavor (-?\d), \
texture (-?\d), \
calories (-?\d)',sample)

r = [[int(x) for x in l] for l in t]

scores = []
for i in range(1,101):
	for j in range(1,101):
		for k in range(1,101):
			for m in range(1,101):
				if i+j+k+m == 100:
					cal = r[0][4] * i + r[1][4] * j + r[2][4] * k + r[3][4] * m
					if cal == 500:
						tmp = 1
						for z in range(4):
							n = r[0][z] * i + r[1][z] * j + r[2][z] * k + r[3][z] * m
							if n < 0:
								n = 0
							tmp *= n
						print i,j,k,m,tmp
						if tmp > 0:
							scores.append(tmp)
			

print max(scores)

