class Fabric {
	constructor() {
		this.coords = new Map();
	}
	process(id,startX,startY,width,height) {
		for (let j = 0; j < height; j++) {
			for (let i = 0; i < width; i++) {
				let index = `${i + +startX}x${j + +startY}`;
				let val = this.coords.get(index) || [];
				val.push(id);
				this.coords.set(index, val)
			}
		}
	}
	overlap(id,startX,startY,width,height) {
		for (let j = 0; j < height; j++) {
			for (let i = 0; i < width; i++) {
				let index = `${i + +startX}x${j + +startY}`;
				let val = this.coords.get(index) || [];
				if (val.length > 1) {
					return true;
				}
			}
		}
		return false;
	}
}

let f = new Fabric();
const fs = require('fs');
let lines = fs.readFileSync('./input.txt','utf-8').split("\n");
let expr = /#(\d+) @ (\d+),(\d+): (\d+)x(\d+)/;
for (let line of lines) {
	let res = expr.exec(line);
	if (res) {
		f.process(...res.slice(1,6));
	}
}
let it = f.coords.values();
let cnt = 0;
while (!(i = it.next()).done) {
	cnt += i.value.length > 1 ? 1 : 0;
}
console.log(cnt);

//part 2
for (let line of lines) {
	let res = expr.exec(line);
	if (res) {
		if (!f.overlap(...res.slice(1,6))) {
			console.log(line, "doesn't overlap");
		}
	}
}