use super::{BoxType, BOX_HEADER_SIZE};
use log::debug;
use crate::buf_ext::BufExt;
use bytes::Buf;
use std::str;

#[derive(Debug, Clone)]
pub struct IlstBox {
    view: bytes::Bytes,
    items: Vec<Item>,
}

#[derive(Debug, Clone)]
struct Item {
    name: String,
    data: bytes::Bytes,
}

impl Item {
    fn parse(buf: &mut bytes::Bytes) -> Self {
        let (size, name) = BoxType::parse_header(buf);
        let name = str::from_utf8(&name).expect("invalid utf8 for ilst box item");
        debug!("parsed item size: {}, name: {}", size, name);
        let data = buf.slice(0..size as usize);
        buf.advance(size);
        Self {
            name: name.to_string(),
            data,
        }
    }
}
impl IlstBox {
    pub fn parse(buf: &mut bytes::Bytes) -> Self {
        let view = buf.slice(0..buf.len());
        let mut items = Vec::new();
        while buf.len() >= BOX_HEADER_SIZE {
            items.push(Item::parse(buf));
        }
        Self { view, items }
    }
}
