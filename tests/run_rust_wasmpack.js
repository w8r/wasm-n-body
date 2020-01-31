const fs = require('fs');
const steps = process.argv.length > 2 ? parseInt(process.argv[2], 10) : 1000;

(async () => {

    const buf = await fs.promises.readFile(__dirname + '/../build/rust_nbody_wasmpack_bg.wasm');
    const nbodyRS = await WebAssembly.instantiate(new Uint8Array(buf)).then(res => res.instance.exports);
    
    const start = Date.now();
    nbodyRS.init();
    nbodyRS.bench(steps);
    console.log('Done in ' + (Date.now() - start) + ' ms');
})();