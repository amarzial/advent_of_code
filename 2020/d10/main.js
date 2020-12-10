const fs = require('fs');

const lines = fs.readFileSync(process.argv[2], 'ascii').split('\n').map(l => parseInt(l));

lines.unshift(0);
lines.sort((a, b) => a - b);
lines[lines.length - 1] = lines[lines.length - 2] + 3;

let current = 0;
let count1 = 0;
let count3 = 0;
for (let i = 0; i < lines.length; i++) {
	let diff = lines[i] - current;
	if (diff == 1) count1++;
	if (diff == 3) count3++;
	current = lines[i];
}

console.log('1:', count1 * count3);

let cache = {};

function combinazioni(i, level) {
	// console.log('  '.repeat(level) + lines[i])
	if (i == lines.length - 1) return 1;
	if (i >= lines.length) return 0;
	let count = 0;
	for (let j = 1; (lines[i + j] - lines[i] <= 3); j++) {
		let out;
		if (cache[i + j]) {
			out = cache[i + j];
		} else {
			out = combinazioni(i + j, level + 1);
			cache[i + j] = out;
		}
		count += out;
	}
	return count;
}

lines.pop();
let time = Date.now();
console.log('2:', combinazioni(0, 0));