data = open("../input/10.txt", 'r').read().splitlines()

parent = {'(': ')', '{': '}', '[': ']', '<': '>'}
points = {')': (3, 1), ']': (57, 2), '}': (1197, 3), '>': (25137, 4)}
erased = {')': 0, ']': 0, '}': 0, '>': 0}
close, incorrect, autocomplite = [], [], []

for line in data:
    for chr in line:
        if chr in parent.keys():
            close.append(parent[chr])
        elif chr in parent.values():
            if chr in close[-1]:
                del close[-1]
            else:
                erased[chr] += 1
                close.append(chr)
                incorrect.append(line)
                break
result = 0
for c in erased:
    result += erased[c] * points[c][0]  # Points 1
print(f"Part 1: {result}")  # 388713

correct = set(data) - set(incorrect)
for line in correct:
    close, value = [], 0
    for chr in line:
        if chr in parent.keys():
            close.append(parent[chr])
        elif chr in parent.values():
            if chr in close[-1]:
                del close[-1]

    for ch in close[::-1]:
        value = value * 5 + points[ch][1]   # Points 2
    autocomplite.append(value)

print(f"Part 2: {sorted(autocomplite)[len(autocomplite) // 2]}")  # 3539961434
