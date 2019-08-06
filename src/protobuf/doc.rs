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
    pub docid: Cow<'a, [u8]>,
    pub doctype: i64,
    pub content: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for Document<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.docid = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(16) => msg.doctype = r.read_int64(bytes)?,
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
        + if self.docid == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.docid).len()) }
        + if self.doctype == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.doctype) as u64) }
        + if self.content == Cow::Borrowed("") { 0 } else { 1 + sizeof_len((&self.content).len()) }
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.docid != Cow::Borrowed(b"") { w.write_with_tag(10, |w| w.write_bytes(&**&self.docid))?; }
        if self.doctype != 0i64 { w.write_with_tag(16, |w| w.write_int64(*&self.doctype))?; }
        if self.content != Cow::Borrowed("") { w.write_with_tag(26, |w| w.write_string(&**&self.content))?; }
        Ok(())
    }
}

