const utils = require('../utils.js');

function convert(string) {
	let out = 0;
	for (let c of string) {
		out <<= 1;
		out += c == '1' ? 1: 0;
	}
	return out;
}

let list = [];
let max = 0;
for (let line of utils.readLines(process.argv[2])) {
	let row = line.substr(0,7).replace(/B/g, '1');
	let seat = line.substr(7,3).replace(/R/g, '1');
	row = convert(row);
	seat = convert(seat);
	let id = row * 8 + seat;
	list[id] = 1;
	max = Math.max(max, id);
}

console.log('1:', max);

for (let i = 80; i< list.length; i++) {
	if (!list[i]) {
		console.log('2:', i);
	}
}
