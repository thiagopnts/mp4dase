use crate::buf_ext::BufExt;
use bytes::Buf;

#[derive(Debug, Clone)]
pub struct StszBox {
    pub view: bytes::Bytes,
    pub version: u8,
    pub flags: u32,
    pub sample_count: u32,
    sample_size: u32,
    pub sample_sizes: Option<Vec<u32>>,
}

impl StszBox {
    pub fn parse(buf: &mut bytes::Bytes) -> Self {
        let view = buf.slice(0..buf.len());
        let version = buf.get_u8();
        let flags = buf.get_u24();
        let sample_size = buf.get_u32();
        let sample_count = buf.get_u32();
        let (sample_size, sample_sizes) = match sample_size {
            0 => {
                let mut entries = Vec::with_capacity(sample_count as usize);
                for _ in 0..sample_count {
                    entries.push(buf.get_u32());
                } 
                (0, Some(entries))
            }
            _ => (sample_size, None),
        };
        StszBox {
            view,
            version,
            flags,
            sample_count,
            sample_size,
            sample_sizes,
        }
    }
}
