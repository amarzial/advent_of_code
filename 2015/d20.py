from numutils import divisors

count = 0
val = 500000
#part 1 
'''
while count< 36000000:
	count = 0
	for n in divisors(val,True):
		count += n*10
	
	val +=1
'''
#part 2	
while count< 36000000:
	count = 0
	for n in divisors(val,True):
		if (val)/n <= 50:
			count += n*11
	
	val +=1

print val-1
