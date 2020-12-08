const fs = require('fs');
let lines = fs.readFileSync('./input.txt','utf-8').split("\n");

lines.pop();
lines.sort()

let guards = new Map();
let guard = null;
for (let i = 0; i < lines.length; i++) {
	let line = lines[i];
	if (!isNaN(n = parseInt(line.substring(26)))) {
		if (!guards.has(n)) {
			guards.set(n, []);
		}
		guard = guards.get(n);
		guard.push([]);
		guard = guard[guard.length - 1];
		
		while (i + 2 < lines.length && lines[i + 1].indexOf("Guard") == -1) {
			let info = [];
			line = lines[++i];
			let min = parseInt(line.substring(15));
			info[0] = min;
			line = lines[++i];
			min = parseInt(line.substring(15));
			info[1] = min;
			guard.push(info);
		}
	}
}

let max = 0, id = 0;
guards.forEach((v, k , map) => {
	let amount = v.reduce((total, current) => {
		return total + current.reduce((total, current) => {
			return total + (current[1] - current[0]);
		}, 0)
	}, 0);
	if (amount > max) {
		max = amount;
		id = k;
	}
});
let guardId = id;
let target = guards.get(id);
let minutes = new Array(60).fill(0);
target.forEach((v,k,e) => {
	v.forEach((v,k,e) => {
		for (let i = v[0]; i < v[1]; i++) {
			minutes[i]++;
		}
	});
});
max = 0;
id = 0;
minutes.forEach((v,k,e) => {
	if (v > max) {
		id = k;
		max = v;
	}
});
console.log(guardId * id);

//part 2
//best of each guard
bestMinutes = {};
guards.forEach((v, k , map) => {
	let tmp = new Array(60).fill(0);
	for (let day of v) {
		for (let interval of day) {
			for (let i = interval[0]; i < interval[1]; i++) {
				tmp[i]++;
			}
		}
	}
	let best = -1;
	let val = -1;
	for (let i = 0; i < tmp.length; i++) {
		if (tmp[i] > val) {
			best = i;
			val = tmp[i];
		}
	}
	bestMinutes[k] = {minute: best, count: val};
});
max = {guard: -1, val: -1, minute: -1};
for (let k of Object.keys(bestMinutes)) {
	let v = bestMinutes[k].count;
	let min = bestMinutes[k].minute;
	if (v > max.val) {
		max.val = v;
		max.guard = parseInt(k);
		max.minute = min;
	}
}
console.log(max.guard * max.minute);