var Board = function (dimX, dimY) {
  this.dimX = dimX;
  this.dimY = dimY;

  // enter start coordinates as an array - [i, j]
  var player1StartPos = [Math.floor(dimY/2), Math.floor(7*dimX/8)] ;
  this.player1 = new Bike(this, player1StartPos, "W");

  var player2StartPos = [Math.floor(dimY/2), Math.floor(dimX/8)];
  this.player2 = new Bike(this, player2StartPos, "E");

  this.player1.opponent = this.player2;
  this.player2.opponent = this.player1;

  // take the difficulty that was defined on a previous game if one exists
  this.difficulty = window.difficulty ? window.difficulty : 1;
};

Board.prototype.validPosition = function (coord) {
  return (coord.i >= 0 && coord.i < this.dimY) &&
         (coord.j >= 0 && coord.j < this.dimX);
};


// so linter doesn't yell at us
var Bike = Bike || {};
