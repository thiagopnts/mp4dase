use crate::buf_ext::BufExt;
use bytes::Buf;

#[derive(Debug, Clone)]
pub struct HdlrBox {
    pub view: bytes::Bytes,
    pub version: u8,
    pub flags: u32,
    pub pre_defined: u32,
    pub handler_type: u32,
    pub name: String,
}

impl HdlrBox {
    pub fn parse(buf: &mut bytes::Bytes) -> Self {
        let view = buf.slice(0..buf.len());
        let version = buf.get_u8();
        let flags = buf.get_u24();
        let pre_defined = buf.get_u32();
        let handler_type = buf.get_u32();
        buf.advance(3);

        let mut name_bytes = Vec::new();
        while buf.has_remaining() {
            name_bytes.push(buf.get_u8());
        }
        let name = String::from_utf8_lossy(&name_bytes).into_owned();

        HdlrBox {
            view,
            version,
            flags,
            pre_defined,
            handler_type,
            name,
        }
    }
}
