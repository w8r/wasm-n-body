mod utils;

use std::cell::RefCell;
use std::f64;
use std::thread::LocalKey;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

const N_BODIES: usize = 1000;

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
thread_local! {
    static BODIES: RefCell<Vec<Point>> = RefCell::new(Vec::with_capacity(N_BODIES));
}

fn energy() -> f64 {
    let mut e = 0.0;
    BODIES.with(|b| {
        let mut bodiesMut = b.borrow_mut();
        let mut bodies = bodiesMut.iter();
        loop {
            let bi = match bodies.next() {
                Some(bi) => bi,
                None => break,
            };

            e += (bi.vx * bi.vx + bi.vy * bi.vy) * bi.mass / 2.0;
            for bj in bodies.clone() {
                let dx = bi.x - bj.x;
                let dy = bi.y - bj.y;
                let dist = (dx * dx + dy * dy).sqrt();
                e -= bi.mass * bj.mass / dist;
            }
        }
    });
    // let mut bodies = bodies.iter();
    // loop {
    //     let bi = match bodies.next() {
    //         Some(bi) => bi,
    //         None => break,
    //     };

    //     e += (bi.vx * bi.vx + bi.vy * bi.vy) * bi.mass / 2.0;
    //     for bj in bodies.clone() {
    //         let dx = bi.x - bj.x;
    //         let dy = bi.y - bj.y;
    //         let dist = (dx * dx + dy * dy).sqrt();
    //         e -= bi.mass * bj.mass / dist;
    //     }
    // }
    e
}

fn advance(dt: f64) {
    BODIES.with(|b| {
        let mut b_slice: &mut [_] = &mut b.borrow_mut();
        loop {
            let bi = match shift_mut_ref(&mut b_slice) {
                Some(bi) => bi,
                None => break,
            };

            for bj in b_slice.iter_mut() {
                let dx = bi.x - bj.x;
                let dy = bi.y - bj.y;

                let d2 = dx * dx + dy * dy;
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
    });
    // let mut b_slice: &mut [_] = bodies;
    // loop {
    //     let bi = match shift_mut_ref(&mut b_slice) {
    //         Some(bi) => bi,
    //         None => break,
    //     };

    //     for bj in b_slice.iter_mut() {
    //         let dx = bi.x - bj.x;
    //         let dy = bi.y - bj.y;

    //         let d2 = dx * dx + dy * dy;
    //         let mag = dt / (d2 * d2.sqrt());

    //         let massj_mag = bj.mass * mag;
    //         bi.vx -= dx * massj_mag;
    //         bi.vy -= dy * massj_mag;

    //         let massi_mag = bi.mass * mag;
    //         bj.vx += dx * massi_mag;
    //         bj.vy += dy * massi_mag;
    //     }
    //     bi.x += dt * bi.vx;
    //     bi.y += dt * bi.vy;
    // }
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

#[wasm_bindgen]
pub fn init() {
    let mut arr: Vec<Point> = Vec::with_capacity(N_BODIES);
    let mut ox = 0.0;
    let mut oy = 0.0;
    let m = 5.0;
    let add = 1.0;
    for i in 0..N_BODIES {
        if i % 10 == 0 {
            oy += 10.0;
            ox = 0.0;
        }
        ox += add;
        let p = Point {
            x: ox,
            y: oy,
            vx: 0.0,
            vy: 0.0,
            mass: m,
        };
        arr.push(p);
    }
    BODIES.with(|b| *b.borrow_mut() = arr)
}

#[wasm_bindgen]
pub fn step() -> f64 {
    advance(0.01);
    energy()
}

#[wasm_bindgen]
pub fn bench(steps: i32) {
    for _ in 0..steps {
        advance(0.01);
    }
}

fn main() {
    init();
    bench(1000);
}
