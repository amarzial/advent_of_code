line = ".^..^....^....^^.^^.^.^^.^.....^.^..^...^^^^^^.^^^^.^.^^^^^^^.^^^^^..^.^^^.^^..^.^^.^....^.^...^^.^."
steps = 400000
safe = line.count(".")

#print(line)


for s in range(steps - 1):
	newline = ""
	for i in range(len(line)):
		char = "."
		status = 0
		if (i > 0) and line[i - 1] == "^":
			status += 1
		if line[i] == "^":
			status += 2
		if (i < len(line) - 1) and line[i + 1] == "^":
			status += 4

		if status == 3 or status == 6 or status == 1 or status == 4:
			char = "^"
		if char == ".":
			safe += 1
		newline += char
	#print (newline)
	line = newline

print("The floor has {} safes".format(safe))
