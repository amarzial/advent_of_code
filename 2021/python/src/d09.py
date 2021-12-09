import operator as op

data = open("../input/09.txt", 'r').read()
matrix = [list(l.strip()) for l in data.split('\n')]
matrix = [[int(x) for x in y] for y in matrix]
L, A = len(matrix[0]), len(matrix)
count, basins = 0, []

def check(y, x):
    temp = []
    if y > 0:
        temp.append((y-1, x))
    if x > 0:
        temp.append((y, x-1))
    if y < A - 1:
        temp.append((y+1, x))
    if x < L - 1:
        temp.append(((y, x+1)))

    for ty, tx in temp:
        if matrix[ty][tx] <= matrix[y][x]:
            return False
    return True


def explore(y, x, visited=[]):
    if matrix[y][x] == 9 or (y, x) in visited:
        return 0
    else: visited.append((y, x))

    u, d, l, r = 0, 0, 0, 0
    if y > 0:
        u = explore(y-1, x, visited)
    if x > 0:
        l = explore(y, x-1, visited)
    if y < A - 1:
        d = explore(y+1, x, visited)
    if x < L - 1:
        r = explore(y, x+1, visited)

    return u + d + l + r + 1

for y in range(A):
    for x in range(L):
        if check(y, x):
            count += matrix[y][x] + 1
            basins.append(explore(y, x))
basins = sorted(basins, reverse=True)

print(f"Part 1: {count}") # 539
print(f"Part 2: {op.mul(basins[0], op.mul(basins[1], basins[2]))}")