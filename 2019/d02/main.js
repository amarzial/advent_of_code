const fs = require('fs');
let file = fs.readFileSync('input.txt', {encoding: 'ascii'});

class Intcode {
	constructor(str, noun = null, verb = null) {
		this.program = str;
		this.setup(noun, verb);
		this.instructions = {
			1: (offset) => { this.pointer(offset + 3, this.pointer(offset + 1) + this.pointer(offset + 2)); return 4; },
			2: (offset) => { this.pointer(offset + 3, this.pointer(offset + 1) * this.pointer(offset + 2)); return 4; },
			99: (offset) => { return null; }
		}
	}

	pointer(offset, value = null) {
		if (value === null) {
			return this.code[this.code[offset]];
		} else {
			this.code[this.code[offset]] = value;
			return;
		}
	}

	setup(noun, verb) {
		this.code = [];
		this.position = 0;
		let re = /(\d+)/g;
		let m = null;
		while (m = re.exec(this.program)) {
			this.code.push(parseInt(m[1]));
		}
		if (noun !== null) this.code[1] = noun;
		if (verb !== null) this.code[2] = verb;
	}

	step() {
		let opcode = this.code[this.position];
		let offset = this.instructions[opcode](this.position);
		this.position += offset;
		return offset !== null;
	}

	first() {
		return this.code[0];
	}

	run() {
		while (this.step());
		return this.first();
	}
};

let pc = new Intcode(file, 12, 2);
// let pc = new Intcode("1,1,1,4,99,5,6,0,99");
console.log(pc.run());