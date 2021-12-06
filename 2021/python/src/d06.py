import copy
class LanternFish:

    def __init__(self, bio):
        self.bio = bio

    def check(self):
        if self.bio < 0:
            self.bio = 6
            population.append(LanternFish(9))

data = open("../input/06.txt", 'r').read()
data = [int(x) for x in data.split(',')]
population = [LanternFish(x) for x in data]

for _ in range(80):
    for fish in population:
        fish.bio -= 1
        fish.check()

print("Part 1:", len(population))

# ///////////// P A R T 2 /////////////

pop, temp_pop = {},{}
for x in range(-1,10):
    pop[x] = data.count(x)

for _ in range(256):
    born = pop[0]
    for i in range(9):
        pop[i] = pop[i+1]
    pop[8] = born
    pop[6] = pop[6] + born

print(sum(pop.values()))


