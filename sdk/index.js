'use strict';
const nacl = require('tweetnacl');
const protobuf = require("protobufjs");
const config = require('./config');
const req = require('./req');

// keypair
const pk = new Buffer.from(config.pk, 'base64');
const sk = new Buffer.from(config.sk, 'base64');
const mock = config.mock;

// protobuf
const pb = async () => {
  let root = await protobuf.load("../proto/tx.proto");

  const doc_model = root.lookupType("Document");
  const raw_tx_model = root.lookupType("Transaction");
  const signed_tx_model = root.lookupType("SignedTransaction");

  let doc = doc_model.create(mock.doc);
  mock.tx.documents = [doc];

  let raw_tx = raw_tx_model.create(mock.tx);
  let raw_tx_buff = raw_tx_model.encode(raw_tx).finish();

  mock.stx.tx = raw_tx_buff;
  mock.stx.signature = new Buffer.from(nacl.sign.detached(raw_tx_buff, sk));

  let signed_tx = signed_tx_model.create(mock.stx);
  let code = signed_tx_model.encode(signed_tx).finish().toString('base64');

  // send code to jsonrpc
  req({
    method: 'sendRawTransaction',
    params: {
      code,
    }
  }).then(r => {
    console.log(r.data);
  });
};

// main
(function() { pb(); })();
