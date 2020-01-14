import axios from 'axios';

const CALL_URL = 'http://localhost:8042/electronCall';

const empty = {};

async function call(method: string, params: any): Promise<any> {
  const message = JSON.stringify({
    method,
    params,
  });

  const res = await axios.post(CALL_URL, message);
  return res.data;
}

function toIsoDate(date: string | null): Date | null {
  if (date === null) {
    return null;
  }
  return new Date(date).toISOString() as unknown as Date;
}

async function init(): Promise<void> {
  return call('core.init', empty);
}

export default {
  call,
  toIsoDate,
  empty,
  init,
};
