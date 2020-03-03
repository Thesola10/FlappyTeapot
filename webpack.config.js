const path = require('path');

module.exports = {
    entry: './game.js',
    output: {
        path: path.resolve(__dirname),
        filename: 'pkg/bundle.js'
    }
}
