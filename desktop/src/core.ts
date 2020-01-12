import axios from 'axios';

const URL = 'http://localhost:8042/electronPost';

export async function call(method: string, params: any): Promise<any> {
  const payload = JSON.stringify({
    method,
    params,
  });

  const res = await axios.post(URL, payload);
  return res.data;
}

export default { call };
