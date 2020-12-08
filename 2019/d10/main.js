const fs = require('fs');
let file = fs.readFileSync('input.txt', {encoding: 'ascii'});

// file = `.#....#####...#..
// ##...##.#####..##
// ##...#...#.#####.
// ..#.....X...###..
// ..#.#.....#....##`

let lines = file.trim().split('\n');

const WIDTH = lines[0].length;
const HEIGHT = lines.length;

let map = lines.join('');

function detect(map, ref_x, ref_y) {
	let angles = {};
	for (let y = 0; y < HEIGHT; y++) {
		for (let x = 0; x < WIDTH; x++) {
			let a = Math.atan2(y - ref_y, x - ref_x);
			if (map[x + y * WIDTH] == '#' && (x != ref_x || y != ref_y)) {
				angles[a] = 1;
			}
		}
	}
	return Object.entries(angles).length;
}

function dist(x1, y1, x2, y2) {
	return Math.sqrt(Math.pow(x1 - x2, 2) + Math.pow(y1 - y2, 2));
}

function rotation(angle) {
	if (angle > 0) {
		return  (angle <= Math.PI / 2) ? Math.PI / 2 - angle : (Math.PI - angle + (Math.PI / 2 * 3));
	} else {
		return (-angle) + (Math.PI / 2);
	}
}

function vaporize(map, ref_x, ref_y) {
	map = map.split('');
	let cnt = 0;

	let angles;
	let rays;
	do {
		angles = {}
		for (let y = 0; y < HEIGHT; y++) {
			for (let x = 0; x < WIDTH; x++) {
				let a = Math.atan2(ref_y - y, x - ref_x);
				if (map[x + y * WIDTH] == '#' && (x != ref_x || y != ref_y)) {
					angles[a] = angles[a] ? (dist(x, y, ref_x, ref_y) < dist(angles[a].x, angles[a].y, ref_x, ref_y) ? {x, y} : angles[a]) : {x, y};
				}
			}
		}
		rays = Object.keys(angles)
		rays.sort((a, b) => {return rotation(a) - rotation(b);});
		for (let k of rays) {
			cnt++;
			// console.log("vaporize", cnt, "=>", angles[k].x, angles[k].y);
			map[angles[k].x + angles[k].y * WIDTH] = '.';
		}
	} while (cnt < 200 && Object.entries(angles).length != 0);
	return angles[rays[rays.length - 1 - (cnt - 200)]] || null;
}

// let out = [];
let max = {}
for (let y = 0; y < HEIGHT; y++) {
	for (let x = 0; x < WIDTH; x++) {
		if (map[x + y * WIDTH] != '#') {continue;}
		let cnt = detect(map, x, y);
		// out[x + y * WIDTH] = cnt;
		if (cnt > (max.cnt || 0)) {max.cnt = cnt; max.coord = {x, y};};
	}
}

console.log("Part 1:", max.cnt);
// console.log(out);

//part 2
let info = vaporize(map, max.coord.x, max.coord.y);
if (info) {
	console.log("Part 2:", info.x * 100 + info.y);
}
// let info = vaporize(map, max.coord.x, max.coord.y);
// console.log("Part 2:", info.x * 100 + info.y);
