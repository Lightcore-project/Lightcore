const r = require ('./req');

r({
  method: 'tx',
  params: {
    hello: 'world'
  }
}).then(r => {
  console.log(r.data);
});
