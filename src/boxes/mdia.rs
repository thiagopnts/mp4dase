use super::{BoxType, BOX_HEADER_SIZE};
use bytes::Buf;

pub struct MdiaBox {
    pub view: bytes::Bytes,
    pub boxes: Vec<BoxType>,
}

impl MdiaBox {
    pub fn parse(buf: &mut bytes::Bytes) -> Self {
        let view = buf.slice(0..buf.len());
        let mut boxes = Vec::new();
        // TODO: check what are the boxes that can be found in mdia
        // to see if we can/need to parse them directly instead of through BoxType.
        while buf.remaining() >= BOX_HEADER_SIZE {
            boxes.push(BoxType::parse(buf));
        }
        return MdiaBox { view, boxes };
    }
}
