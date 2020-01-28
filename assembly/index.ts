//import "allocator/arena";

// From The Computer Language Benchmarks Game
// http://benchmarksgame.alioth.debian.org

type float = f64; // interchangeable f32/f64 for testing

const SOLAR_MASS = <float>(4.0 * Math.PI * Math.PI);
const DAYS_PER_YEAR: float = 365.24;
const N_BODIES:u32 = 1000;

class Point {
  public x: float;
  public y: float;
  public vx: float;
  public vy: float;
  public mass: float;

  constructor (x:float, y: float, vx: float, vy:float, mass: float) {
    this.x = x;
    this.y = y;
    this.vx = vx;
    this.vy = vy;
    this.mass = mass;
  }

  offset (px: float, py: float): this {
    this.vx = -px / this.mass;
    this.vy = -py / this.mass;
    return this;
  }
}


class System {
  constructor (public bodies: Point[]) {
  }

  energy(): float {
    let e: float = 0.0;
    let bodies = this.bodies;

    for (let i: u32 = 0, size: u32 = bodies.length; i < size; ++i) {
      let bodyi:Point = (bodies[i]);

      let ix = bodyi.x;
      let iy = bodyi.y;

      let vx = bodyi.vx;
      let vy = bodyi.vy;

      let bim = bodyi.mass;

      e += 0.5 * bim * (vx * vx + vy * vy);

      for (let j: u32 = i + 1; j < size; ++j) {
        let bodyj:Point = (bodies[j]);
        let dx = ix - bodyj.x;
        let dy = iy - bodyj.y;
        let distance = <float>Math.sqrt(dx * dx + dy * dy);
        e -= bim * bodyj.mass / distance;
      }
    }
    return e;
  }

  advance (dt:float):this {
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
        let distance = <float>Math.sqrt(distanceSq);
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

  public z: float;
  public vz: float;

  constructor(
    x: float,
    y: float,
    z: float,
    vx: float,
    vy: float,
    vz: float,
    mass: float
  ) {
    super(x, y, vx, vy, mass);
    this.vz = vz;
    this.z = z;
  }

  offsetMomentum(px: float, py: float, pz: float): this {
    this.vx = -px / SOLAR_MASS;
    this.vy = -py / SOLAR_MASS;
    this.vz = -pz / SOLAR_MASS;
    return this;
  }
}

var system: System;

export function init(): number {
  let bodyArr:Point[] = new Array(N_BODIES);
  let ox:float = 0.0;
  let oy:float = 0.0;
  let m:float = 5;
  let add:float = 1.0;
  for (let i:u32 = 0; i < N_BODIES; i++) {
    bodyArr[i] = new Point(ox, oy, 0, 0, m);
    if (i % 10 === 0) {
      oy += 10; ox = 0;
    }
    ox += add;
  }
  system = new System(bodyArr);
  return system.energy();
}

export function step(): float {
  system.advance(0.1);
  return system.energy();
}

export function e ():float {
  return system.energy();
}

export function bench(steps: u32): void {
  //log(`${system.energy()}`);
  for (let i: u32 = 0; i < steps; ++i) system.advance(0.01);
  //system.energy()}`);
}

export function getBody(index: i32): Point | null {
  var bodies = system.bodies;
  return <u32>index < <u32>bodies.length ? bodies[index] : null;
}

export function abort(
  message: string | null,
  fileName: string | null,
  lineNumber: u32,
  columnNumber: u32
): void {

}
