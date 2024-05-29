use crate::buf_ext::BufExt;
use bytes::Buf;

pub struct VmhdBox {
    pub view: bytes::Bytes,
    pub version: u8,
    pub flags: u32,
    pub graphics_mode: u16,
    pub opcolor: [u16; 3],
}

impl VmhdBox {
    pub fn parse(buf: &mut bytes::Bytes) -> Self {
        let view = buf.slice(0..buf.len());
        let version = buf.get_u8();
        let flags = buf.get_u24();
        let graphics_mode = buf.get_u16();
        let opcolor = [buf.get_u16(), buf.get_u16(), buf.get_u16()];

        VmhdBox {
            view,
            version,
            flags,
            graphics_mode,
            opcolor,
        }
    }
}
