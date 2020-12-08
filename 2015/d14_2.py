import re
import operator
sample ='''Vixen can fly 19 km/s for 7 seconds, but then must rest for 124 seconds.
Rudolph can fly 3 km/s for 15 seconds, but then must rest for 28 seconds.
Donner can fly 19 km/s for 9 seconds, but then must rest for 164 seconds.
Blitzen can fly 19 km/s for 9 seconds, but then must rest for 158 seconds.
Comet can fly 13 km/s for 7 seconds, but then must rest for 82 seconds.
Cupid can fly 25 km/s for 6 seconds, but then must rest for 145 seconds.
Dasher can fly 14 km/s for 3 seconds, but then must rest for 38 seconds.
Dancer can fly 3 km/s for 16 seconds, but then must rest for 37 seconds.
Prancer can fly 25 km/s for 6 seconds, but then must rest for 143 seconds.'''

class Reindeer():
	def __init__(self,name,speed,atime,rtime):
		self.score = 0
		self.__space = 0
		self.__timer = 0
		self.__name = name
		self.__speed = int(speed)
		self.__atime = int(atime)
		self.__rtime = int(rtime)
	
	def step(self):
		self.__timer += 1
		if self.__timer > (self.__atime+self.__rtime):
			self.__timer = self.__timer%(self.__atime+self.__rtime)
		if self.__timer <= self.__atime:
			self.__space += self.__speed

	def getspace(self):
		return self.__space

	def getname(self):
		return self.__name
	
time = 2503

x=re.findall('(\w+) can fly (\d+) km/s for (\d+) seconds, but then must rest for (\d+) seconds.',sample)
reindeers = [Reindeer(r[0],r[1],r[2],r[3]) for r in x]

for i in range(time):
	p = [0 for k in range(len(reindeers))]
	for j in range(len(reindeers)):
		reindeers[j].step()
		p[j] = reindeers[j].getspace()
	for j in range(len(p)):
		if p[j] == max(p):
			reindeers[j].score += 1

print 'Leaderboard: '
for reindeer in sorted(reindeers, key=operator.attrgetter('score'),reverse = True):
	print reindeer.getname(), str(reindeer.score) + ' points and ' + str(reindeer.getspace()) + ' kilometers'
