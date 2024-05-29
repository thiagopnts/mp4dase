use bytes::Buf;

use std::fmt::{self, Display};

use super::{BoxType, BOX_HEADER_SIZE};

pub struct TrakBox {
    pub view: bytes::Bytes,
    pub boxes: Vec<BoxType>,
}

impl TrakBox {
    pub fn parse(buf: &mut bytes::Bytes) -> Self {
        let view = buf.slice(0..buf.len());
        let mut boxes = Vec::new();
        while buf.remaining() >= BOX_HEADER_SIZE {
        // TODO: check what are the boxes that can be found in mdia
        // to see if we can/need to parse them directly instead of through BoxType.
            boxes.push(BoxType::parse(buf));
        }
        return TrakBox { view, boxes };
    }
}
