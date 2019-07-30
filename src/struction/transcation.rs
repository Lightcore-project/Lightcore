use super::document::Document;

pub struct Transcation {
    tx_id: [u8; 32],
    object_id: [u8; 32],
    from: [u8; 32],
    to: [u8; 32],
    operator: String,
    documets: Vec<Document>
}

impl Transcation {

}

