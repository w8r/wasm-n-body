const wasm = require('./nbody.js');

module.exports = (cb) => {
  wasm()
    .then((exports) => cb({
      init: exports._init,
      bench: exports._bench,
      step: exports._step
    }));
};
