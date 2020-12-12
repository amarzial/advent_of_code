const fs = require('fs');

const lines = fs.readFileSync(process.argv[2], 'ascii').split('\n').map(l => {
    return [l.substring(0, 1), parseInt(l.substring(1))];
});
lines.pop();

class Ship {
    constructor(waypoint) {
        this.start = { x: 0, y: 0 };
        this.facing = 1;
        this.position = { x: 0, y: 0 };
        this.directions = 'NESW';
        this.waypoint = waypoint;
        this.offsets = {
            N: { x: 0, y: 1 },
            S: { x: 0, y: -1 },
            E: { x: 1, y: 0 },
            W: { x: -1, y: 0 },
        }
    }

    action(act, value) {
        if (this.waypoint) {
            this.action2(act, value);
        } else {
            this.action1(act, value);
        }
    }

    action1(act, value) {
        switch (act) {
            case 'N':
            case 'S':
            case 'E':
            case 'W':
                this.position.x += this.offsets[act].x * value;
                this.position.y += this.offsets[act].y * value;
                break;
            case 'R':
                this.facing += value / 90;
                this.facing %= 4;
                break;
            case 'L':
                this.facing -= value / 90;
                this.facing = (4 + this.facing % 4) % 4;
                break;
            case 'F':
                let steps = this.directions[this.facing]
                this.position.x += this.offsets[steps].x * value;
                this.position.y += this.offsets[steps].y * value;
                break;
        }
    }

    action2(act, value) {
        switch (act) {
            case 'N':
            case 'S':
            case 'E':
            case 'W':
                this.waypoint.x += this.offsets[act].x * value;
                this.waypoint.y += this.offsets[act].y * value;
                break;
            case 'R': {
                let rotations = value / 90;
                for (let i = 0; i < rotations; i++) {
                    [this.waypoint.x, this.waypoint.y] = [this.waypoint.y, this.waypoint.x];
                    this.waypoint.y = -this.waypoint.y;
                }
                break;
            }
            case 'L': {
                let rotations = value / 90;
                for (let i = 0; i < rotations; i++) {
                    [this.waypoint.x, this.waypoint.y] = [this.waypoint.y, this.waypoint.x];
                    this.waypoint.x = -this.waypoint.x;
                }
                break;
            }
            case 'F':
                for (let i = 0; i < value; i++) {
                    this.position.x += this.waypoint.x;
                    this.position.y += this.waypoint.y;
                }
                break;
        }
    }

    distance() {
        return Math.abs(this.position.x - this.start.x) + Math.abs(this.position.y - this.start.y)
    }
}

// console.log(lines);

let ship = new Ship(null);

for (let [act, value] of lines) {
    ship.action(act, value);
}
// console.log(ship);
console.log('1:', ship.distance());

ship = new Ship({ x: 10, y: 1 });
for (let [act, value] of lines) {
    ship.action(act, value);
}
// console.log(ship);
console.log('1:', ship.distance());
