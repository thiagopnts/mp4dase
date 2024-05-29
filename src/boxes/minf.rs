use bytes::Buf;
use std::vec::Vec;
use crate::boxes::{BoxType, BOX_HEADER_SIZE};

pub struct MinfBox {
    pub view: bytes::Bytes,
    pub boxes: Vec<BoxType>,
}

impl MinfBox {
    pub fn parse(buf: &mut bytes::Bytes) -> Self {
        let view = buf.slice(0..buf.len());
        let mut boxes = Vec::new();
        while buf.remaining() >= BOX_HEADER_SIZE {
            boxes.push(BoxType::parse(buf));
        }

        MinfBox {
            view,
            boxes,
        }
    }
}

