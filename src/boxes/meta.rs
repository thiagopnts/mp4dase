use super::{BoxType, HdlrBox, BOX_HEADER_SIZE};
use bytes::Buf;
use crate::buf_ext::BufExt;

#[derive(Debug, Clone)]
pub struct MetaBox {
    view: bytes::Bytes,
    version: u8,
    flags: u32,
    handler: HdlrBox,
    other_boxes: Vec<BoxType>,
}

impl MetaBox {
    pub fn parse(buf: &mut bytes::Bytes) -> Self {
        let view = buf.slice(0..buf.len());
        let version = buf.get_u8();
        let flags = buf.get_u24();
        let mut other_boxes = Vec::new();
        let mut handler_box: Option<HdlrBox> = None;
        while buf.len() > BOX_HEADER_SIZE {
            let boxtyp = BoxType::parse(buf);
            if let BoxType::Hdlr(hdlr) = boxtyp {
                handler_box = Some(hdlr);
            } else {
                other_boxes.push(boxtyp);
            }
        }
        let handler = handler_box.expect("meta box missing child hdlr box");
        Self {
            view,
            flags,
            handler,
            version,
            other_boxes,
        }
    }
}
