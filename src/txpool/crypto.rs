// use ed25519_dalek::Digest;
use crate::protobuf::tx::SignedTransaction;

pub struct Signature;

impl Signature {
    pub fn verify_tx(pk: String, stx: SignedTransaction) -> bool {
        let sig = ed25519_dalek::Signature::from_bytes(
            &stx.to_owned().signature.to_vec()
        ).unwrap();

        let public_key = ed25519_dalek::PublicKey::from_bytes(
            &base64::decode(&pk).unwrap()
        ).unwrap();

        let tx = stx.to_owned().tx.to_vec();

        public_key.verify(&tx, &sig).is_ok()
    }
}
