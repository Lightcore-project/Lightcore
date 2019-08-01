module.exports = {
  pk: 'jnmhttFe7ngO7ypwJv62Fz9FOsSEXaQ5uwEmfETYYCU=',
  sk: '3PHFXpbidyiQGSN2cNUhNbYXl0Ua1hXXD5K+AvVXc1eOeaG20V7ueA7vKnAm/rYXP0U6xIRdpDm7ASZ8RNhgJQ==',
  mock: {
    doc: {
      doc_id: new Buffer.from('doc_id'),
      type: 1,
      content: 'content'
    },
    tx: {
      object_id: new Buffer.from('object_id'),
      nonce: 42,
      from: new Buffer.from('from'),
      to: new Buffer.from('to'),
      operator: '',
      documents: []
    },
    stx: {
      signature: new Buffer.from(''),
      tx: {}
    }
  }
}
