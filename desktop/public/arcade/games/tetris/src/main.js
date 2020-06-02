var utils = require('./utils.js');
var consts = require('./consts.js');
var shapes = require('./shapes.js');
var views = require('./views.js');
var canvas = require('./canvas.js');




/**
	Init game matrix
*/
var initMatrix = function(rowCount,columnCount){
	var result = [];
	for (var i = 0; i<rowCount;i++){
		var row = [];
		result.push(row);
		for(var j = 0;j<columnCount;j++){
			row.push(0);
		}
	}

	return result;
};

/**
  Clear game matrix
*/
var clearMatrix = function(matrix){
	for(var i = 0;i<matrix.length;i++){
		for(var j = 0;j<matrix[i].length;j++){
			matrix[i][j] = 0;
		}
	}
};


/**
	Check all full rows in game matrix
	return rows number array. eg: [18,19];
*/
var checkFullRows = function(matrix){
	var rowNumbers = [];
  	for(var i = 0;i<matrix.length;i++){
  		var row = matrix[i];
  		var full = true;
  		for(var j = 0;j<row.length;j++){
  			full = full&&row[j]!==0;
  		}
  		if (full){
  			rowNumbers.push(i);
  		}
  	}

  	return rowNumbers;	
};

/**
	Remove one row from game matrix. 
	copy each previous row data to  next row  which row number less than row;
*/
var removeOneRow = function(matrix,row){
	var colCount = matrix[0].length;
	for(var i = row;i>=0;i--){
		for(var j = 0;j<colCount;j++){
			if (i>0){
				matrix[i][j] = matrix[i-1][j];
			}else{
				matrix[i][j] = 0 ;
			}	
		}
	}	
};
/**
	Remove rows from game matrix by row numbers.
*/
var removeRows = function(matrix,rows){
	for(var i in rows){
		removeOneRow(matrix,rows[i]);
	}
};

/**
	Check game data to determin wether the  game is over
*/
var checkGameOver = function(matrix){
	var firstRow = matrix[0];
	for(var i = 0;i<firstRow.length;i++){
		if (firstRow[i]!==0){
			return true;
		};
	}
	return false;
};


/**
	Calculate  the extra rewards add to the score
*/
var calcRewards = function(rows){
	if (rows&&rows.length>1){
		return Math.pow(2,rows.length - 1)*100;	
	}
	return 0;
};

/**
	Calculate game score
*/
var calcScore = function(rows){
	if (rows&&rows.length){
		return rows.length*100;
	}
	return 0;
};

/**
	Calculate time interval by level, the higher the level,the faster shape moves
*/
var calcIntervalByLevel = function(level){
	return consts.DEFAULT_INTERVAL  - (level-1)*60;
};


// Default max scene size
var defaults = {
	maxHeight:700,
	maxWidth:600
};

/**
	Tetris main object defination
*/
function Tetris(id){
	this.id = id;
	this.init();
}

Tetris.prototype = {

	init:function(options){
		
		var cfg = this.config = utils.extend(options,defaults);
		this.interval = consts.DEFAULT_INTERVAL;
		
		
		views.init(this.id, cfg.maxWidth,cfg.maxHeight);

		canvas.init(views.scene,views.preview);

		this.matrix = initMatrix(consts.ROW_COUNT,consts.COLUMN_COUNT);
		this.reset();

		this._initEvents();
		this._fireShape();


	},
	//Reset game
	reset:function(){
		this.running = false;
		this.isGameOver = false;
		this.level = 1;
		this.score = 0;
		this.startTime = new Date().getTime();
		this.currentTime = this.startTime;
		this.prevTime = this.startTime;
		this.levelTime = this.startTime;
		clearMatrix(this.matrix);
		views.setLevel(this.level);
		views.setScore(this.score);
		views.setGameOver(this.isGameOver);
		this._draw();
	},
	//Start game
	start:function(){
		this.running = true;
		window.requestAnimationFrame(utils.proxy(this._refresh,this));
	},
	//Pause game
	pause:function(){
		this.running = false;
		this.currentTime = new Date().getTime();
		this.prevTime = this.currentTime;
	},
	//Game over
	gamveOver:function(){

	},
	// All key event handlers
	_keydownHandler:function(e){
		
		var matrix = this.matrix;

		if(!e) { 
			var e = window.event;
		}
		if (this.isGameOver||!this.shape){
			return;
		}

		switch(e.keyCode){
			case 37:{this.shape.goLeft(matrix);this._draw();}
			break;
			
			case 39:{this.shape.goRight(matrix);this._draw();}
			break;
			
			case 38:{this.shape.rotate(matrix);this._draw();}
			break;

			case 40:{this.shape.goDown(matrix);this._draw();}
			break;

			case 32:{this.shape.goBottom(matrix);this._update();}
			break;
		}
	},
	// Restart game
	_restartHandler:function(){
		this.reset();
		this.start();
	},
	// Bind game events
	_initEvents:function(){
		window.addEventListener('keydown',utils.proxy(this._keydownHandler,this),false);
		views.btnRestart.addEventListener('click',utils.proxy(this._restartHandler,this),false);
	},

	// Fire a new random shape
	_fireShape:function(){
		this.shape = this.preparedShape||shapes.randomShape();
		this.preparedShape = shapes.randomShape();
		this._draw();
		canvas.drawPreviewShape(this.preparedShape);
	},
	
	// Draw game data
	_draw:function(){
		canvas.drawScene(); 
		canvas.drawShape(this.shape);
		canvas.drawMatrix(this.matrix);
	},
	// Refresh game canvas
	_refresh:function(){
		if (!this.running){
			return;
		}
		this.currentTime = new Date().getTime();
		if (this.currentTime - this.prevTime > this.interval ){
			this._update();
			this.prevTime = this.currentTime;
			this._checkLevel();
		}
		if (!this.isGameOver){
			window.requestAnimationFrame(utils.proxy(this._refresh,this));	
		}
	},
	// Update game data
	_update:function(){
		if (this.shape.canDown(this.matrix)){
			this.shape.goDown(this.matrix);
		}else{
			this.shape.copyTo(this.matrix);
			this._check();
			this._fireShape();
		}
		this._draw();
		this.isGameOver = checkGameOver(this.matrix);
		views.setGameOver(this.isGameOver);
		if (this.isGameOver){
			views.setFinalScore(this.score);
		}
	},
	// Check and update game data
	_check:function(){
		var rows = checkFullRows(this.matrix);
		if (rows.length){
			removeRows(this.matrix,rows);
			
			var score = calcScore(rows);
			var reward = calcRewards(rows);
			this.score += score + reward;

			views.setScore(this.score);
			views.setReward(reward);

		}
	},
	// Check and update game level
	_checkLevel:function(){
		var currentTime = new Date().getTime();
		if (currentTime - this.levelTime > consts.LEVEL_INTERVAL){
			this.level+=1;
			this.interval = calcIntervalByLevel(this.level);
			views.setLevel(this.level);
			this.levelTime = currentTime;
		}
	}
}


window.Tetris = Tetris;





