const steps = process.argv.length > 2 ? parseInt(process.argv[2], 10) : 1000;

require('../C/c.js')((nbodyC) => {

    const start = Date.now();
    nbodyC.init();
    nbodyC.bench(steps);
    console.log('Done in ' + (Date.now() - start) + ' ms');
});