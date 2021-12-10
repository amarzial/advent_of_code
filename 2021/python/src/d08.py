data = open("../input/08.txt", 'r').read().splitlines()
dgt = [s.split(' | ')[1] for s in data]
dgt, count = [s.split() for s in dgt], 0

for s in dgt:
    for k in s:
        if len(k) in (2, 4, 3, 7):
            count += 1
print(f"Part 1: {count}")

# ////////////////// PART 2 //////////////

map = {}
result = 0

def decode(a, b):
    sum = 0
    for l in a:
        if l in b:
            sum += 1
    return sum

def wires(signals):
    for str in signals:
        if len(str) == 2:
            map[1] = str
        elif len(str) == 4:
            map[4] = str
        elif len(str) == 3:
            map[7] = str
        elif len(str) == 7:
            map[8] = str

    for str in signals:
        if len(str) == 5:
            if decode(str, map[1]) == 2:        # 3
                map[3] = str
            elif decode(str, map[4]) == 2:      # 2
                map[2] = str
            else:                               # 5
                map[5] = str

        elif len(str) == 6:
            if decode(str, map[4]) == 4:        # 9
                map[9] = str
            elif decode(str, map[1]) == 2:      # 0
                map[0] = str
            else:                               # 6
                map[6] = str


for line in data:
    count = ''
    sgn, dgt = line.split(' | ')
    wires(sgn.split(' '))
    for d in dgt.split(' '):
        for n in range(10):
            if sorted(d) == sorted(map[n]):
                count += str(n)
    result += int(count)

print(f"Part 2: {result}")








