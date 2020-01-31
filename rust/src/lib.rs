// Code adopted from https://benchmarksgame-team.pages.debian.net/benchmarksgame/program/nbody-rust-1.html

// #![feature(core_intrinsics, panic_implementation)]
// #![no_std]

// use core::intrinsics;
// use core::panic::PanicInfo;
use std::f64;

// #[panic_handler]
// #[no_mangle]
// pub fn panic(_info: &PanicInfo) -> ! {
//   unsafe { intrinsics::abort() }
// }

// #[inline(always)]
// fn sqrt(x: f64) -> f64 {
//   unsafe { intrinsics::sqrtf64(x) }
// }

const PI: f64         = 3.141592653589793;
const SOLAR_MASS: f64 = 4.0 * PI * PI;
const YEAR: f64       = 365.24;
const N_BODIES: usize = 1000;

//mut rk:f64 = 0.0;

// static mut BODIES: [Planet; N_BODIES] = [
//   // Sun
//   Planet {
//     x:   0.0,
//     y:   0.0,
//     z:   0.0,
//     vx:  0.0,
//     vy:  0.0,
//     vz:  0.0,
//     mass: SOLAR_MASS,
//   },
//   // Jupiter
//   Planet {
//     x:    4.84143144246472090e+00,
//     y:   -1.16032004402742839e+00,
//     z:   -1.03622044471123109e-01,
//     vx:   1.66007664274403694e-03 * YEAR,
//     vy:   7.69901118419740425e-03 * YEAR,
//     vz:  -6.90460016972063023e-05 * YEAR,
//     mass: 9.54791938424326609e-04 * SOLAR_MASS,
//   },
//   // Saturn
//   Planet {
//     x:    8.34336671824457987e+00,
//     y:    4.12479856412430479e+00,
//     z:   -4.03523417114321381e-01,
//     vx:  -2.76742510726862411e-03 * YEAR,
//     vy:   4.99852801234917238e-03 * YEAR,
//     vz:   2.30417297573763929e-05 * YEAR,
//     mass: 2.85885980666130812e-04 * SOLAR_MASS,
//   },
//   // Uranus
//   Planet {
//     x:    1.28943695621391310e+01,
//     y:   -1.51111514016986312e+01,
//     z:   -2.23307578892655734e-01,
//     vx:   2.96460137564761618e-03 * YEAR,
//     vy:   2.37847173959480950e-03 * YEAR,
//     vz:  -2.96589568540237556e-05 * YEAR,
//     mass: 4.36624404335156298e-05 * SOLAR_MASS,
//   },
//   // Neptune
//   Planet {
//     x:    1.53796971148509165e+01,
//     y:   -2.59193146099879641e+01,
//     z:    1.79258772950371181e-01,
//     vx:   2.68067772490389322e-03 * YEAR,
//     vy:   1.62824170038242295e-03 * YEAR,
//     vz:  -9.51592254519715870e-05 * YEAR,
//     mass: 5.15138902046611451e-05 * SOLAR_MASS,
//   },


//   Planet {
//     x:    1.28943695621391310e+01 + (rk++),
//     y:   -1.51111514016986312e+01 + (rk++),
//     z:   -2.23307578892655734e-01,
//     vx:   2.96460137564761618e-03 * YEAR,
//     vy:   2.37847173959480950e-03 * YEAR,
//     vz:  -2.96589568540237556e-05 * YEAR,
//     mass: 4.36624404335156298e-05 * SOLAR_MASS,
//   },
//   Planet {
//     x:    1.28943695621391310e+01 + (rk++),
//     y:   -1.51111514016986312e+01 + (rk++),
//     z:   -2.23307578892655734e-01,
//     vx:   2.96460137564761618e-03 * YEAR,
//     vy:   2.37847173959480950e-03 * YEAR,
//     vz:  -2.96589568540237556e-05 * YEAR,
//     mass: 4.36624404335156298e-05 * SOLAR_MASS,
//   },
//   Planet {
//     x:    1.28943695621391310e+01 + (rk++),
//     y:   -1.51111514016986312e+01 + (rk++),
//     z:   -2.23307578892655734e-01,
//     vx:   2.96460137564761618e-03 * YEAR,
//     vy:   2.37847173959480950e-03 * YEAR,
//     vz:  -2.96589568540237556e-05 * YEAR,
//     mass: 4.36624404335156298e-05 * SOLAR_MASS,
//   },
//   Planet {
//     x:    1.28943695621391310e+01 + (rk++),
//     y:   -1.51111514016986312e+01 + (rk++),
//     z:   -2.23307578892655734e-01,
//     vx:   2.96460137564761618e-03 * YEAR,
//     vy:   2.37847173959480950e-03 * YEAR,
//     vz:  -2.96589568540237556e-05 * YEAR,
//     mass: 4.36624404335156298e-05 * SOLAR_MASS,
//   },
//   Planet {
//     x:    1.28943695621391310e+01 + (rk++),
//     y:   -1.51111514016986312e+01 + (rk++),
//     z:   -2.23307578892655734e-01,
//     vx:   2.96460137564761618e-03 * YEAR,
//     vy:   2.37847173959480950e-03 * YEAR,
//     vz:  -2.96589568540237556e-05 * YEAR,
//     mass: 4.36624404335156298e-05 * SOLAR_MASS,
//   },
//   Planet {
//     x:    1.28943695621391310e+01 + (rk++),
//     y:   -1.51111514016986312e+01 + (rk++),
//     z:   -2.23307578892655734e-01,
//     vx:   2.96460137564761618e-03 * YEAR,
//     vy:   2.37847173959480950e-03 * YEAR,
//     vz:  -2.96589568540237556e-05 * YEAR,
//     mass: 4.36624404335156298e-05 * SOLAR_MASS,
//   },
//   Planet {
//     x:    1.28943695621391310e+01 + (rk++),
//     y:   -1.51111514016986312e+01 + (rk++),
//     z:   -2.23307578892655734e-01,
//     vx:   2.96460137564761618e-03 * YEAR,
//     vy:   2.37847173959480950e-03 * YEAR,
//     vz:  -2.96589568540237556e-05 * YEAR,
//     mass: 4.36624404335156298e-05 * SOLAR_MASS,
//   },
// ];

