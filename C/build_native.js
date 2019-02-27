#!/usr/bin/env node

const { readFile, writeFile } = require('fs');
const path = require('path');
const { exec } = require('child_process');

readFile(path.join(__dirname, 'nbody.c'), 'utf8', (err, src) => {
  if (err) throw err;
  src = '/* WARNING: this file is generated, do not edit */\n' + src
    .replace('#include   <emscripten/emscripten.h>', '')
    .replace(/EMSCRIPTEN_KEEPALIVE/g, '');
  writeFile(path.join(__dirname, 'nbody_native.c'), src, 'utf8', (err, res) => {
    if (err) throw err;
    const target = path.join(__dirname, '../build/nbody');
    const source = path.join(__dirname, './nbody_native.c');
    exec(`gcc -o ${target} ${source} -O3`, (err, res) => {
      if (err) throw err;
      console.log('build/nbody compiled');
    });
  });
});
