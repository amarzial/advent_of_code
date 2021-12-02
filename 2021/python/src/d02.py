from getinput import GetInput

INPUT = GetInput('../input/02.txt')

horizon = depth = 0

for line in INPUT:
    go, val = line[0], int(line[-1])
    if go == 'f':
        horizon += val
    elif go == 'd':
        depth += val
    elif go == 'u':
        depth -= val

print("Part 1:", horizon * depth)

horizon = depth = aim = 0

for line in INPUT:
    go, val = line[0], int(line[-1])
    if go == 'd':
        aim += val
    elif go == 'u':
        aim -= val
    elif go == 'f':
        horizon += val
        depth += aim * val

print("Part 2:", horizon * depth)
