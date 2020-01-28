N-body system
=============

An [AssemblyScript](http://assemblyscript.org) example. This is actually a benchmark - visualizing it just so happened.

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

Benchmark
=========

***Environment:***
- MacBook Pro (Retina, 13-inch, Late 2017)
- macOS 10.13.6
- node.js v10.15.1
- rustc 1.33.0-beta.10


***02.2019 Results: 1000 steps, 1000 bodies, 2D ***

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

***01.2020 Results: 1000 steps, 1000 bodies, 2D ***
```
Performing 1000 steps (C WASM) ...
Took 3043.500668ms
Performing 1000 steps (AssemblyScript WASM) ...
Took 8436.071153ms
Performing 1000 steps (AssemblyScript ASMJS) ...
Took 6020.371279ms
Performing 1000 steps (JS) ...
Took 3389.66475ms
Performing 1000 steps (Rust WASM) ...
Took 3847.223546ms

WARMED UP SERIES:

Performing 1000 steps (C WASM) ...
Took 3206.852877ms
Performing 1000 steps (AssemblyScript WASM) ...
Took 8855.058209ms
Performing 1000 steps (AssemblyScript ASMJS) ...
Took 6074.051566ms
Performing 1000 steps (JS) ...
Took 3851.311174ms
Performing 1000 steps (Rust WASM) ...
Took 3893.41615ms
```

There's a substantial drop in AssemblyScript WASM performance.
The code didn't change at all.

___* unminified___
