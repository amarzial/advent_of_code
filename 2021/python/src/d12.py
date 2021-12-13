from collections import defaultdict

matrix = defaultdict(list)
data = open("../input/12.txt", 'r').read().splitlines()

for line in data:
    a, b = line.split('-')
    matrix[a].append(b)
    matrix[b].append(a)

queue = ['start']
complete, visited = 0, set()

while queue:
    path = queue.pop(0)
    if path in visited:
        continue

    visited.add(path)
    last_node = path.split('-')[-1]

    for node in matrix[last_node]:
        if node == 'end':
            complete += 1
        elif not node.islower() or node not in path:
            queue.append(path + '-' + node)

print(f"Part 1: {complete}")

queue = ['start']
complete, visited = 0, set()

while queue:
    path = queue.pop(0)
    if path in visited:
        continue

    visited.add(path)
    last_node = path.split('-')[-1]

    for node in matrix[last_node]:
        if node == 'start' or path[0] == '*' and node.islower() and node in path:
            continue
        if node == 'end':
            complete += 1
            continue
        queue.append(('*' if node.islower() and node in path else '') + path + '-' + node)

print(f"Part 2: {complete}")
