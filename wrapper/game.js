var imported = { imports: { imported_func: arg => console.log(arg) } };

WebAssembly.instantiateStreaming(fetch('game.wasm'), imported)
           .then(obj => obj.instance.exports.start());
