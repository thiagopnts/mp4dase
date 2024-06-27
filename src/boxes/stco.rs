use crate::buf_ext::BufExt;
use bytes::Buf;

#[derive(Debug, Clone)]
pub struct StcoBox {
    pub view: bytes::Bytes,
    pub version: u8,
    pub flags: u32,
    entry_count: u32,
    chunk_size: ChunkSize,
    chunk_offsets: Vec<u64>,
}

#[derive(Debug, Clone)]
pub enum ChunkSize {
    Regular,
    Large,
}

impl StcoBox {
    pub fn parse(buf: &mut bytes::Bytes, chunk_size: ChunkSize) -> Self {
        let view = buf.slice(0..buf.len());
        let version = buf.get_u8();
        let flags = buf.get_u24();
        let entry_count = buf.get_u32();
        let mut chunk_offsets = Vec::with_capacity(entry_count as usize);
        for _ in 0..entry_count {
            let chunk_offset = match chunk_size {
                ChunkSize::Regular => buf.get_u32() as u64,
                ChunkSize::Large => buf.get_u64(),
            };
            chunk_offsets.push(chunk_offset);
        }
        StcoBox {
            view,
            version,
            flags,
            entry_count,
            chunk_size,
            chunk_offsets,
        }
    }
}
