mod merkle;
use std::borrow::Cow;
use std::time::{UNIX_EPOCH, SystemTime};
use ed25519_dalek::{PublicKey, SecretKey, ExpandedSecretKey};
use merkle::MerkleTree;
use quick_protobuf::serialize_into_vec;
use crate::protobuf::tx::SignedTransaction;
use crate::protobuf::block::{Block, SignedBlock, BlockHeader};

fn ts() -> u32 {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH)
        .expect("HaHa, Time went backwards!");
    let in_ms = since_the_epoch.as_secs() * 1000 +
        since_the_epoch.subsec_nanos() as u64 / 1_000_000;

    in_ms as u32
}

fn tx_ser(txs: Vec<SignedTransaction>) -> Vec<Vec<u8>> {
    let mut txs_vec = vec![];
    for i in txs {
        txs_vec.push(serialize_into_vec(&i).unwrap());
    }

    txs_vec
}

fn new_header(pk: PublicKey, txs: Vec<SignedTransaction>) -> BlockHeader {
    // txs
    let txs_vec = tx_ser(txs);
    
    // build merkle tree;
    let tree = MerkleTree::from_vec(txs_vec);

    // return
    BlockHeader {
        height: 9,
        timestamp: ts(),
        miner: Cow::from(pk.as_bytes().to_vec()),
        root: Cow::from(tree.root_hash().to_vec())
    }
}

pub fn new_block(sk: SecretKey, txs: Vec<SignedTransaction>) -> Block {
    let pk = PublicKey::from(&sk);
    let esk = ExpandedSecretKey::from(&sk);

    // txs
    let txs_vec = tx_ser(txs);

    // header
    let header = new_header(pk.to_owned(), txs);

    // return block
    Block {
        header: Some(header),
        txs: Cow::Owned(txs_vec)
    }
}
