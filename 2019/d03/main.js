const fs = require('fs');
let path1 = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51"; 
let path2 = "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"; 

let file = fs.readFileSync('input.txt', {encoding: 'ascii'});
[path1, path2] = file.split('\n');

let grid = new Map();
let steps = new Map();

let re = /(\w)(\d+)/g;

let coord = {x: 0, y: 0};
let modifier = {L: {x: -1, y: 0},
				R: {x: 1, y: 0},
				U: {x: 0, y: 1},
				D: {x: 0, y: -1}}

let cnt = 1;
while (m = re.exec(path1)) {
	for (let i = 0; i < m[2]; i++) {
		coord.x = coord.x + modifier[m[1]].x;
		coord.y = coord.y + modifier[m[1]].y;
		let hash = coord.x + ':' + coord.y;
		grid.set(hash, 1);
		if (!steps.has(hash)) {
			steps.set(hash, {a: cnt});
		}
		cnt++;
	}
}

cnt = 1;
coord = {x: 0, y: 0};
while (m = re.exec(path2)) {
	for (let i = 0; i < m[2]; i++) {
		coord.x = coord.x + modifier[m[1]].x;
		coord.y = coord.y + modifier[m[1]].y;
		let hash = coord.x + ':' + coord.y;
		if (grid.has(hash)) {
			grid.set(hash, grid.get(hash) | 2);
		}
		if (steps.has(hash) && !steps.get(hash).b) {
			steps.set(hash, {a: steps.get(hash).a, b: cnt});
			console.log(steps.get(hash))
		}
		cnt++;
	}
}

let coords = Array.from(grid.keys()).filter((a) => {return grid.get(a) == 3;});
coords.sort(function (a,b) {
	a = a.split(':').reduce((x,y) => Math.abs(parseInt(x)) + Math.abs(parseInt(y)), 0);
	b = b.split(':').reduce((x,y) => Math.abs(parseInt(x)) + Math.abs(parseInt(y)), 0);
	return a - b;
});
console.log(Math.abs(coords[0].split(':').reduce((a,b) => Math.abs(parseInt(a)) + Math.abs(parseInt(b)), 0)));

coords = Array.from(steps.keys()).filter((a) => {return steps.get(a).a && steps.get(a).b;});
coords.sort(function (a,b) {
	a = steps.get(a).a + steps.get(a).b
	b = steps.get(b).a + steps.get(b).b
	return a - b;
});
console.log(steps.get(coords[0]).a + steps.get(coords[0]).b);