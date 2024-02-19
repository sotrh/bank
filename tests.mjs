import pkg from './main.js';
const { Game, Rolls, GameState } = pkg;

function assert(condition, message="Condition was false") {
    if (!condition) {
        throw message;
    }
}

function assertEq(expected, actual) {
    assert(expected === actual, `expected: ${expected}\n actual: ${actual}`);
}

function test7EndsGame() {
    console.log(pkg);
    console.log(typeof (Game));
    let game = new Game();
    game.createPlayer("Bob");
    game.createPlayer("Jill");

    game.start();

    game.handleRoll(Rolls._7);

    assertEq(70, game.score);

    game.handleRoll(Rolls._7);

    assertEq(140, game.score);

    game.handleRoll(Rolls._7);

    assertEq(210, game.score);

    game.handleRoll(Rolls._7);

    assertEq(GameState.Playing, game.state);
    assertEq(0, game.turn);
}
test7EndsGame();