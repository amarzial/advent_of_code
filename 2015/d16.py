import re

f = open('d16_input1.txt', 'r')
pattern = '(\w+: \d+)'

message = {'children': 3,
'cats': 7,
'samoyeds': 2,
'pomeranians': 3,
'akitas': 0,
'vizslas': 0,
'goldfish': 5,
'trees': 3,
'cars': 2,
'perfumes': 1}

def match(dic1,dic2):
	for voice in dic1:
		try:	
			if dic1[voice] != dic2[voice]:
				return False
		except:
			pass
	return True

def match2(dic1,dic2):
	for voice in dic1:
		mode = 0
		if voice == 'cats' or voice == 'trees':
			mode = 1
		elif voice == 'pomeranians' or voice == 'goldfish':
			mode = 2
		try:
			if mode == 0:	
				if dic1[voice] != dic2[voice]:
					return False
			elif mode == 1:	
				if dic1[voice] <= dic2[voice]:
					return False
			elif mode == 2:	
				if dic1[voice] >= dic2[voice]:
					return False
		except:
			pass
	return True

aunts = []
for line in f.readlines():
	props = {}
	x = re.findall(pattern,line)
	for p in x:
		r = p.split(': ')
		props[r[0]] = int(r[1])
	aunts.append(props)
for i in range(len(aunts)):
	aunt = aunts[i]
	if match(aunt,message):
		print 'Match found: ' + str(i+1)
				

