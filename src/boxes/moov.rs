use crate::boxes::{IodsBox, MvhdBox, TrakBox, BOX_HEADER_SIZE, SIZE_LEN, TYPE_LEN};
use bytes::{Buf, Bytes};

use super::BoxType;

pub struct MoovBox {
    pub view: Bytes,
    pub boxes: Vec<BoxType>,
}

impl MoovBox {
    pub fn parse(buf: &mut Bytes) -> MoovBox {
        let view = buf.slice(0..buf.len());
        let mut boxes = Vec::new();
        while buf.remaining() >= BOX_HEADER_SIZE {
            boxes.push(BoxType::parse(buf));
        }
        return MoovBox { view, boxes };
    }
}
