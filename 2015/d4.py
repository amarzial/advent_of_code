import md5

def gethash(seq):
	x = md5.new(seq)
	return x.hexdigest()[:6]
	
found = False
number = 0
while not found:
	if '000000' in gethash('iwrupvqb' + str(number)):
		found = True
		print number
	number += 1
