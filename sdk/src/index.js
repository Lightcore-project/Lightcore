'use strict';

var protobuf = require("protobufjs");

protobuf.load("./proto/tx.proto", function(err, root) {
  if (err) throw err;

  let docModel = root.lookupType("Document");
  let rawTxModel = root.lookupType("Transaction");

  let doc = docModel.create({
    doc_id: new Buffer('doc_id'),
    type: 1,
    content: 'content'
  });

  let rawTx = rawTxModel.create({
    object_id: new Buffer('object_id'),
    nonce: 42,
    from: new Buffer('from'),
    to: new Buffer('to'),
    operator: '',
    documents: [doc]
  });

  let buff = rawTxModel.encode(rawTx).finish();
  console.log(buff);
  console.log(rawTxModel.decode(buff));
});
