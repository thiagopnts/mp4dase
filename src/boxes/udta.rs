
use super::{BoxType, BOX_HEADER_SIZE};

#[derive(Debug, Clone)]
pub struct UdtaBox {
    view: bytes::Bytes,
    boxes: Vec<BoxType>,
}

impl UdtaBox {
    pub fn parse(buf: &mut bytes::Bytes) -> Self {
        let view = buf.slice(0..buf.len());
        let mut boxes = Vec::new();
        while buf.len() > BOX_HEADER_SIZE {
            boxes.push(BoxType::parse(buf));
        }
        UdtaBox { view, boxes }
    }
}
