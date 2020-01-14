import moment from 'moment';

export default function (text: string, format: string) {
  const fmt = format || 'MMMM Do YYYY, HH:mm:ss';
  return moment(text).format(fmt);
}