#[derive(Clone, Copy)]
struct Point {
  x: f64,
  y: f64,
  vx: f64,
  vy: f64,
  mass: f64,
}

impl Point {
  fn new(x: f64, y: f64, vx: f64, vy: f64, mass: f64) -> Self {
    Point { x, y, vx, vy, mass }
  }
}

static mut BODIES: [Point; N_BODIES] = [Point { x: 0.045, y:0.764, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.973, y:0.223, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.744, y:0.668, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.602, y:0.713, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.172, y:0.129, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.984, y:0.152, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.317, y:0.948, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.589, y:0.270, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.138, y:0.152, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.592, y:0.915, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.356, y:0.934, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.191, y:0.196, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.927, y:0.586, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.024, y:0.019, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.584, y:0.535, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.634, y:0.276, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.315, y:0.523, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.082, y:0.862, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.471, y:0.248, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.342, y:0.421, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.234, y:0.776, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.048, y:0.715, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.162, y:0.217, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.799, y:0.949, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.486, y:0.866, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.246, y:0.312, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.899, y:0.334, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.324, y:0.688, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.880, y:0.185, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.694, y:0.805, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.476, y:0.870, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.830, y:0.645, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.923, y:0.184, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.137, y:0.062, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.243, y:0.146, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.101, y:0.176, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.458, y:0.413, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.483, y:0.929, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.570, y:0.650, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.965, y:0.801, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.002, y:0.383, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.584, y:0.257, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.444, y:0.636, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.600, y:0.937, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.839, y:0.453, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.762, y:0.483, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.233, y:0.329, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.124, y:0.682, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.006, y:0.571, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.600, y:0.895, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.545, y:0.544, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.138, y:0.735, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.755, y:0.939, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.853, y:0.062, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.661, y:0.723, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.555, y:0.691, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.944, y:0.433, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.139, y:0.002, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.938, y:0.797, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.416, y:0.563, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.307, y:0.170, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.090, y:0.251, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.344, y:0.403, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.539, y:0.025, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.963, y:0.971, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.607, y:0.492, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.063, y:0.972, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.329, y:0.507, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.828, y:0.645, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.189, y:0.911, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.774, y:0.955, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.727, y:0.957, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.613, y:0.381, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.732, y:0.867, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.730, y:0.664, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.359, y:0.073, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.225, y:0.391, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.550, y:0.967, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.115, y:0.604, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.907, y:0.203, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.303, y:0.940, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.049, y:0.061, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.040, y:0.272, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.072, y:0.628, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.245, y:0.067, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.907, y:0.108, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.358, y:0.612, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.952, y:0.368, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.240, y:0.464, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.047, y:0.700, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.485, y:0.265, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.943, y:0.223, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.284, y:0.857, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.495, y:0.344, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.362, y:0.635, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.522, y:0.509, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.096, y:0.739, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.312, y:0.109, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.871, y:0.391, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.226, y:0.818, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.983, y:0.688, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.347, y:0.946, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.445, y:0.044, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.667, y:0.320, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.173, y:0.449, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.695, y:0.706, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.365, y:0.167, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.300, y:0.544, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.218, y:0.724, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.929, y:0.464, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.873, y:0.943, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.043, y:0.816, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.169, y:0.234, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.536, y:0.501, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.146, y:0.193, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.462, y:0.806, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.885, y:0.640, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.958, y:0.802, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.515, y:0.615, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.696, y:0.935, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.035, y:0.495, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.424, y:0.056, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.426, y:0.306, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.751, y:0.724, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.783, y:0.121, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.566, y:0.240, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.880, y:0.712, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.843, y:0.103, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.972, y:0.109, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.564, y:0.009, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.337, y:0.034, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.516, y:0.589, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.079, y:0.190, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.424, y:0.542, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.332, y:0.640, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.622, y:0.480, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.172, y:0.394, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.114, y:0.185, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.269, y:0.654, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.864, y:0.690, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.323, y:0.109, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.949, y:0.148, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.897, y:0.084, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.329, y:0.335, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.898, y:0.502, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.004, y:0.608, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.445, y:0.440, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.812, y:0.582, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.817, y:0.590, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.331, y:0.944, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.375, y:0.037, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.454, y:0.642, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.245, y:0.419, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.712, y:0.746, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.410, y:0.744, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.562, y:0.838, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.914, y:0.586, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.512, y:0.290, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.969, y:0.992, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.973, y:0.384, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.839, y:0.916, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.852, y:0.755, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.059, y:0.584, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.842, y:0.680, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.497, y:0.168, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.680, y:0.418, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.947, y:0.166, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.896, y:0.715, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.694, y:0.924, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.217, y:0.707, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.479, y:0.444, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.121, y:0.184, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.534, y:0.594, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.187, y:0.841, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.055, y:0.290, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.662, y:0.876, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.946, y:0.448, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.518, y:0.536, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.526, y:0.056, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.646, y:0.146, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.529, y:0.580, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.928, y:0.727, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.905, y:0.332, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.452, y:0.004, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.547, y:0.539, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.462, y:0.091, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.298, y:0.517, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.640, y:0.493, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.255, y:0.670, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.089, y:0.025, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.365, y:0.835, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.465, y:0.895, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.012, y:0.806, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.532, y:0.246, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.150, y:0.493, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.051, y:0.758, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.188, y:0.905, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.943, y:0.233, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.525, y:0.187, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.309, y:0.100, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.461, y:0.152, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.328, y:0.703, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.861, y:0.018, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.142, y:0.026, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.507, y:0.230, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.564, y:0.646, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.854, y:0.766, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.326, y:0.061, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.706, y:0.882, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.882, y:0.788, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.837, y:0.908, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.408, y:0.334, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.076, y:0.778, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.337, y:0.019, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.125, y:0.679, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.740, y:0.660, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.786, y:0.253, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.982, y:0.431, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.621, y:0.389, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.357, y:0.546, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.031, y:0.916, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.419, y:0.566, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.293, y:0.167, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.272, y:0.994, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.572, y:0.439, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.853, y:0.295, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.445, y:0.452, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.825, y:0.369, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.679, y:0.538, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.931, y:0.306, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.377, y:0.314, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.752, y:0.248, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.200, y:0.097, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.808, y:0.256, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.868, y:0.248, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.808, y:0.658, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.305, y:0.472, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.205, y:0.909, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.796, y:0.159, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.829, y:0.573, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.325, y:0.494, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.602, y:0.985, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.923, y:0.901, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.966, y:0.437, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.170, y:0.593, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.438, y:0.658, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.295, y:0.495, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.313, y:0.402, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.057, y:0.641, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.464, y:0.899, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.600, y:0.303, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.688, y:0.022, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.341, y:0.456, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.169, y:0.720, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.703, y:0.608, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.550, y:0.529, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.247, y:0.115, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.818, y:0.089, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.343, y:0.801, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.349, y:0.265, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.611, y:0.183, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.176, y:0.776, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.547, y:0.616, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.907, y:0.179, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.249, y:0.341, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.131, y:0.783, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 1.000, y:0.467, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.119, y:0.030, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.918, y:0.860, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.631, y:0.970, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.428, y:0.788, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.288, y:0.884, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.127, y:0.220, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.074, y:0.186, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.325, y:0.638, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.569, y:0.639, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.578, y:0.338, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.178, y:0.630, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.898, y:0.121, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.249, y:0.045, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.790, y:0.826, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.793, y:0.437, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.623, y:0.831, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.875, y:0.254, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.590, y:0.082, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.404, y:0.027, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.122, y:0.056, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.375, y:0.760, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.606, y:0.626, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.358, y:0.093, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.849, y:0.686, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.763, y:0.322, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.342, y:0.576, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.176, y:0.914, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.632, y:0.299, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.835, y:0.855, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.556, y:0.440, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.846, y:0.612, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.617, y:0.144, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.097, y:0.703, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.162, y:0.383, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.161, y:0.532, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.000, y:0.172, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.365, y:0.520, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.404, y:0.501, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.818, y:0.271, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.632, y:0.901, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.391, y:0.529, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.158, y:0.891, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.082, y:0.986, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.316, y:0.569, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.246, y:0.587, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.223, y:0.563, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.571, y:0.965, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.394, y:0.585, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.461, y:0.204, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.501, y:0.765, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.375, y:0.699, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.349, y:0.224, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.206, y:0.605, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.063, y:0.096, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.644, y:0.290, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.906, y:0.638, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.549, y:0.506, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.226, y:0.972, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.994, y:0.777, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.090, y:0.414, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.212, y:0.848, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.680, y:0.039, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.228, y:0.688, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.639, y:0.034, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.922, y:0.189, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.479, y:0.304, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.166, y:0.107, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.628, y:0.574, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.880, y:0.343, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.789, y:0.250, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.917, y:0.695, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.640, y:0.294, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.158, y:0.414, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.576, y:0.764, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.593, y:0.202, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.581, y:0.295, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.620, y:0.034, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.910, y:0.179, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.181, y:0.223, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.256, y:0.423, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.795, y:0.717, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.900, y:0.462, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.673, y:0.712, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.040, y:0.328, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.714, y:0.232, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.806, y:0.215, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.516, y:0.240, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.177, y:0.026, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.360, y:0.955, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.912, y:0.811, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.577, y:0.494, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.786, y:0.077, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.752, y:0.903, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.879, y:0.470, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.286, y:0.335, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.083, y:0.592, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.371, y:0.723, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.862, y:0.670, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.028, y:0.978, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.157, y:0.018, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.149, y:0.249, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.884, y:0.023, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.344, y:0.362, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.792, y:0.727, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.683, y:0.803, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.138, y:0.889, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.346, y:0.574, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.993, y:0.001, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.983, y:0.086, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.537, y:0.108, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.306, y:0.028, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.774, y:0.583, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.728, y:0.185, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.106, y:0.198, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.117, y:0.404, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.538, y:0.805, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.599, y:0.172, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.234, y:0.652, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.355, y:0.578, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.018, y:0.669, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.005, y:0.795, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.090, y:0.248, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.203, y:0.590, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.306, y:0.328, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.631, y:0.112, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.058, y:0.380, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.792, y:0.275, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.582, y:0.945, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.161, y:0.318, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.653, y:0.358, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.760, y:0.964, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.807, y:0.372, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.228, y:0.448, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.968, y:0.979, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.059, y:0.352, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.357, y:0.524, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.599, y:0.977, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.581, y:0.780, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.627, y:0.803, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.551, y:0.608, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.432, y:0.913, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.041, y:0.240, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.830, y:0.338, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.245, y:0.087, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.549, y:0.958, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.613, y:0.170, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.164, y:0.629, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.873, y:0.976, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.292, y:0.156, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.511, y:0.481, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.293, y:0.885, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.279, y:0.400, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.072, y:0.197, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.662, y:0.576, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.512, y:0.430, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.753, y:0.124, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.832, y:0.314, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.139, y:0.213, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.822, y:0.136, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.660, y:0.860, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.054, y:0.243, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.957, y:0.636, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.345, y:0.462, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.864, y:0.325, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.057, y:0.777, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.571, y:0.940, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.487, y:0.900, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.115, y:0.124, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.547, y:0.943, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.545, y:0.975, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.714, y:0.080, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.549, y:0.354, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.188, y:0.009, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.490, y:0.388, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.613, y:0.403, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.755, y:0.387, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.644, y:0.531, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.383, y:0.092, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.315, y:0.252, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.692, y:0.005, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.040, y:0.712, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.648, y:0.622, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.368, y:0.084, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.653, y:0.081, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.830, y:0.551, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.297, y:0.606, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.203, y:0.930, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.384, y:0.176, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.883, y:0.012, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.097, y:0.818, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.848, y:0.480, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.714, y:0.662, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.867, y:0.736, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.376, y:0.233, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.367, y:0.041, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.849, y:0.040, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.476, y:0.816, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.363, y:0.761, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.587, y:0.467, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.171, y:0.663, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.153, y:0.912, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.285, y:0.749, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.273, y:0.139, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.870, y:0.742, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.024, y:0.674, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.486, y:0.598, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.462, y:0.276, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.832, y:0.932, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.993, y:0.446, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.972, y:0.816, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.693, y:0.506, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.634, y:0.917, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.705, y:0.906, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.626, y:0.048, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.469, y:0.999, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.804, y:0.079, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.074, y:0.277, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.048, y:0.740, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.718, y:0.720, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.364, y:0.729, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.432, y:0.033, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.824, y:0.292, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.014, y:0.218, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.127, y:0.515, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.120, y:0.985, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.046, y:0.796, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.053, y:0.954, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.247, y:0.829, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.468, y:0.873, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.818, y:0.259, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.594, y:0.662, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.997, y:0.149, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.683, y:0.175, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.844, y:0.492, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.692, y:0.623, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.088, y:0.538, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.635, y:0.324, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.881, y:0.510, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.701, y:0.158, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.245, y:0.365, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.428, y:0.401, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.171, y:0.904, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.975, y:0.117, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.320, y:0.216, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.639, y:0.268, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.266, y:0.145, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.107, y:0.369, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.772, y:0.306, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.901, y:0.134, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.127, y:0.589, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.769, y:0.026, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.017, y:0.139, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.341, y:0.166, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.397, y:0.648, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.514, y:0.419, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.881, y:0.275, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.727, y:0.087, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.177, y:0.969, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.420, y:0.724, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.895, y:0.179, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.587, y:0.854, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.088, y:0.054, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.985, y:0.663, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.828, y:0.126, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.915, y:0.850, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.606, y:0.113, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.904, y:0.981, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.955, y:0.774, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.057, y:0.073, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.102, y:0.911, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.860, y:0.805, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.328, y:0.727, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.563, y:0.571, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.390, y:0.234, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.663, y:0.988, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.851, y:0.069, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.937, y:0.436, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.672, y:0.154, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.558, y:0.619, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.903, y:0.175, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.587, y:0.077, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.442, y:0.569, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.882, y:0.681, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.455, y:0.801, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.148, y:0.273, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.717, y:0.453, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.102, y:0.676, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.821, y:0.547, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.614, y:0.303, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.253, y:0.180, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.227, y:0.448, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.367, y:0.537, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.157, y:0.641, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.608, y:0.653, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.537, y:0.029, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.353, y:0.016, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.676, y:0.333, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.958, y:0.801, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.355, y:0.223, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.807, y:0.670, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.851, y:0.409, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.979, y:0.585, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.109, y:0.899, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.340, y:0.129, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.628, y:0.276, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.461, y:0.264, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.002, y:0.614, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.124, y:0.619, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.747, y:0.487, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.895, y:0.174, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.187, y:0.508, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.415, y:0.241, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.240, y:0.857, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.681, y:0.026, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.736, y:0.845, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.828, y:0.245, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.505, y:0.594, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.558, y:0.181, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.615, y:0.889, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.031, y:0.059, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.303, y:0.922, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.103, y:0.976, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.563, y:0.553, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.145, y:0.509, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.620, y:0.537, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.352, y:0.001, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.021, y:0.272, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.317, y:0.578, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.134, y:0.935, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.407, y:0.304, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.199, y:0.301, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.948, y:0.245, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.510, y:0.256, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.081, y:0.409, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.898, y:0.221, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.496, y:0.182, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.884, y:0.537, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.158, y:0.873, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.852, y:0.230, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.955, y:0.875, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.090, y:0.111, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.118, y:0.730, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.195, y:0.236, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.251, y:0.008, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.958, y:0.424, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.458, y:0.970, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.089, y:0.002, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.384, y:0.074, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.184, y:0.751, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.294, y:0.090, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.465, y:0.415, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.988, y:0.488, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.129, y:0.737, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.373, y:0.649, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.537, y:0.523, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.571, y:0.343, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.461, y:0.740, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.571, y:0.329, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.141, y:0.964, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.321, y:0.554, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.050, y:0.579, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.092, y:0.687, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.200, y:0.584, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.545, y:0.074, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.992, y:0.452, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.306, y:0.139, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.010, y:0.375, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.977, y:0.112, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.648, y:0.249, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.915, y:0.960, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.627, y:0.157, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.451, y:0.548, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.083, y:0.950, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.378, y:0.626, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.550, y:0.590, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.227, y:0.573, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.322, y:0.186, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.792, y:0.897, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.706, y:0.154, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.041, y:0.188, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.027, y:0.023, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.661, y:0.152, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.655, y:0.546, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.995, y:0.616, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.764, y:0.035, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.833, y:0.586, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.716, y:0.413, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.008, y:0.353, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.963, y:0.490, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.798, y:0.851, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.729, y:0.392, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.340, y:0.011, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.609, y:0.327, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.739, y:0.255, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.595, y:0.354, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.819, y:0.464, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.879, y:0.760, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.936, y:0.398, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.791, y:0.995, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.329, y:0.678, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.758, y:0.324, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.849, y:0.015, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.065, y:0.378, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.419, y:0.223, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.998, y:0.932, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.484, y:0.025, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.912, y:0.297, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.676, y:0.029, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.644, y:0.885, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.483, y:0.200, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.585, y:0.938, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.323, y:0.115, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.482, y:0.090, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.484, y:0.630, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.902, y:0.296, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.030, y:0.479, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.869, y:0.401, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.358, y:0.267, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.330, y:0.597, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.104, y:0.649, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.508, y:0.755, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.108, y:0.319, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.191, y:0.380, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.922, y:0.576, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.531, y:0.576, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.208, y:0.599, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.197, y:0.788, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.675, y:0.572, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.463, y:0.534, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.760, y:0.410, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.248, y:0.909, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.476, y:0.216, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.420, y:0.783, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.459, y:0.027, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.982, y:0.385, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.039, y:0.715, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.211, y:0.798, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.829, y:0.016, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.814, y:0.205, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.575, y:0.388, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.421, y:0.333, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.042, y:0.149, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.278, y:0.162, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.970, y:0.560, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.616, y:0.073, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.114, y:0.463, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.017, y:0.599, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.811, y:0.736, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.634, y:0.091, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.233, y:0.809, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.549, y:0.988, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.983, y:0.181, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.775, y:0.429, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.093, y:0.816, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.070, y:0.268, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.814, y:0.793, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.716, y:0.316, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.655, y:0.129, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.698, y:0.811, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.506, y:0.934, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.351, y:0.973, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.852, y:0.982, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.193, y:0.416, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.442, y:0.074, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.264, y:0.131, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.507, y:0.473, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.420, y:0.644, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.399, y:0.590, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.846, y:0.868, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.402, y:0.390, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.455, y:0.499, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.774, y:0.743, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.486, y:0.702, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.571, y:0.259, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.841, y:0.452, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.477, y:0.945, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.381, y:0.803, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.495, y:0.364, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.995, y:0.155, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.302, y:0.701, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.707, y:0.725, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.213, y:0.435, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.752, y:0.761, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.679, y:0.764, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.463, y:0.332, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.300, y:0.325, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.606, y:0.520, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.194, y:0.968, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.825, y:0.452, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.714, y:0.156, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.087, y:0.076, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.354, y:0.410, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.018, y:0.372, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.734, y:0.282, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.597, y:0.186, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.114, y:0.156, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.431, y:0.148, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.149, y:0.130, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.115, y:0.392, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.236, y:0.990, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.904, y:0.514, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.490, y:0.633, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.280, y:0.477, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.286, y:0.444, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.115, y:0.583, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.984, y:0.237, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.045, y:0.577, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.632, y:0.193, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.779, y:0.417, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.847, y:0.444, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.842, y:0.059, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.529, y:0.454, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.125, y:0.972, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.187, y:0.903, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.789, y:0.670, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.161, y:0.170, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.359, y:0.003, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.039, y:0.212, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.443, y:0.223, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.006, y:0.033, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.847, y:0.336, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.270, y:0.495, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.581, y:0.446, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.284, y:0.334, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.053, y:0.359, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.745, y:0.328, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.133, y:0.780, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.402, y:0.926, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.673, y:0.227, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.425, y:0.550, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.440, y:0.078, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.184, y:0.917, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.799, y:0.945, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.698, y:0.030, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.254, y:0.293, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.121, y:0.510, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.011, y:0.758, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.663, y:0.898, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.171, y:0.787, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.408, y:0.565, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.395, y:0.622, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.965, y:0.249, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.802, y:0.017, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.530, y:0.419, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.114, y:0.353, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.571, y:0.070, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.137, y:0.310, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.096, y:0.939, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.976, y:0.511, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.669, y:0.350, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.880, y:0.895, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.196, y:0.645, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.550, y:0.059, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.722, y:0.914, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.787, y:0.774, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.398, y:0.434, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.218, y:0.271, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.075, y:0.581, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.827, y:0.467, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.569, y:0.444, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.320, y:0.012, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.635, y:0.569, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.534, y:0.993, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.095, y:0.940, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.727, y:0.002, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.633, y:0.419, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.233, y:0.960, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.249, y:0.009, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.618, y:0.435, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.252, y:0.223, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.011, y:0.120, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.369, y:0.579, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.857, y:0.982, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.792, y:0.580, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.461, y:0.536, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.927, y:0.249, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.765, y:0.821, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.662, y:0.320, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.376, y:0.186, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.170, y:0.349, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.311, y:0.181, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.506, y:0.414, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.425, y:0.271, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.501, y:0.285, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.620, y:0.559, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.904, y:0.708, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.695, y:0.911, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.783, y:0.691, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.990, y:0.135, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.311, y:0.032, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.112, y:0.484, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.521, y:0.955, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.842, y:0.458, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.514, y:0.014, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.501, y:0.513, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.717, y:0.780, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.471, y:0.799, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.160, y:0.643, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.219, y:0.648, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.143, y:0.819, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.089, y:0.714, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.029, y:0.106, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.984, y:0.314, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.697, y:0.473, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.779, y:0.720, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.676, y:0.766, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.590, y:0.059, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.460, y:0.309, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.621, y:0.788, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.126, y:0.625, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.531, y:0.612, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.522, y:0.258, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.790, y:0.617, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.198, y:0.587, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.283, y:0.556, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.564, y:0.924, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.154, y:0.781, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.562, y:0.246, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.126, y:0.434, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.844, y:0.513, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.935, y:0.739, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.055, y:0.774, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.129, y:0.856, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.672, y:0.058, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.721, y:0.688, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.675, y:0.452, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.539, y:0.930, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.850, y:0.771, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.846, y:0.870, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.975, y:0.843, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.891, y:0.834, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.859, y:0.882, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.452, y:0.948, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.709, y:0.588, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.824, y:0.509, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.535, y:0.304, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.212, y:0.034, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.783, y:0.514, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.668, y:0.901, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.760, y:0.905, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.405, y:0.268, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.860, y:0.089, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.715, y:0.057, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.054, y:0.216, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.525, y:0.362, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.303, y:0.541, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.734, y:0.431, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.853, y:0.361, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.804, y:0.453, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.871, y:0.518, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.769, y:0.309, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.679, y:0.963, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.865, y:0.762, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.190, y:0.156, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.150, y:0.502, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.680, y:0.915, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.405, y:0.124, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.984, y:0.837, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.129, y:0.664, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.510, y:0.913, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.246, y:0.277, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.872, y:0.849, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.530, y:0.940, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.661, y:0.720, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.090, y:0.398, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.224, y:0.284, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.164, y:0.563, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.384, y:0.404, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.242, y:0.446, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.643, y:0.772, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.054, y:0.923, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.751, y:0.797, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.389, y:0.572, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.824, y:0.893, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.914, y:0.256, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.055, y:0.761, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.077, y:0.293, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.156, y:0.500, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.080, y:0.582, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.622, y:0.487, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.741, y:0.428, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.941, y:0.895, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.543, y:0.429, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.434, y:0.426, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.753, y:0.365, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.611, y:0.213, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.836, y:0.259, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.287, y:0.093, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.736, y:0.424, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.517, y:0.682, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.863, y:0.560, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.859, y:0.729, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.363, y:0.253, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.879, y:0.733, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.784, y:0.464, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.231, y:0.277, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.761, y:0.450, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.377, y:0.853, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.748, y:0.472, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.682, y:0.263, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.826, y:0.785, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.102, y:0.306, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.897, y:0.335, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.096, y:0.068, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.678, y:0.035, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.144, y:0.022, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.451, y:0.086, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.977, y:0.624, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.221, y:0.152, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.279, y:0.762, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.093, y:0.500, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.614, y:0.284, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.332, y:0.677, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.379, y:0.313, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.914, y:0.560, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.385, y:0.127, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.649, y:0.289, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.212, y:0.298, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.194, y:0.140, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.769, y:0.436, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.671, y:0.118, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.494, y:0.580, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.076, y:0.335, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.624, y:0.646, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.562, y:0.670, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.114, y:0.946, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.936, y:0.369, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.625, y:0.773, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.251, y:0.882, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.283, y:0.524, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.498, y:0.983, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.360, y:0.768, vx:0.0, vy:0.0, mass: 5.0 },Point { x: 0.727, y:0.932, vx:0.0, vy:0.0, mass: 5.0 }];


