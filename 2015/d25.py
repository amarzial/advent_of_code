#from tqdm import tqdm
import time

timeStart = time.time()

row = 2978
column = 3083

index = 1
for i in range(1,row):
	index += i    

for j in range(1,column):
	index += 2 + i
	i+=1
print(index)

first = 20151125
multiplier = 252533
divider = 33554393
code = first

for i in range(1,index):
	code = (code * multiplier) % divider
	#print code
print(code)
print(str(time.time()-timeStart))
