const intcode = require('../libs/intcode.js');
const fs = require('fs');
	
let file = fs.readFileSync('input.txt', {encoding: 'ascii'});

class Robot {
	constructor(part2 = false) {
		this.position = {x: 0, y: 0};
		this.direction = 0;
		this.grid = {};
		this.inCnt = 0;
		if (part2) {
			this.write(1);
		}
	}

	read(coords = null) {
		let x = coords ? coords.x : this.position.x
		let y = coords ? coords.y : this.position.y
		return this.grid[x + ',' + y] || 0;
	}

	write(color) {
		this.grid[this.position.x + ',' + this.position.y] = color;
	}

	print() {
		let min = {x: 0, y: 0}, max = {x: 0, y: 0};
		for (let v in this.grid) {
			let coords = v.split(',');
			min.x = Math.min(coords[0], min.x);
			min.y = Math.min(coords[1], min.y);
			max.x = Math.max(coords[0], max.x);
			max.y = Math.max(coords[1], max.y);
		}
		for (let h = min.y; h <= max.y; h++) {
			let line = '';
			for (let w = max.x; w >= min.x; w--) {
				line += this.read({x: w, y: h}) ? '#' : ' ';
			}
			console.log(line);
		}
	}

	turn(mode) {
		let amount = mode ? -1 : 1;
		switch (this.direction) {
			case 0:
				this.position.x += amount;
			break;
			case 1:
				this.position.y += amount;
			break;
			case 2:
				this.position.x -= amount;
			break;
			case 3:
				this.position.y -= amount;
			break;
		}
		this.direction += amount;
		this.direction = this.direction < 0 ? 3 : (this.direction > 3 ? 0 : this.direction);
	}

	input(val) {
		this.inCnt++;
		if (this.inCnt % 2) {
			this.write(val);
		} else {
			this.turn(val);
		}
	}
}

let r = new Robot();

let ic = new intcode.IntCode(file);
ic.output = function(val) {r.input(val);}
ic.input = function() {return r.read();}
ic.run();

console.log(Object.entries(r.grid).length);

//part 2
let r2 = new Robot(true);

let ic2 = new intcode.IntCode(file);
ic2.output = function(val) {r2.input(val);}
ic2.input = function() {return r2.read();}
ic2.run();
r2.print();
