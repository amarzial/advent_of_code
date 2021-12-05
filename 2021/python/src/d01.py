
INPUT = open('../input/01.txt').read().splitlines()


# Part1
def solve(nums):
    prev, deep = int(nums[0]), 0
    for n in nums:
        if prev < int(n):
            deep += 1
        prev = int(n)
    return deep


# Part2
partials, diz = [], []
for line in INPUT:
    diz.append(int(line))

for i in range(len(diz) - 2):
    Sum = diz[i] + diz[i + 1] + diz[i + 2]
    partials.append(Sum)

print("Part 1:", solve(INPUT))
print("Part 2:", solve(partials))