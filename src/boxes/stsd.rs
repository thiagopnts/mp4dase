use bytes::Buf;
use crate::buf_ext::BufExt;

pub struct StsdBox {
    pub view: bytes::Bytes,
    pub version: u8,
    pub flags: u32,
    pub entry_count: u32,
    pub sample_descriptions: Vec<SampleDescription>,
}

pub struct SampleDescription {
    pub data: bytes::Bytes,
}

impl StsdBox {
    pub fn parse(buf: &mut bytes::Bytes) -> Self {
        let view = buf.slice(0..buf.len());
        let version = buf.get_u8();
        let flags = buf.get_u24();
        let entry_count = buf.get_u32();
        let mut sample_descriptions = Vec::new();

        // for _ in 0..entry_count {
        //     let size = buf.get_u32();
        //     buf.advance(size as usize - 4);
        //     sample_descriptions.push(SampleDescription { data });
        // }

        StsdBox {
            view,
            version,
            flags,
            entry_count,
            sample_descriptions,
        }
    }
}

