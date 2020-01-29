import path from 'path';

declare const __static: string; // eslint-disable-line

const WEBSITE_DOMAIN = 'bloom.sh';


// eslint disables are due to __static being marked as not declared
export default {
  TRAY_ICON: path.join(__static, 'assets', 'icons', 'tray.png'), // eslint-disable-line
  WINDOW_TITLE: 'Bloom',
  WINDOW_ICON: path.join(__static, 'bloom_1024.png'), // eslint-disable-line
  WINDOW_URL: 'http://localhost:8080',
  WINDOW_DEFAULT_WIDTH: 900,
  WINDOW_DEFAULT_HEIGHT: 700,
  WINDOW_MIN_WIDTH: 600,
  WINDOW_MIN_HEIGHT: 600,
  WEBSITE_URL: `https://${WEBSITE_DOMAIN}`,
  TERMS_URL: `https://${WEBSITE_DOMAIN}/terms`,
  PRIVACY_URL: `https://${WEBSITE_DOMAIN}/privacy`,
  HELP_URL: `https://help.${WEBSITE_DOMAIN}`,
};
