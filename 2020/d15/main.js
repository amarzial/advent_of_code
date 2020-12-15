const fs = require('fs');

const list = fs.readFileSync(process.argv[2], 'ascii').replace('\n', '').split(',').map(l => parseInt(l));

const history = new Map();

let last = 0;
let n;
let prev;
for (let i = 0; i < 30000000; i++) {
    if (list[i] === undefined) {
        prev = history.get(last);
        if (prev[0] === null) {
            n = 0;
        } else {
            n = prev[1] - prev[0];
        }
    } else {
        n = list[i];
    }
    last = n;

    if (!history.has(n)) { history.set(n, [null, null]); }
    let elem = history.get(n);
    elem[0] = elem[1];
    elem[1] = i;

    if (i == 2020 - 1) {
        console.log('1:', last);
    }
}

console.log('2:', last);
