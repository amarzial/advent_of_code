const fs = require('fs');

const lines =
    fs.readFileSync(process.argv[2], 'ascii').split('\n').map(n => parseInt(n));
lines.pop();

function isValid(number, position) {
  for (let i = position - preamble_size; i < position; i++) {
    for (let j = i + 1; j < position; j++) {
      if (lines[i] + lines[j] == number) {
        return true;
      }
    }
  }
  return false;
}

const preamble_size = 25;
let out = null;
for (let i = preamble_size; i < lines.length; i++) {
  let valid = isValid(lines[i], i);
  if (!valid) {
    console.log('1:', lines[i]);
    out = lines[i];
    break;
  }
}

part2: {
  for (let i = 0; i < lines.length; i++) {
    let sum = lines[i];
    let min = sum;
    let max = sum;
    for (let j = i + 1; j < lines.length; j++) {
      sum += lines[j];
      min = Math.min(min, lines[j]);
      max = Math.max(max, lines[j]);
      if (sum == out) {
        console.log('2:', min + max);
        break part2;
      }
      if (sum > out) {
        break;
      }
    }
  }
};
