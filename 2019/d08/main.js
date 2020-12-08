const fs = require('fs');
let file = fs.readFileSync('input.txt', {encoding: 'ascii'});
// file = "123456789012";
file = file.trim();
const IMG_WIDTH = 25;
const IMG_HEIGHT = 6;

let layers = [];
for (let i = 0; i < file.length; i++) {
	let idx = parseInt(i / (IMG_WIDTH * IMG_HEIGHT));
	if (!layers[idx]) {
		layers[idx] = Array(IMG_WIDTH * IMG_HEIGHT);
	}
	layers[idx][((i % IMG_WIDTH) + parseInt(i / IMG_WIDTH) * IMG_WIDTH) % (IMG_WIDTH * IMG_HEIGHT)] = file[i];
}

function count(arr, what) {
	return arr.reduce((cnt, elem) => {return cnt + (elem == what ? 1 : 0);}, 0);
}

let match = null;
let lowest = null;
for (let i = 0; i < layers.length; i++) {
	let cnt = count(layers[i], 0);
	if (lowest === null || cnt < lowest) {
		lowest = cnt;
		match = i;
	}
}

console.log(count(layers[match], 1) * count(layers[match], 2));

//part2
layers.reverse();
let image = layers.reduce((res, current) => {
	for (let i = 0; i < current.length; i++) {
		if (current[i] != 2) {
			res[i] = current[i];
		}
	}
	return res;
}, layers[0]);

function print(img) {
	let row = '';
	for (let i = 0; i < img.length; i++) {
		row += parseInt(img[i]) ? '#' : ' ';
		if (row.length == IMG_WIDTH) {console.log(row); row = ''};
	}
}

print(image);