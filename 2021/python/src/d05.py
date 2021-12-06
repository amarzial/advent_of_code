from collections import defaultdict

data = open('../input/05.txt', 'r').read().splitlines()


def solve(part):
    grid = defaultdict(int)

    for line in data:
        x1, y1, x2, y2 = [int(x) for x in line.replace(' -> ', ',').split(',')]

        if y1 == y2:
            for k in range(min(x1, x2), max(x1, x2) + 1):
                grid[(k, y1)] += 1
                continue

        elif x1 == x2:
            for k in range(min(y1,y2), max(y1, y2) + 1):
                grid[(x1, k)] += 1
                continue

        else:
            if part == 2:
                a, b = (x1, y1), (x2, y2)
                start, end = min(a, b, key=lambda x: x[1]), max(a, b, key=lambda x: x[1])
                x = start[0]
                x_shift = 1 if start[0] < end[0] else -1
                for k in range(start[1], end[1] + 1):
                    grid[(x, k)] += 1
                    x += x_shift

    return sum(1 for v in grid.values() if v >= 2)


print("Part 1:", solve(1))
print("Part 2:", solve(2))
