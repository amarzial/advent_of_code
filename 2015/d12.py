import re

print sum(map(int,re.findall('-?\d+',open('d12_input1.txt', 'r').read())))

