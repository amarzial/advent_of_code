const fs = require('fs');
var readlineSync = require('readline-sync');

let file = fs.readFileSync('input.txt', {encoding: 'ascii'});

class Intcode {
	constructor(str, noun = null, verb = null) {
		this.program = str;
		this.setup(noun, verb);
		this.instructions = {
			1: (a,b,c) => { c.out = a.value + b.value; },
			2: (a,b,c) => { c.out = a.value * b.value; },
			3: (a) => { a.out = this.input(); },
			4: (a) => { this.output(a.value); },
			5: (a,b) => { return a.value ? b.value : undefined; },
			6: (a,b) => { return !a.value ? b.value : undefined; },
			7: (a,b,c) => { c.out = a.value < b.value ? 1 : 0; },
			8: (a,b,c) => { c.out = a.value == b.value ? 1 : 0; },
			99: () => {},
		}
	}

	access(offset, mode, value = null) {
		if (!mode || value !== null) {
			offset = this.code[offset];
		} else if (mode == 1) {
		}
		if (value === null) {
			return this.code[offset];
		} else {
			this.code[offset] = value;
			return;
		}
	}

	input() {
		let value = readlineSync.question("Input: ");
		return parseInt(value);
	}

	output(out) {
		process.stdout.write(String(out) + '\n');
	}

	setup(noun, verb) {
		this.code = [];
		this.position = 0;
		let re = /(-?\d+)/g;
		let m = null;
		while (m = re.exec(this.program)) {
			this.code.push(parseInt(m[1]));
		}
		if (noun !== null) this.code[1] = noun;
		if (verb !== null) this.code[2] = verb;
	}

	step() {
		let opcode = this.code[this.position];
		let modes = String(parseInt(this.code[this.position] / 100)).split('').reverse().map((a) => {return parseInt(a);});
		opcode = opcode % 100;
		let fnc = this.instructions[opcode];
		let args = [];

		if (!fnc) {
			console.log("fail", opcode, this.position);
			for (let i = 0; i < this.code.length; i++) {
				console.log(i, this.code[i]);
			}
			return false;
		}
		modes.length = fnc.length;

		for (let i = 0; i < fnc.length; i++) {
			args.push({value: this.access(this.position + 1 + i, modes[i])});
		}
		// let parms = this.code.slice(this.position + 1, this.position + 1 + fnc.length);
		let jump = fnc(...args);
		// console.log(opcode, modes, parms, args);
		for (let i = 0; i < fnc.length; i++) {
			if (args[i].out !== undefined) {
				this.access(this.position + 1 + i, modes[i], args[i].out);
			}
		}

		if (jump !== undefined) {
			this.position = jump;
		} else {
			this.position += 1 + fnc.length;
		}
		return !!fnc.length;
	}

	run() {
		while (this.step());
	}
};

// let pc = new Intcode("3, 5, 4, 5, 99, 0");
let pc = new Intcode(file);
pc.run();
// console.log(pc.code);

