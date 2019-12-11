import moment from 'moment';

export default function (text: string) {
  return moment(text).calendar();
}
