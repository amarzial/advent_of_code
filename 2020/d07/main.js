const utils = require('../utils.js');

function parseBag(line) {
  const match = line.match(/(.*?) bags contain /);
  const content = line.substring(match[1].length);
  const regex = /(\d+|no) (.*?) bags?./g;
  let m;
  let bags = {};
  while (m = regex.exec(content)) {
    if (parseInt(m[1])) {
      bags[m[2]] = parseInt(m[1]);
    }
  }
  return [ match[1], bags ];
}

function findGold(bags, bagName, level) {
  // console.log(' '.repeat(2 * level) + bagName);
  const bag = bags[bagName];
  if (!bag) {
    return false;
  }
  if (bag['shiny gold']) {
    return true;
  } else {
    for (let b of Object.keys(bag)) {
      let g = findGold(bags, b, level + 1);
      if (g) {
        return true;
      }
    }
  }
  return false;
}

function countBags(bags, bagName) {
  const bag = bags[bagName];
  if (!bag) {
    return 0;
  }
  let count = 0;
  for (let [k, v] of Object.entries(bag)) {
    // console.log(k, v);
    count += v;
    count += v * countBags(bags, k);
  }
  return count;
}

const bags = {};
for (let line of utils.readLines(process.argv[2])) {
  let [name, content] = parseBag(line);
  bags[name] = content;
}

let count = 0;
for (let bag of Object.keys(bags)) {
  count += findGold(bags, bag, 0) ? 1 : 0;
}
console.log('1:', count);
console.log('2:', countBags(bags, 'shiny gold'));
