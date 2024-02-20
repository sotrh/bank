/* 
    Possible game states

    Note: `Object.freeze()` creates an object that cannot be
    modified. It's useful for data that you don't want to change.

    Note: we could use strings like "Setup", "Playing" instead,
    but tying things to an object helps prevent errors from
    typos.
*/
var GameState = Object.freeze({
  Setup: 0,
  Playing: 1,
  GameOver: 2,
});

var Rolls = Object.freeze({
  Doubles: 0,
  // While numbers can be fields in an object, you can't use
  // the `.` operator with them, so I'll use `_N` instead
  _2: 2,
  _3: 3,
  _4: 4,
  _5: 5,
  _6: 6,
  _7: 7,
  _8: 8,
  _9: 9,
  _10: 10,
  _11: 11,
  _12: 12,
});

class Game {
  state = GameState.Setup;
  score = 0;
  round = 0;
  turn = 0;
  numRounds = 0;
  currentPlayer = 0;
  players = [];
  rolls = [];

  constructor() {}

  start(numRounds = 10) {
    if (this.players.length < 2) {
      alert("You need at least 2 players");
      return;
    }

    // The `this` keyword specifies the object that is calling
    // this function (ie. the current game)
    this.state = GameState.Playing;
    this.score = 0;
    this.round = 0;
    this.turn = 0;
    this.currentPlayer = 0;
    this.numRounds = numRounds;

    for (let player of this.players) {
      player.banked = false;
    }
  }

  roll() {
    function d6() {
      return Math.floor(Math.random() * 6) + 1;
    }

    let a = d6();
    let b = d6();

    if (a === b && this.turn >= 3) {
      this.handleRoll(Rolls.Doubles);
    } else {
      this.handleRoll(a + b);
    }
  }

  handleRoll(roll) {
    if (this.state !== GameState.Playing) {
      alert("The game is not playing");
      return;
    }

    // A `switch` statement is like an `if-else if-else` chain,
    // but it's faster. The syntax is a little weird though.
    // Blame C for that.
    switch (roll) {
      // Normal cases
      case Rolls._2:
      case Rolls._3:
      case Rolls._4:
      case Rolls._5:
      case Rolls._6:
      case Rolls._8:
      case Rolls._9:
      case Rolls._10:
      case Rolls._11:
      case Rolls._12:
        this.score += roll;
        break;
      // 7 adds 70 before turn 3 and ends
      // the round after turn 3
      case Rolls._7:
        if (this.turn < 3) {
          this.score += 70;
        } else {
          this.endRound();
          return;
        }
        break;

      case Rolls.Doubles:
        if (this.turn < 3) {
          alert("Can't roll doubles on turn 3");
          return;
        }
        this.score *= 2;
        break;
    }

    this.turn += 1;
    this.currentPlayer += 1;

    if (this.currentPlayer >= this.players.length) {
      // The `%` operator gets the remainder of a division. It's a
      // great way to keep a number in a particular range. For example
      // if you wanted a number `n` between 0 and 5, you could do
      // ```
      // n = n % 5
      // ```
      // If `n == 7` then `n % 5` would be 2.
      this.currentPlayer = this.currentPlayer % this.players.length;
    }
  }

  createPlayer(name) {
    for (let player of this.players) {
      if (player.name === name) {
        alert(`"${name}" is already in use.`);
        return;
      }
    }

    const player = { name, score: 0, banked: false };
    this.players.push(player);

    return player;
  }

  endRound() {
    // `player` is of type `Player`
    for (let player of this.players) {
      if (player.banked) {
        player.score += this.score;

        // Reset the players `banked` flag for next round
        player.banked = false;
      }
    }

    // reset the current bank score and advance to next round
    this.score = 0;
    this.turn = 0;
    this.round += 1;

    if (this.round == this.numRounds) {
      this.state = GameState.GameOver;
    }
  }

  debugState() {
    switch (this.state) {
      case GameState.Setup:
        console.log("Setup");
        console.log("players:", this.players);
        break;
      case GameState.Playing:
        console.log("Playing");
        console.log("players:", this.players);
        console.log("round:", this.round);
        console.log("bank:", this.score);
        console.log("current player:", this.players[this.currentPlayer]);
        break;
      case GameState.GameOver:
        console.log("GameOver");
        console.log("players:", this.players);
        break;
    }
  }
}

/**
 * Manages UI
 */
class UiManager {
  game = new Game();

  setupListeners() {
    document
      .getElementById("add-players-form")
      .addEventListener("submit", (e) => {
        e.preventDefault();
        this.handlePlayerFormSubmit(e.target);
      });
  }

  clearPlayerForm() {
    const playerForm = document.getElementById("add-players-form");
    playerForm.reset();

    const playerList = document.getElementById("add-players-list");
    while (playerList.lastElementChild) {
      playerList.removeChild(playerList.lastElementChild);
    }
  }

  handlePlayerFormSubmit(form) {
    const formData = new FormData(form);
    const name = formData.get("name");
    const player = this.game.createPlayer(name);

    console.log(player);
    if (!player) return;

    const playerList = document.getElementById("add-players-list");
    const playerDiv = document.createElement("div");
    playerDiv.classList.add("player");
    playerDiv.textContent = player.name;
    playerList.appendChild(playerDiv);

    form.reset();

    document.getElementById("add-players-list-button").disabled =
      this.game.players.length < 2;
  }

  startGame() {
    const currentPlayerText = document.getElementById("current-player");
    const currentPlayer = this.game.players[this.game.currentPlayer];
    currentPlayerText.textContent = `${currentPlayer.name}'s Turn (${currentPlayer.score})`;

    const currentRound = document.getElementById("current-round");
    currentRound.textContent = `Round ${this.game.round + 1}`;

    this.clearPlayerForm();

    const playerList = document.getElementById("play-screen-scores");
    playerList.innerHTML = '';

    for (let player of this.game.players) {
      const div = document.createElement("div");
      playerList.appendChild(div);

      const icon = document.createElement("img");
      icon.src = "#";
      div.appendChild(icon);

      const name = document.createElement("div");
      name.textContent = player.name;
      div.appendChild(name);

      const score = document.createElement("div");
      score.textContent = `(${player.score})`;
      div.appendChild(score);
    }

    this.game.start(10);
  }

  buttonPressed(roll) {
    this.game.handleRoll(roll);
    const bankScore = document.getElementById("bank-score");
    bankScore.textContent = `${this.game.score}`;

    const currentPlayer = document.getElementById("current-player");
    currentPlayerText.textContent = `${currentPlayer.name}'s Turn (${currentPlayer.score})`;

    const currentRound = document.getElementById("current-round");
    currentRound.textContent = `Round ${this.game.round + 1}`;
  }
}

var ui = new UiManager();
ui.setupListeners();
