const fs = require('fs');

const content = fs.readFileSync(process.argv[2], 'ascii');

const gruppi = content.split('\n\n');

let count = 0;
let count2 = 0;
for (let gruppo of gruppi) {
  const risposte = {};
  const persone = gruppo.split('\n').filter(a => !!a);
  for (let persona of persone) {
    for (let c of persona) {
      if (risposte[c] === undefined) {
        risposte[c] = 0;
      }
      risposte[c]++;
    }
  }

  count += Object.keys(risposte).length;
  count2 += a = Object.values(risposte).reduce(
      (t, c) => { return t + (c == persone.length ? 1 : 0); }, 0);
}

console.log('1:', count);
console.log('2:', count2);
