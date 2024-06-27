use bytes::Buf;
use crate::buf_ext::BufExt;

#[derive(Debug, Clone)]
pub struct StssBox {
    pub view: bytes::Bytes,
    pub version: u8,
    pub flags: u32,
    pub entry_count: u32,
    pub entries: Vec<StssEntry>,
}

#[derive(Debug, Clone)]
pub struct StssEntry {
    pub sample_number: u32,
}

impl StssBox {
    pub fn parse(buf: &mut bytes::Bytes) -> Self {
        let view = buf.slice(0..buf.len());
        let version = buf.get_u8();
        let flags = buf.get_u24();
        let entry_count = buf.get_u32();
        let mut entries = Vec::new();
        for _ in 0..entry_count {
            let sample_number = buf.get_u32();
            entries.push(StssEntry {
                sample_number,
            });
        }
        StssBox {
            view,
            version,
            flags,
            entry_count,
            entries,
        }
    }
}
