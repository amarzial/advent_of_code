const fs = require('fs');

const file = fs.readFileSync(process.argv[2], 'ascii').split('\n');

function getInfo(string) {
  let open = false;
  let abba = undefined;
  let bab_found = undefined;
  let bab_info = [ {}, {} ];
  for (let i = 0; i < string.length; i++) {
    let s4 = string.substring(i, i + 3);
    if (s4.indexOf('[') != -1) {
      open = true;
      continue;
    } else if (s4.indexOf(']') != -1) {
      open = false;
      continue;
    }
    let ab = string.substring(i, i + 2);
    let ba = string.substring(i + 2, i + 4).split('').reverse().join('');
    let bab = string.substring(i, i + 3);

    let found = ab == ba;
    if (open && found) {
      abba = false;
    }
    if (bab[0] == bab[2] && bab[0] != bab[1]) {
      if (bab_info[open ? 0 : 1][bab]) {
        // console.log(bab, bab_info);
        bab_found = true;
      }
      bab = bab.split('');
      let b = bab[0];
      let a = bab[1];
      bab[0] = bab[2] = a;
      bab[1] = b;
      bab = bab.join('');

      if (!bab_found) {
        bab_info[open ? 1 : 0][bab] = true;
      }
    }

    if (found && !open && abba === undefined) {
      abba = true;
    }
  }
  return {ssl : !!abba, tls : !!bab_found};
}

let count = 0;
let count2 = 0;
for (let line of file) {
  const info = getInfo(line);
  // info.tls && console.log(line, info.tls);
  count += info.ssl ? 1 : 0;
  count2 += info.tls ? 1 : 0;
}

console.log('1:', count);
console.log('2:', count2);
