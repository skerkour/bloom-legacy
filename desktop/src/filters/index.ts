import truncate from './truncate';
import calendar from './calendar';
import filesize from './filesize';
import date from './date';
import duration from './duration';

export default {
  install(vue: any) {
    vue.filter('truncate', truncate);
    vue.filter('calendar', calendar);
    vue.filter('filesize', filesize);
    vue.filter('date', date);
    vue.filter('duration', duration);
  },
};
