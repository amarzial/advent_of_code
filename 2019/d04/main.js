const N_MIN = 156218;
const N_MAX = 652527;

function is_valid(code) {
	code = String(code);
	let prev = 0;
	let amount = [];
	let valid = true;
	for (let i = 0; i < code.length; i++) {
		valid = code[i] >= prev;
		prev = code[i];
		amount[code[i]] = (amount[code[i]] | 0) + 1;
		if (!valid) return false;
	}
	for (let i of amount) {
		if (i >= 2) return true;
	}
	return false;
}

function is_valid2(code) {
	code = String(code);
	let prev = 0;
	let amount = [];
	let valid = true;
	for (let i = 0; i < code.length; i++) {
		valid = code[i] >= prev;
		prev = code[i];
		amount[code[i]] = (amount[code[i]] | 0) + 1;
		if (!valid) return false;
	}
	for (let i of amount) {
		if (i == 2) return true;
	}
	return false;
}

let cnt = 0;
let cnt2 = 0;
for (let n = N_MIN; n <= N_MAX; n++) {
	cnt += is_valid(n) ? 1 : 0;
	cnt2 += is_valid2(n) ? 1 : 0;
}
console.log("part 1:", cnt);
console.log("part 2:", cnt2);