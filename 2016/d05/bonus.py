import md5

passwd = [0 for x in range(8)]
size = 0
code = 'uqwqemis'

i = 0
while size < 8:
	found = 0
	while not found:
		m = md5.new()
		m.update(code + str(i))
		res = m.hexdigest()
		if res.startswith('00000') and res[5].isdigit():
			if int(res[5]) < 8 and passwd[int(res[5])] == 0:
				found = 1
				size += 1
				passwd[int(res[5])] = res[6]
		i += 1
	print passwd
print passwd[:8]
