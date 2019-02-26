#!/usr/bin/env bash
emcc main.c -s WASM=1 -o nbody.html -s NO_EXIT_RUNTIME=1  -s EXTRA_EXPORTED_RUNTIME_METHODS='["ccall", "addOnPostRun"]' -s EXPORTED_FUNCTIONS="['_bench','_init', '_step']" -s NO_EXIT_RUNTIME="1" -s ALLOW_MEMORY_GROWTH="1" -s NO_FILESYSTEM="1"
