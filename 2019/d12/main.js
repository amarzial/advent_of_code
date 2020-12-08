const fs = require('fs');
	
let file = fs.readFileSync('input.txt', {encoding: 'ascii'});

// file = `<x=-1, y=0, z=2>
// <x=2, y=-10, z=-7>
// <x=4, y=-8, z=8>
// <x=3, y=5, z=-1>`;

let moons = []
for (let line of file.split('\n')) {
	let res = /<x=(-?\d+), y=(-?\d+), z=(-?\d+)>/.exec(line);
	moons.push({
		position: {x: parseInt(res[1]), y: parseInt(res[2]), z: parseInt(res[3])},
		velocity: {x: 0, y: 0, z: 0}
	});
}

function calcGravity(moons) {
	for (let i = 0; i < moons.length; i++) {
		let next = JSON.parse(JSON.stringify(moons[i].velocity));
		for (let j = 0; j < moons.length; j++) {
			let x = moons[i].position.x - moons[j].position.x;
			let y = moons[i].position.y - moons[j].position.y;
			let z = moons[i].position.z - moons[j].position.z;
			next.x -= x >= 1 ? 1 : (x <= -1 ? -1 : 0);
			next.y -= y >= 1 ? 1 : (y <= -1 ? -1 : 0);
			next.z -= z >= 1 ? 1 : (z <= -1 ? -1 : 0);
		}
		moons[i].next = next;
	}
}

function applyVelocity(moons) {
	for (let i = 0; i < moons.length; i++) {
		moons[i].position.x += moons[i].next.x;
		moons[i].position.y += moons[i].next.y;
		moons[i].position.z += moons[i].next.z;
		moons[i].velocity = moons[i].next;
	}
}

function totalEnergy(moons) {
	let energy = 0;
	for (let i = 0; i < moons.length; i++) {
		let potential = Math.abs(moons[i].position.x) + Math.abs(moons[i].position.y) + Math.abs(moons[i].position.z);
		let kinetik = Math.abs(moons[i].velocity.x) + Math.abs(moons[i].velocity.y) + Math.abs(moons[i].velocity.z);
		energy += potential * kinetik;
	}
	return energy;
}

	// console.log(moons);
for (let i = 0; i < 1000; i++) {
	calcGravity(moons);
	applyVelocity(moons);
	// console.log(moons);
}

console.log(totalEnergy(moons));

//part2