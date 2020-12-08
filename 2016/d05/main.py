import md5

passwd = ''
size = 0
code = 'uqwqemis'

i = 0
while size < 8:
	found = 0
	while not found:
		m = md5.new()
		m.update(code + str(i))
		res = m.hexdigest()
		if res.startswith('00000'):
			found = 1
			size += 1
			passwd += res[5]
		i += 1
	print 'hit: ' + passwd
print passwd
