const wasm = require('./nbody');

module.exports = (cb) => {
  wasm.addOnPostRun(() => {
    cb({
      init: wasm._init,
      bench: wasm._bench,
      step: wasm._step
    });
  });
};
