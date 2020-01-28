// From The Computer Language Benchmarks Game
// http://benchmarksgame.alioth.debian.org

type float = f64; // interchangeable f32/f64 for testing

const SOLAR_MASS = <float>(4.0 * Math.PI * Math.PI);
const DAYS_PER_YEAR: float = 365.24;
const N_BODIES = 1000;

class Point {
  constructor(
    public x: float,
    public y: float,
    public vx: float,
    public vy: float,
    public mass: float
  ) {}

  offset(px: float, py: float): this {
    this.vx = -px / this.mass;
    this.vy = -py / this.mass;
    return this;
  }
}

class System {
  constructor (public bodies: Point[]) {
  }

  energy(): float {
    let e: float = 0;
    let bodies = this.bodies;

    for (let i = 0, size = bodies.length; i < size; ++i) {
      let bodyi = unchecked(bodies[i]);

      let ix = bodyi.x;
      let iy = bodyi.y;

      let vx = bodyi.vx;
      let vy = bodyi.vy;

      let bim = bodyi.mass;

      e += 0.5 * bim * (vx * vx + vy * vy);

      for (let j = i + 1; j < size; ++j) {
        let bodyj = unchecked(bodies[j]);
        let dx = ix - bodyj.x;
        let dy = iy - bodyj.y;
        let distance = <float>Math.sqrt(dx * dx + dy * dy);
        e -= bim * bodyj.mass / distance;
      }
    }
    return e;
  }

  @inline
  advance(dt: float): this {
    let bodies = this.bodies;
    let size = bodies.length;

    for (let i = 0; i < size; i++) {
      let bodyi = unchecked(bodies[i]);
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

var system: System;

export function init(): float {
  let bodyArr = new Array<Point>(N_BODIES);
  let ox: float = 0;
  let oy: float = 0;
  let m:  float = 5;
  for (let i = 0; i < N_BODIES; i++) {
    unchecked(bodyArr[i] = new Point(ox, oy, 0, 0, m));
    if (i % 10 === 0) {
      oy += 10; ox = 0;
    }
    ox += 1;
  }
  system = new System(bodyArr);
  return system.energy();
}

export function step(): float {
  system.advance(0.1);
  return system.energy();
}

export function e(): float {
  return system.energy();
}

export function bench(steps: i32): void {
  for (let i = 0; i < steps; ++i) system.advance(0.01);
}

export function getBody(index: i32): Point | null {
  var bodies = system.bodies;
  return <u32>index < <u32>bodies.length ? unchecked(bodies[index]) : null;
}