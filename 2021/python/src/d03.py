
data = open('../input/03.txt', 'r').read().splitlines()

ip = 0
gamma = ''

while ip <= len(data[0]) - 1:
    ones, zeros, = 0, 0
    for line in data:
        if line[ip] == '1':
            ones += 1
        else: zeros += 1
    gamma += '1' if ones > zeros else '0'
    ip += 1

epsilon = gamma.replace('1','2').replace('0','1').replace('2','0')
print("Part 1:", int(gamma,2) * int(epsilon,2))


# /////////////////// PART 2 ////////////////////


def clean(data, n):
    garbage, new_data = [], data.copy()
    for i in range(len(new_data)):
        if new_data[i][ip] == n:
            garbage.append(new_data[i])
    return set(new_data) - set(garbage)

o2, co2 = data.copy(), data.copy()

ip = 0
while len(o2) > 1:
    ones, zeros, = 0, 0
    for line in o2:
        if line[ip] == '1':
            ones += 1
        else: zeros += 1

    if ones > zeros:
        o2 = list(clean(o2, '0'))
    elif ones < zeros:
        o2 = list(clean(o2, '1'))
    elif ones == zeros:
        o2 = list(clean(o2, '0'))
    ip += 1

ip = 0
while len(co2) > 1:
    ones, zeros, = 0, 0
    for line in co2:
        if line[ip] == '1':
            ones += 1
        else: zeros += 1

    if ones > zeros:
        co2 = list(clean(co2, '1'))
    elif ones < zeros:
        co2 = list(clean(co2, '0'))
    elif ones == zeros:
        co2 = list(clean(co2, '1'))
    ip += 1

print("Part 2:", int(o2[0], 2) * int(co2[0], 2))

