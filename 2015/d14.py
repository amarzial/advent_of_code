import re
sample ='''Vixen can fly 19 km/s for 7 seconds, but then must rest for 124 seconds.
Rudolph can fly 3 km/s for 15 seconds, but then must rest for 28 seconds.
Donner can fly 19 km/s for 9 seconds, but then must rest for 164 seconds.
Blitzen can fly 19 km/s for 9 seconds, but then must rest for 158 seconds.
Comet can fly 13 km/s for 7 seconds, but then must rest for 82 seconds.
Cupid can fly 25 km/s for 6 seconds, but then must rest for 145 seconds.
Dasher can fly 14 km/s for 3 seconds, but then must rest for 38 seconds.
Dancer can fly 3 km/s for 16 seconds, but then must rest for 37 seconds.
Prancer can fly 25 km/s for 6 seconds, but then must rest for 143 seconds.'''

time = 2503

x=re.findall('\w+ can fly (\d+) km/s for (\d+) seconds, but then must rest for (\d+) seconds.',sample)
r = [[int(i) for i in j] for j in x]
print r
rest = lambda tot,act: act if time%tot > act else time%tot  
space = [v*a*(time/(a+p)) + v*rest(a+p,a) for v,a,p in r]
print max(space)
