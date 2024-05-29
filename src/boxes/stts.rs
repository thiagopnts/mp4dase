use bytes::Buf;
use crate::buf_ext::BufExt;

pub struct SttsBox {
    pub view: bytes::Bytes,
    pub version: u8,
    pub flags: u32,
    pub entry_count: u32,
    pub entries: Vec<SttsEntry>,
}

pub struct SttsEntry {
    pub sample_count: u32,
    pub sample_delta: u32,
}

impl SttsBox {
    pub fn parse(buf: &mut bytes::Bytes) -> Self {
        let view = buf.slice(0..buf.len());
        let version = buf.get_u8();
        let flags = buf.get_u24();
        let entry_count = buf.get_u32();
        let mut entries = Vec::new();

        for _ in 0..entry_count {
            let sample_count = buf.get_u32();
            let sample_delta = buf.get_u32();
            entries.push(SttsEntry {
                sample_count,
                sample_delta,
            });
        }

        SttsBox {
            view,
            version,
            flags,
            entry_count,
            entries,
        }
    }
}
