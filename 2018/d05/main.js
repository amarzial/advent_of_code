const fs = require('fs');
let line = fs.readFileSync('./input.txt','utf-8').split("");
line.pop();
base = line;
console.log("length: ", line.length);

let results = [];
for (let c = -1; c < 26; c++) {
let char = String.fromCharCode('a'.charCodeAt(0) + c);
line = base.slice().filter((v) => {return v.toLowerCase() != char;});
let done = false;
while (!done) {
	done = true;
	for (let i = 0; i < line.length - 1; i++) {
		if (line[i] == ' ') {
			continue;
		}
		for (let j = i + 1; j < line.length; j++) {
			if (line[j] == ' ') {
				continue;
			}
			if (line[i].toLowerCase() == line[j].toLowerCase() && line[i] != line[j]) {
				line[i] = line[j] = ' ';
				done = false;
			}
			break;
		}
	}
}
line = line.filter((v) => {return v != ' ';});
//console.log(line.replace(/ /g, ''));
if (c == -1) {
	console.log("part 1: ", line.length);
} else {
	results.push({char: char, count: line.length});
}
}
console.log("part 2: ", results.reduce((prev, curr) => prev.count < curr.count ? prev : curr));
