data = open("../input/07.txt", 'r').read()
data = [int(x) for x in data.split(',')]
low, high = min(data), max(data)

fuel, diz = 0, {}
for pos in range(low, high + 1):
    for n in data:
        fuel += abs(n - pos)
    diz[pos] = fuel
    fuel = 0

print("Part 1:", diz[min(diz, key=lambda k: diz[k])])

fuel, diz = 0, {}
for pos in range(low, high + 1):
    for i in [abs(n - pos) for n in data]:
        fuel += i * (i + 1)//2
    diz[pos] = fuel
    fuel = 0

print("Part 2:", diz[min(diz, key=lambda k: diz[k])])