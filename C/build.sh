#!/usr/bin/env bash
emcc ./nbody.c -s WASM=1 -o ../build/nbody-c.js -O3 -s NO_EXIT_RUNTIME=1  -s EXTRA_EXPORTED_RUNTIME_METHODS='["ccall", "addOnPostRun"]' -s EXPORTED_FUNCTIONS="['_bench','_init', '_step', _main]" -s NO_EXIT_RUNTIME="1" -s ALLOW_MEMORY_GROWTH="1" -s NO_FILESYSTEM="1" -s MODULARIZE=1 -s DEMANGLE_SUPPORT=1
