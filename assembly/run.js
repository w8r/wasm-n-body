#!/usr/bin/env node
const fs = require('fs');
const path = require('path');

const src = fs.readFileSync(path.join(__dirname, '../build/index.js'), 'utf8')
const scopeJS = {
  require:   () => {},
  exports:   {},
  unchecked: expr => expr
};

const sys = new Function(
  ...Object.keys(scopeJS)
    .concat(src + "\nreturn exports"))(...Object.values(scopeJS)
);

sys.init();
sys.bench(1000);
