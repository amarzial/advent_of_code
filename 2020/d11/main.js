const fs = require('fs');

let map = fs.readFileSync(process.argv[2], 'ascii').split('\n');
map.pop();
const width = map[0].length;
const height = map.length;

map = [].concat(...map.map(l => l.split('')));

const check = [[-1, -1], [-1, 0], [-1, 1], [0, -1], [0, 1], [1, -1], [1, 0], [1, 1]];

function print(m) {
    for (let y = 0; y < height; y++) {
        console.log(m.slice(y * width, (y + 1) * width).join(''));
    }
    console.log('');
}

function neighbors(x, y) {
    let count = 0;
    for (let [offset_y, offset_x] of check) {
        let target_x = x + offset_x;
        let target_y = y + offset_y;
        if (target_x < 0 || target_x >= width || target_y < 0 || target_y >= height) {
            continue;
        }
        count += map[target_y * width + target_x] == '#' ? 1 : 0;
    }
    return count;
}

function neighbors2(x, y) {
    let count = 0;
    for (let [offset_y, offset_x] of check) {
        let target_x = x + offset_x;
        let target_y = y + offset_y;
        while (!(target_x < 0 || target_x >= width || target_y < 0 || target_y >= height)) {
            if (map[target_y * width + target_x] != '.') {
                // console.log(x, y, target_x, target_y, map[target_y * width + target_x]);
                count += map[target_y * width + target_x] == '#' ? 1 : 0;
                break;
            }
            target_x += offset_x;
            target_y += offset_y;
        }
    }
    return count;
}

function run(count_fnc, treshold) {
    let map_temp = [...map];
    for (let y = 0; y < height; y++) {
        for (let x = 0; x < width; x++) {
            let c = map[y * width + x];
            if (c == '.') {
                map_temp[y * width + x] = '.';
            } else {
                let n = count_fnc(x, y);
                if (c == 'L' && n == 0) {
                    map_temp[y * width + x] = '#';
                } else if (c == '#' && n >= treshold) {
                    map_temp[y * width + x] = 'L';
                }
            }
        }
    }
    // print(map_temp);
    let old = map;
    map = map_temp;
    return old;
}

function show(n_func) {
    let map_temp = [...map];
    for (let y = 0; y < height; y++) {
        for (let x = 0; x < width; x++) {
            let c = map[y * width + x];
            if (c == '.') {
                map_temp[y * width + x] = '.';
            } else {
                let n = n_func(x, y);
                map_temp[y * width + x] = n;
            }
        }
    }
    // print(map_temp);
    print(map_temp);
}

let start = [...map];
let old;
do {
    old = run(neighbors, 4);
} while (old.join('') != map.join(''));

console.log('1:', map.reduce((t, c) => t + (c == '#' ? 1 : 0), 0));

map = [...start];
do {
    old = run(neighbors2, 5);
    // print(map);
    // show(neighbors2);
    // if (cnt++ > 0) break;
} while (old.join('') != map.join(''));

console.log('2:', map.reduce((t, c) => t + (c == '#' ? 1 : 0), 0));
