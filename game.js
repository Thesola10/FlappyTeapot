const game = import("./pkg/flappyteapot.js");

var reader = new FileReader();

reader.readAsArrayBuffer(new File(["wat"], "./assets/teapot_idle.png"));

game.then(game => {
    game.addPngFile(reader.result, "teapot_idle");
    game.start();
});
