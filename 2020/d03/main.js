const fs = require('fs');

const map = fs.readFileSync(process.argv[2], 'ascii').split('\n');
map.pop();
const width = map[0].length;

let count = [
  [ 3, 1 ],
  [ 1, 1 ],
  [ 5, 1 ],
  [ 7, 1 ],
  [ 1, 2 ],
];

function run(slope) {
  let [x, y] = [ 0, 0 ];
  let count = 0;
  do {
    count += map[y][x % width] == '#' ? 1 : 0;
    y += slope[1];
    x += slope[0];
  } while (y < map.length)
  return count;
}

count = count.map(run);

console.log('1:', count[0]);
console.log('2:', count.reduce((t, c) => t * c));
