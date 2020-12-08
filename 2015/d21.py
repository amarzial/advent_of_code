import re

class Equip():
	def __init__(self,tipo,name,cost,dmg,armor):
		self.tipo = tipo
		self.name = name
		self.cost = int(cost)
		self.dmg = int(dmg)
		self.armor = int(armor)
	
	def stat(self):
		return (self.tipo,self.name,self.cost,self.dmg,self.armor)
			
class Inventory():
	
	def __init__(self):
		self.weapons = []
		self.armor = []
		self.rings = []
	
	def add(self, eq):
		if eq.tipo == "Weapons":
			self.weapons.append(eq)
		elif eq.tipo == "Armor":
			self.armor.append(eq)
		elif eq.tipo == "Rings":
			self.rings.append(eq)

class Hero():
	def __init__(self,equips):
		self.hp = 100
		self.load(equips)
		
	def load(self,equips):
		self.damage = 0
		self.armor = 0
		self.moneyspent = 0
		for e in equips:
			print e.stat()
			self.damage += e.dmg
			self.armor += e.armor
			self.moneyspent += e.cost
	
	def stat(self):
		return self.damage,self.armor,self.moneyspent
	
	def combat(self):
		print self.stat()
		bhp=100
		bd=8
		ba=2
		herodps=self.damage-ba
		bossdps=bd-self.armor
		#print self.damage
		#print self.armor
		if herodps <1:
			herodps = 1
		if bossdps <1:
			bossdps = 1	
		heroturns = float(self.hp)/bossdps
		bossturns = float(bhp)/herodps
		#print 'hero dps', herodps
		#print 'boss dps', bossdps
		if heroturns >= bossturns:
			return True
		else:
			return False
	
def win(weapon):
	wins = []
	setup = []
	caucci = Hero([weapon])
	if caucci.combat():
		wins.append(weapon.cost)
	
	for i in range(-1,len(inv.armor)):
		money = weapon.cost
		if i != -1:
			arm = inv.armor[i]
			money += arm.cost
			caucci.load([weapon,arm])
			if caucci.combat():
				wins.append(money)
		for j in range(-1,len(inv.rings)-1):
			tmpm = money
			if j != -1:
				ring1 = inv.rings[j]
				tmpm += ring1.cost
				if i == -1:
					setup = [weapon,ring1]
				else:
					setup = [weapon,arm,ring1]
				caucci.load(setup)
				if caucci.combat():
					wins.append(tmpm)
			for ring2 in inv.rings[j+1:]:
				tmpm2 = tmpm
				tmpm2 += ring2.cost
				if j == -1 and i == -1:
					setup = [weapon,ring2]
				elif i == -1:
					setup = [weapon,ring1,ring2]
				elif j == -1:
					setup = [weapon,arm,ring2]
				else:
					setup = [weapon,arm,ring1,ring2]
				caucci.load(setup)
				if caucci.combat():
					wins.append(tmpm2)
				
				
	return min(wins)
			
'''
boss		
Hit Points: 100
Damage: 8
Armor: 2
'''

			
tipo = ''
inv = Inventory()
for line in open('d21_equip.txt', 'r'):
	x=re.match('(\w+): ',line)
	if x:
		tipo = x.group(1)
	else:
		if tipo == 'Rings':
			pat = '(\w+)\s+[+]\d\s+(\d+)\s+(\d+)\s+(\d+)'
		else:
			pat = '(\w+)\s+(\d+)\s+(\d+)\s+(\d+)'
		val = re.match(pat,line)	
		inv.add(Equip(tipo,val.group(1),val.group(2),val.group(3),val.group(4)))


print min([win(wep) for wep in inv.weapons])
	
