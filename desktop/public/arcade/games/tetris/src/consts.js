
//colors for shapes
var colors = ['#00af9d','#ffb652','#cd66cc','#66bc29','#0096db','#3a7dda','#ffe100'];

//sidebar width
var sideWidth = 120;


//scene column count
var columnCount = 10;

//scene row count;
var rowCount = 20;

//previewCount
var previewCount = 6;

//scene gradient start color 
var sceneBgStart = '#8e9ba6';

//scene gradient end color 
var sceneBgEnd = '#5c6975';

//preview background color
var previewBg = '#2f2f2f';

//grid line color
var gridLineColor = 'rgba(255,255,255,0.2)';

//box border color
var boxBorderColor = 'rgba(255,255,255,0.5)';


// Game speed
var defaultInterval = 600;


// Level update interval 
var levelInterval = 120 * 1000; 



var exports = module.exports = {};

exports.COLORS =  colors;

exports.SIDE_WIDTH = sideWidth;

exports.ROW_COUNT = rowCount;

exports.COLUMN_COUNT = columnCount;

exports.SCENE_BG_START = sceneBgStart;

exports.SCENE_BG_END = sceneBgEnd;

exports.PREVIEW_BG = previewBg;

exports.PREVIEW_COUNT = previewCount;

exports.GRID_LINE_COLOR = gridLineColor;

exports.BOX_BORDER_COLOR = boxBorderColor;

exports.DEFAULT_INTERVAL = defaultInterval;

exports.LEVEL_INTERVAL = levelInterval;
