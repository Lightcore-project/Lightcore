// Automatically generated rust module for 'doc.proto' file

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(unknown_lints)]
#![allow(clippy)]
#![cfg_attr(rustfmt, rustfmt_skip)]


use std::io::Write;
use std::borrow::Cow;
use quick_protobuf::{MessageRead, MessageWrite, BytesReader, Writer, Result};
use quick_protobuf::sizeofs::*;
use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Document<'a> {
    pub doc_id: Cow<'a, [u8]>,
    pub type_pb: mod_Document::DocType,
    pub content: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for Document<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.doc_id = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(16) => msg.type_pb = r.read_enum(bytes)?,
                Ok(26) => msg.content = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Document<'a> {
    fn get_size(&self) -> usize {
        0
            + if self.doc_id == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.doc_id).len()) }
        + if self.type_pb == doc::mod_Document::DocType::PlainText { 0 } else { 1 + sizeof_varint(*(&self.type_pb) as u64) }
        + if self.content == Cow::Borrowed("") { 0 } else { 1 + sizeof_len((&self.content).len()) }
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.doc_id != Cow::Borrowed(b"") { w.write_with_tag(10, |w| w.write_bytes(&**&self.doc_id))?; }
        if self.type_pb != doc::mod_Document::DocType::PlainText { w.write_with_tag(16, |w| w.write_enum(*&self.type_pb as i32))?; }
        if self.content != Cow::Borrowed("") { w.write_with_tag(26, |w| w.write_string(&**&self.content))?; }
        Ok(())
    }
}

pub mod mod_Document {


    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub enum DocType {
        PlainText = 1,
        RDFTurle = 2,
        RDFXML = 3,
        RDFJsonLD = 4,
    }

    impl Default for DocType {
        fn default() -> Self {
            DocType::PlainText
        }
    }

    impl From<i32> for DocType {
        fn from(i: i32) -> Self {
            match i {
                1 => DocType::PlainText,
                2 => DocType::RDFTurle,
                3 => DocType::RDFXML,
                4 => DocType::RDFJsonLD,
                _ => Self::default(),
            }
        }
    }

    impl<'a> From<&'a str> for DocType {
        fn from(s: &'a str) -> Self {
            match s {
                "PlainText" => DocType::PlainText,
                "RDFTurle" => DocType::RDFTurle,
                "RDFXML" => DocType::RDFXML,
                "RDFJsonLD" => DocType::RDFJsonLD,
                _ => Self::default(),
            }
        }
    }
}

