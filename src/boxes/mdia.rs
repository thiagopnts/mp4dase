use super::{BoxType, HdlrBox, MdhdBox, MinfBox, BOX_HEADER_SIZE};
use bytes::Buf;

#[derive(Debug, Clone)]
pub struct MdiaBox {
    pub view: bytes::Bytes,
    pub minf: MinfBox,
    pub mdhd: MdhdBox,
    pub hdlr: HdlrBox,
    // elng: ElngBox,
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
        let minf = BoxType::find_box(&boxes, |b| match b {
            BoxType::Minf(minf) => Some(minf),
            _ => None,
        }).expect("mdia missing minf");
        let mdhd = BoxType::find_box(&boxes, |b| match b {
            BoxType::Mdhd(mdhd) => Some(mdhd),
            _ => None,
        }).expect("mdia missing mdhd");
        let hdlr = BoxType::find_box(&boxes, |b| match b {
            BoxType::Hdlr(hdlr) => Some(hdlr),
            _ => None,
        }).expect("mdia missing hdlr");
        return MdiaBox { view, minf, mdhd, hdlr };
    }
}
