N-body system
=============

A WASM workbench for the n-body scenario.

Requirements
------------

* [Node.js](https://nodejs.org/it/) installed (v. 10+ for async/await support)
* [Emscripten](https://emscripten.org/) installed
* [Rust](https://www.rust-lang.org/) installed
* [wasm-pack](https://github.com/rustwasm/wasm-pack) installed

Instructions
------------

First, install the development dependencies:

```
$> npm install
```

Now, to build [assembly/index.ts](./assembly/index.ts) to an untouched and an optimized `.wasm` including their respective `.wat` representations, run:

You will need emscripten installed on your machine (for C &rarr; WASM)
```
$> npm run build
```

To run the benchmark:

```
$> npm run test
```

To run the native test benchmark (optimized C agains nodejs)

```
$> npm run testnative
```

To run the browser test benchmark

```
$> npm run browserbuild
$> npx serve
```

Open the browser and point it to `localhost:5000`

Benchmark
=========

***Environment:***
- MacBook Pro (Retina, 13-inch, Late 2017)
- macOS 10.13.6
- node.js v10.15.1
- rustc 1.33.0-beta.10


***Results: 1000 steps, 1000 bodies, 2D***

```
COLD SERIES:

Performing 1000 steps (AssemblyScript WASM) ...
Took 2932.2897430000003ms
Performing 1000 steps (AssemblyScript ASMJS) ...
Took 3862.352643ms
Performing 1000 steps (JS) ...
Took 2912.77102ms
Performing 1000 steps (Rust WASM) ...
Took 3096.005317ms

WARMED UP SERIES:

Performing 1000 steps (AssemblyScript WASM) ...
Took 3037.334141ms
Performing 1000 steps (AssemblyScript ASMJS) ...
Took 4032.277231ms
Performing 1000 steps (JS) ...
Took 3043.310233ms
Performing 1000 steps (Rust WASM) ...
Took 3340.9384410000002ms

```

___* unminified___

Wants to experiment stuff?
=========================

Wants to test that [new shiny SIMD support in WASM](https://v8.dev/features/simd)?
We got you covered: install the required tools and follow the instructions in `./C/build.sh` to enable it.

Wants to try out this benchmarks? Cool, try them out and let us know the numbers!
