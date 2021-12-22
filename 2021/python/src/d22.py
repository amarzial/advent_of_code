data = open("../input/22.txt", 'r').read()

cubes = set()
limit = range(-50, 51)

for line in data.split('\n'):
    a, b, c = line.split(',')
    s = a[1]
    x, X = int(a.split('..')[0].split('=')[1]), int(a.split('..')[-1])
    y, Y = int(b.split('..')[0].split('=')[1]), int(b.split('..')[-1])
    z, Z = int(c.split('..')[0].split('=')[1]), int(c.split('..')[-1])

    for xx in range(max(-50, x), min(X+1, 51)):
        for yy in range(max(-50, y), min(Y+1, 51)):
            for zz in range(max(-50, z), min(Z+1, 51)):
                if s == 'n': cubes.add((xx, yy, zz))
                else: cubes.discard((xx, yy, zz))

print(f"Part 1: {len(cubes)}")