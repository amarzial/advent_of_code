data = open("../input/13.txt", 'r').read()
listOfDots, foldInst = data.split('\n\n')

folds = []  # Folds list prepare
for line in foldInst.split('\n'):
    axis, n = line.split('=')[0][-1], int(line.split('=')[1])
    folds.append((axis, n))

dots = set()  # Dots list prepare
for line in listOfDots.split('\n'):
    x, y = int(line.split(',')[0]), int(line.split(',')[1])
    dots.add((x, y))


def fold(axis, n, dots):  # Fold function
    temp = set()
    for x, y in dots:
        if axis == 'x':
            if x > n:
                x = (n * 2) - x
        if axis == 'y':
            if y > n:
                y = (n * 2) - y

        temp.add((x, y))
    return temp


def solve(part):  # Same solving func for both
    _dots = dots.copy()
    for f in (folds[:1] if part == 1 else folds):
        _dots = fold(f[0], f[1], _dots)
        if f[0] == 'x':
            width = f[1]
        elif f[0] == 'y':
            height = f[1]

    matrix = [[0 for _ in range((447 if part == 1 else width) * 2)] \
              for _ in range((width if part == 1 else height) * 2)]

    for x, y in _dots:
        matrix[y][x] = 1

    return matrix


# Part 1
matrix = solve(1)
count = 0
for row in matrix:
    count += sum(x for x in row)
print(f"Part 1: {count}")  # 682

# Part 2
matrix = solve(2)
print("Part 2:")  # FAGURZHE
for row in matrix:
    print(''.join('▓' if x == 1 else ' ' for x in row))
