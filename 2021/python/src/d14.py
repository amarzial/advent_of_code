from collections import defaultdict

data = open("../input/14.txt", 'r').read().replace("\n\n", "\n").splitlines()

diz = {}
mol = data[0]
for line in data[1:]:
    a, b = line.split(' -> ')
    diz[a] = b

alpha = defaultdict(lambda: 0, '')

for _ in range(10):
    new_mol = mol[0]
    while len(mol) > 0:
        m = mol[:2]
        if m in diz:
            new_mol += diz[m] + m[1]
        mol = mol[1:]
    mol = new_mol

for l in new_mol:
    alpha[l] += 1

print(f"Part 1: {max(alpha.values()) - min(alpha.values())}")

#/////////////  P A R T 2  /////////////

# data = open("../input/14.txt", 'r').read().replace("\n\n", "\n").splitlines()
#
# count = defaultdict(int)
# char_count = defaultdict(int)
#
# diz = {}
# mol = data[0]
# for line in data[1:]:
#     a, b = line.split(' -> ')
#     diz[a] = b
#
# # Initialize
# for i in range(len(mol)-1):
#     pattern = mol[i] + mol[i+1]
#     count[pattern] += 1
#
# for char in mol:
#     char_count[char] += 1
#
#
# for _ in range(40):
#     new_count = []
#     for k, v in count.items():
#         middle_char = diz[k]
#         first = k[0] + middle_char
#         second = middle_char + k[1]
#         new_count.append((first, v))
#         new_count.append((second, v))
#         char_count[middle_char] += v
#     count = defaultdict(int)
#     for k, v in new_count:
#         count[k] += v
#
# print(f"Part 2: {max(char_count.values()) - min(char_count.values())}")