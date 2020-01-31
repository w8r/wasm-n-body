#!/usr/bin/env bash
emcc ./nbody.c -s WASM=1 -o ./nbody.js -O3 -s NO_EXIT_RUNTIME=1  -s EXTRA_EXPORTED_RUNTIME_METHODS='["ccall", "addOnPostRun"]' -s EXPORTED_FUNCTIONS="['_bench','_init', '_step']" -s NO_EXIT_RUNTIME="1" -s ALLOW_MEMORY_GROWTH="1" -s NO_FILESYSTEM="1" -s MODULARIZE=1 -s DEMANGLE_SUPPORT=1

# If you want to use the SIMD WASM optimization as in
# comment the line above and uncomment the following:
# emcc ./nbody.c -s WASM=1 -o ./nbody.js -msimd128 -O3 -s NO_EXIT_RUNTIME=1  -s EXTRA_EXPORTED_RUNTIME_METHODS='["ccall", "addOnPostRun"]' -s EXPORTED_FUNCTIONS="['_bench','_init', '_step']" -s NO_EXIT_RUNTIME="1" -s ALLOW_MEMORY_GROWTH="1" -s NO_FILESYSTEM="1" -s MODULARIZE=1 -s DEMANGLE_SUPPORT=1