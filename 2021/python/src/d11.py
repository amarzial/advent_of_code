stack = []

def possibleDirections():
    return [(-1, 0), (1, 0), (0, -1), (0, 1), (-1, -1), (-1, 1), (1, 1), (1, -1)]


def solve(n):
    data = open("../input/11.txt", 'r').read().splitlines()
    matrix = [[int(x) for x in y] for y in data]
    A, L = len(matrix) - 1, len(matrix[0]) - 1

    flash = 0
    for step in range(n):
        for y in range(A + 1):
            for x in range(L + 1):
                matrix[y][x] = (matrix[y][x] + 1) % 10
                if matrix[y][x] == 0:
                    flash += 1
                    stack.append((y, x))

        while stack:
            y, x = stack.pop()
            for dy, dx in possibleDirections():
                yy, xx = y + dy, x + dx
                if 0 <= yy <= A:
                    if 0 <= xx <= L:
                        if matrix[yy][xx] != 0:
                            matrix[yy][xx] = (matrix[yy][xx] + 1) % 10
                            if matrix[yy][xx] == 0:
                                flash += 1
                                stack.append((yy, xx))
            zeros = 0
            for r in matrix:
                zeros += sum(1 if x == 0 else 0 for x in r)
                if zeros == 100:
                    return step + 1
    return flash


print(f"Part 1: {solve(100)}")  # 1647
print(f"Part 2: {solve(400)}")  # 348
