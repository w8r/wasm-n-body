"use strict";
//import "allocator/arena";
Object.defineProperty(exports, "__esModule", { value: true });
const SOLAR_MASS = (4.0 * Math.PI * Math.PI);
const DAYS_PER_YEAR = 365.24;
const N_BODIES = 1000;
class Point {
    constructor(x, y, vx, vy, mass) {
        this.x = x;
        this.y = y;
        this.vx = vx;
        this.vy = vy;
        this.mass = mass;
    }
    offset(px, py) {
        this.vx = -px / this.mass;
        this.vy = -py / this.mass;
        return this;
    }
}
class System {
    constructor(bodies) {
        this.bodies = bodies;
    }
    energy() {
        let e = 0.0;
        let bodies = this.bodies;
        for (let i = 0, size = bodies.length; i < size; ++i) {
            let bodyi = (bodies[i]);
            let ix = bodyi.x;
            let iy = bodyi.y;
            let vx = bodyi.vx;
            let vy = bodyi.vy;
            let bim = bodyi.mass;
            e += 0.5 * bim * (vx * vx + vy * vy);
            for (let j = i + 1; j < size; ++j) {
                let bodyj = (bodies[j]);
                let dx = ix - bodyj.x;
                let dy = iy - bodyj.y;
                let distance = Math.sqrt(dx * dx + dy * dy);
                e -= bim * bodyj.mass / distance;
            }
        }
        return e;
    }
    advance(dt) {
        let bodies = this.bodies;
        let size = bodies.length;
        for (let i = 0; i < size; i++) {
            let bodyi = this.bodies[i];
            let ix = bodyi.x;
            let iy = bodyi.y;
            let bivx = bodyi.vx;
            let bivy = bodyi.vy;
            let bodyim = bodyi.mass;
            for (let j = i + 1; j < size; ++j) {
                let bodyj = unchecked(bodies[j]);
                let dx = ix - bodyj.x;
                let dy = iy - bodyj.y;
                let distanceSq = dx * dx + dy * dy;
                let distance = Math.sqrt(distanceSq);
                let mag = dt / (distanceSq * distance);
                let bim = bodyim * mag;
                let bjm = bodyj.mass * mag;
                bivx -= dx * bjm;
                bivy -= dy * bjm;
                bodyj.vx += dx * bim;
                bodyj.vy += dy * bim;
            }
            bodyi.vx = bivx;
            bodyi.vy = bivy;
            bodyi.x += dt * bivx;
            bodyi.y += dt * bivy;
        }
        return this;
    }
}
class Body extends Point {
    constructor(x, y, z, vx, vy, vz, mass) {
        super(x, y, vx, vy, mass);
        this.vz = vz;
        this.z = z;
    }
    offsetMomentum(px, py, pz) {
        this.vx = -px / SOLAR_MASS;
        this.vy = -py / SOLAR_MASS;
        this.vz = -pz / SOLAR_MASS;
        return this;
    }
}
var system;
function init() {
    let bodyArr = new Array(N_BODIES);
    let ox = 0.0;
    let oy = 0.0;
    let m = 5;
    let add = 1.0;
    for (let i = 0; i < N_BODIES; i++) {
        bodyArr[i] = new Point(ox, oy, 0, 0, m);
        if (i % 10 === 0) {
            oy += 10;
            ox = 0;
        }
        ox += add;
    }
    system = new System(bodyArr);
    return system.energy();
}
exports.init = init;
function step() {
    system.advance(0.1);
    return system.energy();
}
exports.step = step;
function e() {
    return system.energy();
}
exports.e = e;
function bench(steps) {
    //log(`${system.energy()}`);
    for (let i = 0; i < steps; ++i)
        system.advance(0.01);
    //system.energy()}`);
}
exports.bench = bench;
function getBody(index) {
    var bodies = system.bodies;
    return index < bodies.length ? bodies[index] : null;
}
exports.getBody = getBody;
function abort(message, fileName, lineNumber, columnNumber) {
}
exports.abort = abort;
