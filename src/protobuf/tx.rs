// Automatically generated rust module for 'tx.proto' file
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
pub struct Transaction<'a> {
    pub object_id: Cow<'a, [u8]>,
    pub nonce: i64,
    pub from: Cow<'a, [u8]>,
    pub to: Cow<'a, [u8]>,
    pub operator: Cow<'a, str>,
    pub documents: Vec<doc::Document<'a>>,
}

impl<'a> MessageRead<'a> for Transaction<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.object_id = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(16) => msg.nonce = r.read_int64(bytes)?,
                Ok(26) => msg.from = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(34) => msg.to = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(42) => msg.operator = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(50) => msg.documents.push(r.read_message::<doc::Document>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Transaction<'a> {
    fn get_size(&self) -> usize {
        0
            + if self.object_id == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.object_id).len()) }
        + if self.nonce == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.nonce) as u64) }
        + if self.from == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.from).len()) }
        + if self.to == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.to).len()) }
        + if self.operator == Cow::Borrowed("") { 0 } else { 1 + sizeof_len((&self.operator).len()) }
        + self.documents.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.object_id != Cow::Borrowed(b"") { w.write_with_tag(10, |w| w.write_bytes(&**&self.object_id))?; }
        if self.nonce != 0i64 { w.write_with_tag(16, |w| w.write_int64(*&self.nonce))?; }
        if self.from != Cow::Borrowed(b"") { w.write_with_tag(26, |w| w.write_bytes(&**&self.from))?; }
        if self.to != Cow::Borrowed(b"") { w.write_with_tag(34, |w| w.write_bytes(&**&self.to))?; }
        if self.operator != Cow::Borrowed("") { w.write_with_tag(42, |w| w.write_string(&**&self.operator))?; }
        for s in &self.documents { w.write_with_tag(50, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct SignedTransaction<'a> {
    pub signature: Cow<'a, [u8]>,
    pub tx: Option<Transaction<'a>>,
}

impl<'a> MessageRead<'a> for SignedTransaction<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.signature = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.tx = Some(r.read_message::<Transaction>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for SignedTransaction<'a> {
    fn get_size(&self) -> usize {
        0
            + if self.signature == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.signature).len()) }
        + self.tx.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.signature != Cow::Borrowed(b"") { w.write_with_tag(10, |w| w.write_bytes(&**&self.signature))?; }
        if let Some(ref s) = self.tx { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}
