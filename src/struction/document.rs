use crate::error::Error;

pub enum DocType {
    PlainText,
    RDFTurle,
    RDFXML,
    RDFJsonLD
}

pub struct Document {
    doctype: DocType,
    content: String,
}

impl Document {
    fn new(t: DocType, content: &str) -> Self {
        Document {
            doctype: t,
            content: String::from(content)
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

