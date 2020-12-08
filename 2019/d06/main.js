const fs = require('fs');
let file = fs.readFileSync('input.txt', {encoding: 'ascii'});
// file = `COM)B
// B)C
// C)D
// D)E
// E)F
// B)G
// G)H
// D)I
// E)J
// J)K
// K)L
// K)YOU
// I)SAN`;
let lines = file.split('\n');

let map = {}
function orbits(item) {
	if (map[item] == 'COM') return 1;
	return 1 + orbits(map[item]);
}

function path(item, target) {
	let pos1 = item;
	let steps1, steps2;
	steps1 = 0;
	while (map[pos1] != 'COM') {
		pos1 = map[pos1];
		let pos2 = target;
		steps1 ++;
		steps2 = 0;
		while (map[pos2] != 'COM') {
			pos2 = map[pos2];
			steps2++;
			if (pos1 == pos2) {
				return steps1 + steps2;
			}
		}
	}
	return false;
}

for (let line of lines) {
	let orbit = /([a-zA-Z0-9]+)\)([a-zA-Z0-9]+)/.exec(line);
	map[orbit[2]] = orbit[1];
}

let count = 0;
for (let k in map) {
	count += orbits(k);
}
console.log('Part 1:', count);

for (let k in map) {
	count += orbits(k);
}
console.log('Part 2:', path('YOU', 'SAN') - 2);