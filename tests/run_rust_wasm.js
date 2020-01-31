const steps = process.argv.length > 2 ? parseInt(process.argv[2], 10) : 1000;
const nbodyRS = require("../rust/index.js");

const start = Date.now();
nbodyRS.init();
nbodyRS.bench(steps);
console.log('Done in ' + (Date.now() - start) + ' ms');