const intcode = require('../libs/intcode.js');
const fs = require('fs');

let file = fs.readFileSync('input.txt', {encoding: 'ascii'});

// file ="109,40,203,0,204,0,99";

let ic = new intcode.IntCode(file);
ic.input = () => {return 1;};
ic.run();

//part 2
ic = new intcode.IntCode(file);
ic.input = () => {return 2;};
ic.run();
