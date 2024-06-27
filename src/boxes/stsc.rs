use bytes::Buf;
use crate::buf_ext::BufExt;

#[derive(Debug, Clone)]
pub struct StscBox {
    pub view: bytes::Bytes,
    pub version: u8,
    pub flags: u32,
    pub entry_count: u32,
    pub entries: Vec<StscEntry>,
}

#[derive(Debug, Clone)]
pub struct StscEntry {
    first_chunk: u32,
    samples_per_chunk: u32,
    sample_description_index: u32,
}

impl StscBox {
    pub fn parse(buf: &mut bytes::Bytes) -> Self {
        let view = buf.slice(0..buf.len());
        let version = buf.get_u8();
        let flags = buf.get_u24();
        let entry_count = buf.get_u32();
        let mut entries = Vec::with_capacity(entry_count as usize);
        for _ in 0..entry_count {
            entries.push(StscEntry {
                first_chunk: buf.get_u32(),
                samples_per_chunk: buf.get_u32(),
                sample_description_index: buf.get_u32(),
            });
        }
        StscBox {
            view,
            version,
            flags,
            entry_count,
            entries,
        }
    }
}
