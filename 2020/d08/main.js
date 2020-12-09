const fs = require('fs');

const instruction = {
  jmp : function(prog, arg) { return parseInt(arg); },
  nop : function(prog, arg) { return null; },
  acc : function(prog, arg) {
    prog.register += arg;
    return null;
  }
}

let program = fs.readFileSync(process.argv[2], 'ascii')
                .split('\n').map((l) => {
                  if (!l) {return null;}
                  let [_, cmd, arg] = l.match(/(\w+) ((\+|\-)\d+)/);
                  return [cmd, arg];
                });

function run(replace = null) {
  const lines = program.map(l => { return {line : l, count : 0}; });
  let prog = {register: 0};
  let ip = 0;
  let stop = false;
  while (!stop) {
    if (ip < 0 || ip >= program.length - 1) {
      return [true, prog.register];
    }
    let line = lines[ip];
    line.count++;
    if (line.count > 1) {
      return [false, prog.register];
    }
    // console.log(program.length,ip, line);
    let [cmd, arg] = line.line;
    if (replace && replace[ip]) {
      cmd = replace[ip];
    }
    let next = instruction[cmd](prog, parseInt(arg));
    ip += (next === null) ? 1 : next;
  }
}

let [status, register] = run();
console.log('1:', register);
for (let i = 0; i < program.length; i++) {
  let line = program[i];
  let replace = false;
  if (line[0] == 'jmp') {
    replace = {[i]: 'nop'};
  } else if ((line[0] == 'nop') && (parseInt(line[1]) != 0)){
    replace = {[i]: 'jmp'};
  }
  let [status, register] = run(replace);
  if (status) {
    console.log('2:', register);
    break;
  }
}