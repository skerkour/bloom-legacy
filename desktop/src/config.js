const path = require('path');

const WEBSITE_DOMAIN = 'bloom.sh';

module.exports = {
  TRAY_ICON: path.join('public', 'assets', 'icons', 'tray.png'),
  WINDOW_TITLE: 'Bloom',
  WINDOW_ICON: path.join('public', 'bloom_1024.png'),
  WINDOW_URL: 'http://localhost:8080',
  WINDOW_DEFAULT_WIDTH: 900,
  WINDOW_DEFAULT_HEIGHT: 700,
  WINDOW_MIN_WIDTH: 600,
  WINDOW_MIN_HEIGHT: 600,
  WEBSITE_URL: `https://${WEBSITE_DOMAIN}`,
  TERMS_URL: `https://${WEBSITE_DOMAIN}/terms`,
  PRIVACY_URL: `https://${WEBSITE_DOMAIN}/privacy`,
  HELP_URL: `https://${WEBSITE_DOMAIN}/help`,
};
