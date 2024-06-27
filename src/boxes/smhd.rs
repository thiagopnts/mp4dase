use crate::buf_ext::BufExt;
use bytes::Buf;

#[derive(Debug, Clone)]
pub struct SmhdBox {
    pub view: bytes::Bytes,
    pub version: u8,
    pub flags: u32,
    pub balance: i16,
}
impl SmhdBox {
    pub fn parse(buf: &mut bytes::Bytes) -> Self {
        let view = buf.slice(0..buf.len());
        let version = buf.get_u8();
        let flags = buf.get_u24();
        let balance = buf.get_i16();
        SmhdBox {
            view,
            balance,
            version,
            flags,
        }
    }
}
