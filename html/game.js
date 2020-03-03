const game = import("../pkg/flappyteapot.js");

game.then(game => {
    game.start();
});
