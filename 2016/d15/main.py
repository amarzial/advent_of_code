comb = []
comb.append((1, 13))
comb.append((10, 19))
comb.append((2, 3))
comb.append((1, 7))
comb.append((3, 5))
comb.append((5, 17))
comb.append((0, 11))
t = 0
valid = False
while not valid:
    valid = True
    adv = t+1
    for c in comb:
        if (c[0] + adv) % c[1] != 0:
            valid = False
            break
        adv += 1
    if valid:
        print (t)
        break
    t += 1
