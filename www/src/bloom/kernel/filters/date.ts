import moment from 'moment';

export default function(text: string, format: string) {
  if (format === undefined) {
    format = 'MMMM Do YYYY, HH:mm:ss';
  }
  return moment(text).format(format);
}
