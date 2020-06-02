var Bike = function (board, startPos, dir) {
  this.dir = dir;
  this.turning = false;
  this.board = board;
  this.alive = true;
  this.opponent = null;

  var start = new Coord(startPos[0], startPos[1]);
  this.segments = [start];
};

Bike.DIFFS = {
  "N": new Coord(-1, 0),
  "E": new Coord(0, 1),
  "S": new Coord(1, 0),
  "W": new Coord(0, -1)
};

Bike.prototype.isOccupying = function (coord) {
  var result = false;
  this.segments.forEach(function (segment) {
    if (segment.equals(coord)) {
      result = true;
    }
  });
  return result;
};

Bike.prototype.head = function () {
  return this.segments[this.segments.length - 1];
};

Bike.prototype.isValid = function(coord) {
  // check boundaries on board
  if (!this.board.validPosition(coord)) {
    return false;
  }
  // check if bike runs into itself
  for (var i = 0; i < this.segments.length - 1; i++) {
    if (this.segments[i].equals(coord)) {
      return false;
    }
  }
  // check if bike runs into opponent
  if (this.opponent.isOccupying(coord)) {
    return false;
  }
  return true;
};

Bike.prototype.move = function () {
  var nextCoord = this.head().plus(Bike.DIFFS[this.dir]);
  this.segments.push(nextCoord);

  this.turning = false;
  if (!this.isValid(nextCoord) ) {
    this.alive = false;
  }
};

Bike.prototype.turn = function (dir) {
  // don't allow user to turn directly around in opposite direction
  if (Bike.DIFFS[dir].isOpposite(Bike.DIFFS[this.dir]) || this.turning) {
    return;
  } else {
    this.turning = true;
    this.dir = dir;
  }
};

/// Computer AI
Bike.prototype.computerChangeDir = function () {
  var turningDirs;
  if (this.dir === "N" || this.dir === "S") {
    turningDirs = ["W", "E"];
  } else {
    turningDirs = ["N", "S"];
  }

  // decide the turn to make based on the length of the open path
  var firstDir = turningDirs[0];
  var firstDirPathCount = 0; //for counting the open spaces on this path
  var firstDirCoord = this.head().plus(Bike.DIFFS[firstDir]);

  while (this.isValid(firstDirCoord)) {
    firstDirPathCount += 1;
    // go to next coord and see if it is free
    firstDirCoord = firstDirCoord.plus(Bike.DIFFS[firstDir]);
  }

  var secondDir = turningDirs[1];
  var secondDirPathCount = 0;
  var secondDirCoord = this.head().plus(Bike.DIFFS[secondDir]);

  while (this.isValid(secondDirCoord)) {
    secondDirPathCount += 1;
    secondDirCoord = secondDirCoord.plus(Bike.DIFFS[secondDir]);
  }

  // go with the direction that has the clearest path
  if (firstDirPathCount > secondDirPathCount) {
    this.dir = firstDir;
  } else {
    this.dir = secondDir;
  }
};

Bike.prototype.computerMove = function () {
  var nextCoord = this.head().plus(Bike.DIFFS[this.dir]);

  // make a random turn once in awhile to avoid wall hugging
  if (Math.random() > this.computerTurnFrequency() ) {
    this.computerChangeDir();
  }

  if (this.isValid(nextCoord)) {
    this.segments.push(nextCoord);
  } else {
    this.computerChangeDir();
    nextCoord = this.head().plus(Bike.DIFFS[this.dir]);
    this.segments.push(nextCoord);
  }

  // if still invalid the computer lost
  if (!this.isValid(nextCoord)) {
    this.alive = false;
  }
};

Bike.prototype.computerTurnFrequency = function () {
  // Make the easy difficulty turn more frequently
  if (this.board.difficulty === 1) {
    return 0.95;
  } else if (this.board.difficulty === 2) {
    return 0.98;
  } else {
    return 1.0; //hardest will make no turns and just hug the wall
  }
};

// so linter doesn't yell at us
var Coord = Coord || {};
