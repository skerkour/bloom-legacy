import axios from 'axios';

const URL = 'http://localhost:8042/electronPost';

async function call(method: string, params: any): Promise<any> {
  const payload = JSON.stringify({
    method,
    params,
  });

  const res = await axios.post(URL, payload);
  return res.data;
}

function toIsoDate(date: string | null): Date | null {
  if (date === null) {
    return null;
  }
  return new Date(date).toISOString() as unknown as Date;
}

export default { call, toIsoDate };
