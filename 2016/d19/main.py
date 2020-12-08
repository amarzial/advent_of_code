size = 3018458

winner = 0
power = 0

while winner + 2**power < size:
	winner += 2**power
	power += 1

print ("Elf {} wins".format(1 + (size - winner - 1) * 2))


import math
size = 3018458
root = math.log(size, 3)
base = math.floor(root)
up = math.ceil(root)
diff = 3**(up) - 3**base
evens = diff - (3**(base + 1) // 2 + 1)
idx = (size - 3**base)
if (base == up):
    n = size
else:
    if idx // 2 <= evens:
        n = idx // 2 * 2 + (1 if idx % 2 != 0 else 0)
    else:
        n = ((idx - evens - 1) * 2 - 1) + 2 * evens
print ("Elf {} wins".format(n))
