def steps(path):
	n=0
        s=0
        w=0
        e=0
        for step in path:
                if '^' in step:
                        n +=1
                elif 'v' in step:
                        s +=1
                elif '<' in step:
                        w +=1
                elif '>' in step:
                        e +=1
	return n,s,e,w

def delivers(pattern,mode=0):
	if mode:
		counter = 0
		santa = []
		robot = []
		for step in pattern:
			if counter % 2 == 0:
                		santa.append(step)
        		else:
                		robot.append(step)
        		counter +=1
		n,s,w,e = steps(santa)
		rn,rs,rw,re = steps(robot)
		mn = max(n,rn)
		ms = max(s,rs)
		mw = max(w,rw)
		me = max(e,re)
	else:
		mn,ms,me,mw = steps(pattern)
	mappa = [[0 for x in range(me+mw+1)] for x in range(mn+ms+1)]
	x=mw
	y=ms
	xr=x
	yr=y
	mappa[y][x] = 1
	count = 0
	for step in pattern:
		a=0
		b=0
                if '^' in step:
			a = 1
                elif 'v' in step:
                        a = -1
                elif '<' in step:
                        b = -1
                elif '>' in step:
                        b = 1
		if count % 2 and mode:
			xr += b
			yr += a
			mappa[yr][xr] += 1
		else: 	
			x += b
			y += a
			mappa[y][x] += 1
		count +=1 
	count = 0
	for i in range(mn+ms+1):
		for j in range(mw+me+1):
			if mappa[i][j] >= 1:
				count +=1
	return count
	

f = open('read.txt', 'r')
x = int(raw_input('Mode: \n0)Solo\n1)Dual\n: '))
print delivers(f.read(),x)
