var readlineSync = require('readline-sync');

class IntCode {
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
			9: (a) => { this.relativeBase += a.value; },
			99: () => {},
		}
	}

	access(offset, mode, value = null) {
		if (!mode) {
			offset = this.code[offset];
		} else if (mode == 1) {
			offset = value !== null ? this.code[offset] : offset;
		} else if (mode == 2) {
			offset = this.relativeBase + this.code[offset];
		}
		if (value === null) {
			return this.code[offset] || 0;
		} else {
			// console.log(`writing ${value} at ${offset} with mode ${mode}, relative base: ${this.relativeBase}`);
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
		this.done = false;
		this.position = 0;
		this.relativeBase = 0;
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
		// console.log("opcode", opcode, "modes", modes, "params", parms, "args", args);
		for (let i = 0; i < fnc.length; i++) {
			if (args[i].out !== undefined) {
				this.access(this.position + 1 + i, modes[i], args[i].out);
			}
		}

		// console.log(this.code);
		if (jump !== undefined) {
			this.position = jump;
		} else {
			this.position += 1 + fnc.length;
		}
		return !!fnc.length;
	}

	run() {
		this.running = true;
		while (this.running && !(this.done = !this.step()));
		return this.done;
	}

	pause() {
		this.running = false;
	}
};

exports.IntCode = IntCode;