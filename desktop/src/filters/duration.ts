import moment from 'moment';

export default function (ms: number) {
  return moment.utc(ms * 1000).format('HH:mm:ss');
}
