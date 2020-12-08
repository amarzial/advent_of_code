const intcode = require('../libs/intcode.js');
const fs = require('fs');

let file = fs.readFileSync('input.txt', {encoding: 'ascii'});
// file ="3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0";
const permutator = (inputArr) => {
  let result = [];

  const permute = (arr, m = []) => {
    if (arr.length === 0) {
      result.push(m)
    } else {
      for (let i = 0; i < arr.length; i++) {
        let curr = arr.slice();
        let next = curr.splice(i, 1);
        permute(curr.slice(), m.concat(next))
     }
   }
 }

 permute(inputArr)

 return result;
}
let possible_phases = [0,1,2,3,4];
let phases = permutator(possible_phases);
let thrust = 0;

for (let phase of phases) {
	let output = 0;
	for (let i = 0; i < phase.length; i++) {
		let ic = new intcode.IntCode(file);
		let gen = (function*() {yield phase[i]; yield output;})();
		ic.input = () => {return gen.next().value;};
		ic.output = (v) => {output = v;};
		ic.run();
	}
	thrust = Math.max(thrust, output);
}
console.log(thrust);

//part 2
// file = "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10";
// ic = new intcode.IntCode(file);
// ic.run();
possible_phases = [5,6,7,8,9];
phases = permutator(possible_phases);
let amps = [];
thrust = 0;
for (let phase of phases) {
	let output = 0;
	for (let i = 0; i < phase.length; i++) {
		amps[i] = new intcode.IntCode(file);
		let gen = (function*() {yield phase[i]; while (true) yield output;})();
		amps[i].input = () => {return gen.next().value;};
		amps[i].output = (v) => {amps[i].pause(); output = v;};
	}

	for (let i = 0; !amps[4].done; i++) {
		amps[i % 5].run();
	}
	thrust = Math.max(thrust, output);
}
console.log(thrust);
