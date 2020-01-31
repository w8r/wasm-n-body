const fs = require('fs');
// it's just one file so they only requires a wrapper around. Pretty easy to do with few lines now
const mappings = [
    {
        title: 'JS version',
        filename: 'index', wrapperCode: (src) => `
    function JSwrapper(require, exports, unchecked) {
        ${src}
        return exports;
    }
    `},
    {
        title: 'ASM version',
        filename: 'index.asm', wrapperCode: (src) => `
    function MyAsmModule(exports) {
        ${src.replace(/export var/g, 'exports.')}
        return exports;
    }
    `}
];

(async () => {

    for await ( const {title, filename, wrapperCode} of mappings){
        console.log('Browserify', title);
        const input = await fs.promises.readFile(__dirname + `/build/${filename}.js`, 'utf8');
        const output = wrapperCode(input);
        await fs.promises.writeFile(__dirname + `/build/${filename}.browser.js`, output);
    }

    console.log('Browser version done')
})()