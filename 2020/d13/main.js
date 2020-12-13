const fs = require('fs');

const file = fs.readFileSync(process.argv[2], 'ascii').split('\n');

const departure = parseInt(file[0]);
const lines = file[1].split(',').map(l => (parseInt(l) || null));

let minWait = null;
let minId = 0;
for (let line of lines) {
    if (departure % line == 0) {
        minWait = 0;
        minId = line;
        break;
    } else {
        let diff = (Math.ceil(departure / line) * line) - departure;
        if (diff < minWait || minWait === null) {
            minWait = diff;
            minId = line;
        }
    }
}

console.log('1:', minId * minWait);

let gcd = (x, y) => {
    let _gcd = (a, b) => (b === 0 ? a : _gcd(b, a % b)),
        abs = Math.abs;
    return _gcd(abs(x), abs(y));
}

let lcm = (x, y) =>
    x === 0 || y === 0 ? 0 : Math.abs(Math.floor(x / gcd(x, y)) * y);

function findCommon(start, next, increment, offset) {
    let times = 1;
    while ((start + (increment * times) + offset) % next != 0) {
        times++;
    }
    return start + (increment * times);
}

let start = 0;
let increment = 1;
let out;

for (let i = 0; i < lines.length; i++) {
    if (!lines[i]) { continue; }
    let n = findCommon(start, lines[i], increment, i);
    increment *= lines[i];
    start = n;
    out = n;
}

console.log('2:', out);
