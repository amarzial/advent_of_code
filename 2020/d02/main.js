const utils = require('../utils.js');

function checkPolicy(row) {
  let match = /(\d+)-(\d+) (\w): (.+)/.exec(row);
  let min = parseInt(match[1]);
  let max = parseInt(match[2]);
  let letter = match[3];
  let password = match[4];

  let cnt = 0;
  for (let c of password) {
    cnt += c == letter ? 1 : 0;
  }
  return cnt >= min && cnt <= max;
}

function checkPolicy2(row) {
  let match = /(\d+)-(\d+) (\w): (.+)/.exec(row);
  let min = parseInt(match[1]);
  let max = parseInt(match[2]);
  let letter = match[3];
  let password = match[4];

  return (password[min - 1] == letter) != (password[max - 1] == letter);
}

let cnt = 0;
let cnt2 = 0;
for (let line of utils.readLines(process.argv[2])) {
  cnt += checkPolicy(line) ? 1 : 0;
  cnt2 += checkPolicy2(line) ? 1 : 0;
}
console.log('1:', cnt);
console.log('2:', cnt2);