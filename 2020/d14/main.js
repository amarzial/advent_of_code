const fs = require('fs');

const lines = fs.readFileSync(process.argv[2], 'ascii').split('\n');
lines.pop();

let mem = {};
let mem2 = {};

function writeMem(address, value, mask) {
    if (mask) {
        mem[address] = (BigInt(value) | mask[0]) & ~mask[1];
    } else {
        mem[address] = value;
    }
}

function* part2address(address, mask) {
    let addr = BigInt(address) | mask[0];
    let len = BigInt(mask[2].length);
    const mappings = [];

    for (let i = 0n; i < mask[2].length; i++) {
        if (mask[2][len - 1n - i] == 'X') {
            mappings.push(i);
        }
    }
    for (let n = 0n; n < 2n ** BigInt(mappings.length); n++) {
        let tmp = addr;
        for (let mi = 0; mi < mappings.length; mi++) {
            let m = mappings[mi];
            let current = 1n << m;
            if (n & (1n << BigInt(mi))) {
                tmp |= current;
            } else {
                tmp &= ~current;
            }
        }
        yield tmp;
    }
    if (!mappings.length) return address;
}

function writeMemPart2(address, value, mask) {
    for (let addr of part2address(address, mask)) {
        mem2[addr] = BigInt(value);
    }
}

function readMask(m) {
    let m1 = 0n;
    let m2 = 0n;
    let len = BigInt(m.length);
    for (let i = 0n; i < m.length; i++) {
        if (m[len - 1n - i] == '1') {
            m1 |= (1n << i);
        } else if (m[len - 1n - i] == '0') {
            m2 |= (1n << i);
        }
    }
    return [m1, m2, m];
}

let mask = [0, 0, ''];

for (let line of lines) {
    if (line.startsWith('mask')) {
        mask = readMask(line.substring(7));
    } else {
        let match = /mem\[(\d+)\] = (\d+)/.exec(line);
        writeMem(match[1], match[2], mask);
        writeMemPart2(match[1], match[2], mask);
    }
}

const s1 = Object.values(mem).reduce((t, cur) => { return t + cur; }, 0n);
console.log('1:', parseInt(s1.toString()));

const s2 = Object.values(mem2).reduce((t, cur) => { return t + cur; }, 0n);
console.log('2:', parseInt(s2.toString()));
