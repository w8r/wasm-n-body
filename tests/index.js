const fs = require("fs");

// Load WASM version
const nbodyAS = require("../assembly/index.js");
const nbodyRS = require("../rust/index.js");

(async () => {
  const buf = await fs.promises.readFile(__dirname + '/../build/rust_nbody_wasmpack_bg.wasm');
  const nbodyRS2 = await WebAssembly.instantiate(new Uint8Array(buf)).then(res => res.instance.exports);

  require('../C/c.js')((nbodyC) => {

    // Load ASMJS version
    var src = fs.readFileSync(__dirname + "/../build/index.asm.js", "utf8")
                .replace(/export [^$]*$/g, "");
  
  
    const nbodyAsmJS = eval(src + ";asmFunc")({
      Int8Array,
      Int16Array,
      Int32Array,
      Uint8Array,
      Uint16Array,
      Uint32Array,
      Float32Array,
      Float64Array,
      Math
    }, {
      abort: () => { throw Error(); }
    }, new ArrayBuffer(0x10000));
  
    // Load JS version
    src = fs.readFileSync(__dirname + "/../build/index.js", "utf8");
    const scopeJS = {
      require:   () => {},
      exports:   {},
      unchecked: expr => expr
    };
  
    const nbodyJS = new Function(
      ...Object.keys(scopeJS).concat(src + "\nreturn exports"))(...Object.values(scopeJS)
    );
  
    function gcCollect() {
      if (global.gc) {
        global.gc();
      }
    }
  
    function sleep(delay) {
      var start = Date.now();
      while (Date.now() < start + delay);
    }
  
    function test(nbody, steps) {
      nbody.init();
      var start = process.hrtime();
      nbody.bench(steps);
      let t = process.hrtime(start);
      gcCollect();
      return t;
    }
  
    function testSpecial(nbody, steps) {
      nbody.init();
      var start = process.hrtime();
      nbody.bench(steps);
      let t = process.hrtime(start);
      gcCollect();
      return t;
    }
  
    var steps = process.argv.length > 2 ? parseInt(process.argv[2], 10) : 1000;
  
    function prologue(name, steps) {
      console.log("Performing " + steps + " steps (" + name + ") ...");
    }
  
    function epilogue(time) {
      console.log("Took " + (time[0] * 1e3 + time[1] / 1e6) + "ms");
    }
  
    console.log("\nCOLD SERIES:\n");
  
    prologue("C WASM", steps);
    epilogue(test(nbodyC, steps));
  
    prologue("AssemblyScript WASM", steps);
    epilogue(testSpecial(nbodyAS, steps));
  
    prologue("AssemblyScript ASMJS", steps);
    epilogue(test(nbodyAsmJS, steps));
  
    prologue("JS", steps);
    epilogue(test(nbodyJS, steps));
  
    prologue("Rust WASM", steps);
    epilogue(test(nbodyRS, steps));
  
    prologue("Rust WASM-pack", steps);
    epilogue(test(nbodyRS2, steps));
  
  
    console.log("\nWARMED UP SERIES:\n");
    sleep(1000);
  
    prologue("C WASM", steps);
    epilogue(test(nbodyC, steps));
  
    prologue("AssemblyScript WASM", steps);
    epilogue(test(nbodyAS, steps));
  
    prologue("AssemblyScript ASMJS", steps);
    epilogue(test(nbodyAsmJS, steps));
  
    prologue("JS", steps);
    epilogue(test(nbodyJS, steps));
  
    prologue("Rust WASM", steps);
    epilogue(test(nbodyRS, steps));
  
    prologue("Rust WASM-pack", steps);
    epilogue(test(nbodyRS2, steps));
  });
  
})();

