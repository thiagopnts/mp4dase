use crate::boxes::{BoxType, BOX_HEADER_SIZE};
use bytes::Buf;

#[derive(Debug, Clone)]
pub struct StblBox {
    pub view: bytes::Bytes,
    pub boxes: Vec<BoxType>,
}

impl StblBox {
    pub fn parse(buf: &mut bytes::Bytes) -> Self {
        let view = buf.slice(0..buf.len());
        let mut boxes = Vec::new();

        while buf.remaining() >= BOX_HEADER_SIZE {
            boxes.push(BoxType::parse(buf));
        }

        StblBox { view, boxes }
    }
}
