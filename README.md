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

```
$> npm run build
```

Afterwards, run `npm run server` to start a <a href="http://localhost:9080">local server</a>. Should also automatically launch a browser.

To run the benchmark:

```
$> npm run test [1000]
```

Benchmark
=========

***Environment:***
- MacBook Pro (Retina, 13-inch, Late 2017)
- macOS 10.13.6
- node.js v10.15.1
- rustc 1.33.0-beta.10

***Results: 1000 steps, 1000 bodies, 2D ***

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
