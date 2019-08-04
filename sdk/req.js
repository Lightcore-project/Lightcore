const conf = require('./config');
const axios = require('axios');
axios.default.headers = {
  'content-type': "application/json"
};

const req = async data => {
  let { method, params } = data;
  let json = {
    "id": "1",
    "method": method,
    "jsonrpc": "2.0",
    "params": params
  }

  return axios.post(conf.rpcURL, json);
}

module.exports = req;
