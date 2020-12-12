const utils = require('../utils.js');

const required = {
  byr: v => (v >= 1920 && v <= 2002),
  iyr: v => (v >= 2010 && v <= 2020),
  eyr: v => (v >= 2020 && v <= 2030),
  hgt: v => {
    let val = parseInt(v);
    if (v.indexOf('in') != -1) {
      return val >= 59 && val <= 76;
    } else if (v.indexOf('cm') != -1) {
      return val >= 150 && val <= 193;
    }
    return false;
  },
  hcl: v => /^#[0-9a-f]{6}$/.test(v),
  ecl: v =>
    ({ amb: 1, blu: 1, brn: 1, gry: 1, grn: 1, hzl: 1, oth: 1 }[v]),
  pid: v => /^\d{9}$/.test(v),
};

function check(passport, strict) {
  for (let [k, v] of Object.entries(required)) {
    if (passport[k] === undefined) {
      return false;
    } else if (strict && !v(passport[k])) {
      // console.log(k, passport[k], 'fail');
      return false;
    }
  }
  return true;
}

let count = 0;
let count2 = 0;
for (let elem of utils.readLines(process.argv[2], '\n\n')) {
  const regex = RegExp('(\\w{3}):(\\S+)', 'g');
  const passport = Array.from(elem.matchAll(regex))
    .reduce((a, b) => (a[b[1]] = b[2], a), {});
  count += check(passport, false) ? 1 : 0;
  count2 += check(passport, true) ? 1 : 0;
}
console.log('1:', count);
console.log('2:', count2);
