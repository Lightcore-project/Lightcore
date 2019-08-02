// Automatically generated rust module for 'state.proto' file

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(unknown_lints)]
#![allow(clippy)]
#![cfg_attr(rustfmt, rustfmt_skip)]


use std::io::Write;
use std::borrow::Cow;
use std::collections::HashMap;
use quick_protobuf::{MessageRead, MessageWrite, BytesReader, Writer, Result};
use quick_protobuf::sizeofs::*;
use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct StateBytes<'a> {
    pub object_id: Cow<'a, [u8]>,
    pub state: Cow<'a, [u8]>,
    pub matrix: HashMap<Cow<'a, [u8]>, Cow<'a, [u8]>>,
}

impl<'a> MessageRead<'a> for StateBytes<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.object_id = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.state = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(26) => {
                    let (key, value) = r.read_map(bytes, |r, bytes| Ok(r.read_bytes(bytes).map(Cow::Borrowed)?), |r, bytes| Ok(r.read_bytes(bytes).map(Cow::Borrowed)?))?;
                    msg.matrix.insert(key, value);
                }
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for StateBytes<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.object_id == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.object_id).len()) }
        + if self.state == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.state).len()) }
        + self.matrix.iter().map(|(k, v)| 1 + sizeof_len(2 + sizeof_len((k).len()) + sizeof_len((v).len()))).sum::<usize>()
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.object_id != Cow::Borrowed(b"") { w.write_with_tag(10, |w| w.write_bytes(&**&self.object_id))?; }
        if self.state != Cow::Borrowed(b"") { w.write_with_tag(18, |w| w.write_bytes(&**&self.state))?; }
        for (k, v) in self.matrix.iter() { w.write_with_tag(26, |w| w.write_map(2 + sizeof_len((k).len()) + sizeof_len((v).len()), 10, |w| w.write_bytes(&**k), 18, |w| w.write_bytes(&**v)))?; }
        Ok(())
    }
}

