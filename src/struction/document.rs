use sha3::Digest;
use sha3::Sha3_256;

use crate::error::Error;

pub enum DocType {
    PlainText,
    RDFTurle,
    RDFXML,
    RDFJsonLD
}

pub struct Document {
    doc_id: [u8;32],
    doctype: DocType,
    content: String,
}

impl Document {
    fn new(t: DocType, content: &str) -> Self {
        let mut hash = <[u8;32]>::default();
        let mut hasher = Sha3_256::new();
        hasher.input(content);
        hash.copy_from_slice(&hasher.result()[..]);

        Document {
            doc_id: hash,
            doctype: t,
            content: String::from(content),
        }
    }

    fn check_type() -> bool {
        // need TODO
        true
    }

    fn convert(t: DocType) -> Result<Self,Error> {
        Err(Error::NotImpl)
    }
}

