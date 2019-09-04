const path = require('path');

module.exports = {
  TRAY_ICON: path.join('public', 'tray_icon.png'),
  WINDOW_TITLE: 'Bloom',
  WINDOW_ICON: path.join('public', 'bloom_1024.png'),
  WINDOW_URL: 'http://localhost:8080',
  WINDOW_DEFAULT_WIDTH: 800,
  WINDOW_DEFAULT_HEIGHT: 610,
  WINDOW_MIN_WIDTH: 640,
  WINDOW_MIN_HEIGHT: 550,
};