#[derive(Clone, Copy)]
struct Planet {
  x: f64,
  y: f64,
  z: f64,
  vx: f64,
  vy: f64,
  vz: f64,
  mass: f64,
}



fn advance1(bodies: &mut [Planet; N_BODIES], dt: f64) {
  let mut b_slice: &mut [_] = bodies;
  loop {
    let bi = match shift_mut_ref(&mut b_slice) {
      Some(bi) => bi,
      None     => break,
    };

    for bj in b_slice.iter_mut() {
      let dx = bi.x - bj.x;
      let dy = bi.y - bj.y;
      let dz = bi.z - bj.z;

      let d2  = dx * dx + dy * dy + dz * dz;
      let mag = dt / (d2 * d2.sqrt());

      let massj_mag = bj.mass * mag;
      bi.vx -= dx * massj_mag;
      bi.vy -= dy * massj_mag;
      bi.vz -= dz * massj_mag;

      let massi_mag = bi.mass * mag;
      bj.vx += dx * massi_mag;
      bj.vy += dy * massi_mag;
      bj.vz += dz * massi_mag;
    }
    bi.x += dt * bi.vx;
    bi.y += dt * bi.vy;
    bi.z += dt * bi.vz;
  }
}

fn energy1(bodies: &[Planet; N_BODIES]) -> f64 {
  let mut e = 0.0;
  let mut bodies = bodies.iter();

  loop {
    let bi = match bodies.next() {
      Some(bi) => bi,
      None     => break,
    };

    e += (bi.vx * bi.vx + bi.vy * bi.vy + bi.vz * bi.vz) * bi.mass / 2.0;
    for bj in bodies.clone() {
      let dx = bi.x - bj.x;
      let dy = bi.y - bj.y;
      let dz = bi.z - bj.z;
      let dist = (dx * dx + dy * dy + dz * dz).sqrt();
      e -= bi.mass * bj.mass / dist;
    }
  }
  e
}

