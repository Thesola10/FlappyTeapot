const path = require('path');

module.exports = {
    entry: './html/game.js',
    output: {
        path: path.resolve(__dirname, 'pkg'),
        filename: 'bundle.js'
    }
}
