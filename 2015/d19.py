import re
mol = 'CRnSiRnCaPTiMgYCaPTiRnFArSiThFArCaSiThSiThPB\
CaCaSiRnSiRnTiTiMgArPBCaPMgYPTiRnFArFArCaSiRnBPMgArPRnCa\
PTiRnFArCaSiThCaCaFArPBCaCaPTiTiRnFArCaSiRnSiAlYSiThRnFAr\
ArCaSiRnBFArCaCaSiRnSiThCaCaCaFYCaPTiBCaSiThCaSiThPMgArSi\
RnCaPBFYCaCaFArCaCaCaCaSiThCaSiRnPRnFArPBSiThPRnFArSiRnMg\
ArCaFYFArCaSiRnSiAlArTiTiTiTiTiTiTiRnPMgArPTiTiTiBSiRnSiAl\
ArTiTiRnPMgArCaFYBPBPTiRnSiRnMgArSiThCaFArCaSiThFArPRnFAr\
CaSiRnTiBSiThSiRnSiAlYCaFArPRnFArSiThCaFArCaCaSiThCaCaCa\
SiRnPRnCaFArFYPMgArCaPBCaPBSiRnFYPBCaFArCaSiAl'

l = re.findall('(\w\w?) => (\w+)',open('d19_input1.txt', 'r').read())
'''
subst = {}
for v in r:
	
	if v[0] in subst:
		subst[v[0]].append(v[1])
	else:
		subst[v[0]] = [v[1],]
'''
def part1(r,molecule):
	s = set()
	for old,new in r:
		for i in range(0,len(molecule)-len(old)+1):
			if old in molecule[i:i+len(old)]:
				s.add(molecule[0:i] + new + molecule[i+len(old):])

	print len(s)

def part2(r,molecule):
	base = molecule
	count = 0
	while base != 'e':
		tmp = base
		for a,b in r:
			if not b in base:
				continue
			base = base.replace(b,a,1)
			count += 1
		
		if base == tmp:
			base = molecule
			count = 0
			shuffle(r)
	return count			
 
print part2(l,mol)
