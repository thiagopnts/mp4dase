use crate::buf_ext::BufExt;
use bytes::Buf;

use super::BoxType;

#[derive(Debug, Clone)]
pub struct MdhdBox {
    pub view: bytes::Bytes,
    pub version: u8,
    pub flags: u32,
    pub creation_time: u64,
    pub modification_time: u64,
    pub timescale: u32,
    pub duration: u64,
    pub language: u16,
    pub pre_defined: u16,
}

impl MdhdBox {
    pub fn parse(buf: &mut bytes::Bytes) -> Self {
        let view = buf.slice(0..buf.len());
        let version = buf.get_u8();
        let flags = buf.get_u24();
        let creation_time = BoxType::parse_field_by_version(version, buf);
        let modification_time = BoxType::parse_field_by_version(version, buf);
        let timescale = buf.get_u32();
        let duration = BoxType::parse_field_by_version(version, buf);
        let language = buf.get_u16();
        let pre_defined = buf.get_u16();

        MdhdBox {
            view,
            version,
            flags,
            creation_time,
            modification_time,
            timescale,
            duration,
            language,
            pre_defined,
        }
    }
}
