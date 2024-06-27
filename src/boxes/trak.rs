use bytes::Buf;

use std::fmt::{self, Display};

use super::{tkhd, BoxType, MdiaBox, TkhdBox, BOX_HEADER_SIZE};

#[derive(Debug, Clone)]
pub struct TrakBox {
    pub view: bytes::Bytes,
    pub tkhd: TkhdBox,
    pub mdia: MdiaBox,
    // tref: Option<TrefBox>,
    // trgr: Option<TrgrBox>,
    // ttyp: Option<TtypBox>,
    pub boxes: Vec<BoxType>,
}

impl TrakBox {
    pub fn parse(buf: &mut bytes::Bytes) -> Self {
        let view = buf.slice(0..buf.len());
        let mut boxes = Vec::new();
        while buf.remaining() >= BOX_HEADER_SIZE {
            boxes.push(BoxType::parse(buf));
        }
        let tkhd = BoxType::find_box(&boxes, |b| match b {
            BoxType::Tkhd(tkhd) => Some(tkhd),
            _ => None,
        }).expect("trak is missing tkhd");
        let mdia = BoxType::find_box(&boxes, |b| match b {
            BoxType::Mdia(mdia) => Some(mdia),
            _ => None,
        }).expect("trak is missing mdia");
        return TrakBox { view, tkhd, mdia, boxes };
    }
}
