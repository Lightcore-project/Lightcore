// Automatically generated rust module for 'block.proto' file

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
pub struct BlockHeader<'a> {
    pub height: u64,
    pub timestamp: u32,
    pub miner: Cow<'a, [u8]>,
    pub root: Cow<'a, [u8]>,
}

impl<'a> MessageRead<'a> for BlockHeader<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.height = r.read_uint64(bytes)?,
                Ok(16) => msg.timestamp = r.read_uint32(bytes)?,
                Ok(26) => msg.miner = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(34) => msg.root = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for BlockHeader<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.height == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.height) as u64) }
        + if self.timestamp == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.timestamp) as u64) }
        + if self.miner == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.miner).len()) }
        + if self.root == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.root).len()) }
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.height != 0u64 { w.write_with_tag(8, |w| w.write_uint64(*&self.height))?; }
        if self.timestamp != 0u32 { w.write_with_tag(16, |w| w.write_uint32(*&self.timestamp))?; }
        if self.miner != Cow::Borrowed(b"") { w.write_with_tag(26, |w| w.write_bytes(&**&self.miner))?; }
        if self.root != Cow::Borrowed(b"") { w.write_with_tag(34, |w| w.write_bytes(&**&self.root))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Block<'a> {
    pub header: Option<BlockHeader<'a>>,
    pub txs: Cow<'a, [u8]>,
}

impl<'a> MessageRead<'a> for Block<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.header = Some(r.read_message::<BlockHeader>(bytes)?),
                Ok(18) => msg.txs = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Block<'a> {
    fn get_size(&self) -> usize {
        0
        + self.header.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + if self.txs == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.txs).len()) }
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.header { w.write_with_tag(10, |w| w.write_message(s))?; }
        if self.txs != Cow::Borrowed(b"") { w.write_with_tag(18, |w| w.write_bytes(&**&self.txs))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct SignedBlock<'a> {
    pub id: Cow<'a, [u8]>,
    pub signature: Cow<'a, [u8]>,
    pub block: Cow<'a, [u8]>,
}

impl<'a> MessageRead<'a> for SignedBlock<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.id = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.signature = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(26) => msg.block = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for SignedBlock<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.id == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.id).len()) }
        + if self.signature == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.signature).len()) }
        + if self.block == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.block).len()) }
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != Cow::Borrowed(b"") { w.write_with_tag(10, |w| w.write_bytes(&**&self.id))?; }
        if self.signature != Cow::Borrowed(b"") { w.write_with_tag(18, |w| w.write_bytes(&**&self.signature))?; }
        if self.block != Cow::Borrowed(b"") { w.write_with_tag(26, |w| w.write_bytes(&**&self.block))?; }
        Ok(())
    }
}