fn advance(bodies: &mut [Point; N_BODIES], dt: f64) {
  let mut b_slice: &mut [_] = bodies;
  loop {
    let bi = match shift_mut_ref(&mut b_slice) {
      Some(bi) => bi,
      None     => break,
    };

    for bj in b_slice.iter_mut() {
      let dx = bi.x - bj.x;
      let dy = bi.y - bj.y;

      let d2  = dx * dx + dy * dy;
      let mag = dt / (d2 * d2.sqrt());

      let massj_mag = bj.mass * mag;
      bi.vx -= dx * massj_mag;
      bi.vy -= dy * massj_mag;

      let massi_mag = bi.mass * mag;
      bj.vx += dx * massi_mag;
      bj.vy += dy * massi_mag;
    }
    bi.x += dt * bi.vx;
    bi.y += dt * bi.vy;
  }
}

fn energy(bodies: &[Point; N_BODIES]) -> f64 {
  let mut e = 0.0;
  let mut bodies = bodies.iter();

  loop {
    let bi = match bodies.next() {
      Some(bi) => bi,
      None     => break,
    };

    e += (bi.vx * bi.vx + bi.vy * bi.vy) * bi.mass / 2.0;
    for bj in bodies.clone() {
      let dx = bi.x - bj.x;
      let dy = bi.y - bj.y;
      let dist = (dx * dx + dy * dy).sqrt();
      e -= bi.mass * bj.mass / dist;
    }
  }
  e
}

