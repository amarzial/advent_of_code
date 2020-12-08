vocals = 'aeiou'
invalid = ('ab','cd','pq','xy')

def check(word):
	count = 1
	for ch in invalid:
		if ch in word:
			count = 0
			
	if count:
		count = 0
		for l in word:
			for v in vocals:
				if v == l:
					count += 1
		if count >= 3:
			count = 0
			for i in range(len(word)-1):
				if word[i] == word[i+1]:
					return True
	return False

def check2(word):
	for i in range(len(word)-3):
		tmp = word[i]+word[i+1]
		for j in range(i+2,len(word)-1):
			tmp2 = word[j]+word[j+1]
			if tmp == tmp2:
				for k in range(len(word)-2):
					if word[k] == word[k+2]:
						return True
				break
	return False			

count = 0
for line in open('read.txt', 'r'):
	if check2(line):
		count += 1
print count
