/**
 All dom definitions and actions
*/
var utils = require('./utils.js');
var consts = require('./consts.js');

var $ = utils.$;

//doms
var scene = $('scene');
var side = $('side');
var info = $('info');
var preview = $('preview');
var level = $('level');
var score = $('score');
var rewardInfo = $('rewardInfo');
var reward = $('reward');
var gameOver = $('gameOver');
var btnRestart = $('restart');
var finalScore = $('finalScore');


//defaults
var SIDE_WIDTH = consts.SIDE_WIDTH;


/**
	Caculate the game container size
*/
var getContainerSize = function(maxW,maxH){

	var dw = document.documentElement.clientWidth;
	var dh = document.documentElement.clientHeight;

	var size = {};
	if (dw>dh){
		size.height = Math.min(maxH,dh);
		size.width = Math.min(size.height /2 + SIDE_WIDTH,maxW);
	}else{
		size.width = Math.min(maxW,dw);
		size.height =  Math.min(maxH,dh);
	}
	return size;

};


/**
	Layout game elements
*/
var layoutView = function(container,maxW,maxH){
	var size = getContainerSize(maxW,maxH);
	var st = container.style;
	st.height = size.height + 'px';
	st.width = size.width + 'px';
	st.marginTop = (-(size.height/2)) + 'px';
	st.marginLeft = (-(size.width/2)) + 'px';

	//layout scene
	scene.height = size.height;
	scene.width = scene.height / 2;

	var sideW = size.width - scene.width;
	side.style.width = sideW+ 'px';
	if (sideW< SIDE_WIDTH ){
		info.style.width = side.style.width;
	}
	preview.width = 80;
	preview.height = 80;

	gameOver.style.width = scene.width +'px';

}

/**
	Main tetris game view
*/
var tetrisView = {


	init:function(id, maxW,maxH){
	  this.container = $(id);
	  this.scene = scene;
	  this.preview = preview;
	  this.btnRestart = btnRestart;
	  layoutView(this.container,maxW,maxH);
	  this.scene.focus();

	  rewardInfo.addEventListener('animationEnd',function(e){
		 rewardInfo.className = 'invisible';
	  });
	},
	// Update the score 
	setScore:function(scoreNumber){
		score.innerHTML = scoreNumber;	
	},
	// Update the finnal score
	setFinalScore:function(scoreNumber){
		finalScore.innerHTML = scoreNumber;
	},
	// Update the level
	setLevel:function(levelNumber){
		level.innerHTML = levelNumber;
	},
	// Update the extra reward score
	setReward:function(rewardScore){
		if (rewardScore>0){
			reward.innerHTML = rewardScore;
			rewardInfo.className = 'fadeOutUp animated';	
		}else{
			rewardInfo.className = 'invisible';
		}
	},
	// Set game over view
	setGameOver:function(isGameOver){
		gameOver.style.display = isGameOver?'block':'none';
	}
};

module.exports = tetrisView;