// fn offset_momentum(bodies: &mut [Planet; N_BODIES]) {
//   let mut px = 0.0;
//   let mut py = 0.0;
//   let mut pz = 0.0;
//   for bi in bodies.iter() {
//     px += bi.vx * bi.mass;
//     py += bi.vy * bi.mass;
//     pz += bi.vz * bi.mass;
//   }
//   let sun = &mut bodies[0];
//   sun.vx = -px / SOLAR_MASS;
//   sun.vy = -py / SOLAR_MASS;
//   sun.vz = -pz / SOLAR_MASS;
// }

fn offset_momentum(bodies: &mut [Point; N_BODIES]) {
  let mut px = 0.0;
  let mut py = 0.0;
  for bi in bodies.iter() {
    px += bi.vx * bi.mass;
    py += bi.vy * bi.mass;
  }
  let sun = &mut bodies[0];
  sun.vx = -px / SOLAR_MASS;
  sun.vy = -py / SOLAR_MASS;
}

#[no_mangle]
pub unsafe extern "C" fn init() {
  // offset_momentum(&mut BODIES);
}

#[no_mangle]
pub unsafe extern "C" fn step() -> f64 {
  advance(&mut BODIES, 0.01);
  energy(&BODIES)
}

#[no_mangle]
pub unsafe extern "C" fn bench(steps: i32) {
  for _ in 0..steps {
    //advance(&mut BODIES, 0.01);
    advance(&mut BODIES, 0.01);
  }
}

/// Pop a mutable reference off the head of a slice, mutating the slice to no
/// longer contain the mutable reference.
fn shift_mut_ref<'a, T>(r: &mut &'a mut [T]) -> Option<&'a mut T> {
  if r.len() == 0 {
    return None;
  }
  let tmp = core::mem::replace(r, &mut []);
  let (h, t) = tmp.split_at_mut(1);
  *r = t;
  Some(&mut h[0])
}

fn main() {
  unsafe {
    init();
    bench(1000);
  }
}