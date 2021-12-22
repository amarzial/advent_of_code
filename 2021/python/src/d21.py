class Dice:
    def __init__(self):
        self.face = 0
        self.round = 0

    def roll(self):
        a, b, c = self.face + 1, self.face + 2, self.face + 3
        self.face = (c - 1) % 100 + 1
        self.round += 3
        return a + b + c


class Player:
    def __init__(self, pos):
        self.pos = pos
        self.score = 0

    def play(self, dice):
        self.pos = (self.pos + dice) % 10
        if self.pos == 0:
            self.score += 10
        else:
            self.score += self.pos


dice = Dice()
players = [Player(10), Player(7)]

for _ in range(500):
    for i in range(2):
        players[i].play(dice.roll())
        if players[i].score >= 1000:
            print(players[(i + 1) % 2].score * dice.round)
            exit()