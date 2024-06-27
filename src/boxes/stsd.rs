use crate::buf_ext::BufExt;
use bytes::Buf;
use log::debug;

#[derive(Debug, Clone)]
pub struct StsdBox {
    pub view: bytes::Bytes,
    pub version: u8,
    pub flags: u32,
    pub entry_count: u32,
    pub sample_descriptions: Vec<SampleDescription>,
}

#[derive(Debug, Clone)]
pub struct SampleDescription {
    size: u32,
    format: [u8; 4],
}

impl StsdBox {
    pub fn format(&self) -> String {
        // FIXME: need to check how to handle multiple sample descriptions
        return self
            .sample_descriptions
            .iter()
            .map(|sd| String::from_utf8_lossy(&sd.format).to_string())
            .collect::<Vec<String>>()
            .first()
            .expect("sample description format not found")
            .into();
    }

    pub fn parse(buf: &mut bytes::Bytes) -> Self {
        debug!("parsing stsd box");
        let view = buf.slice(0..buf.len());
        let version = buf.get_u8();
        let flags = buf.get_u24();
        let entry_count = buf.get_u32();
        let mut sample_descriptions = Vec::new();

        debug!("parsing {} sample descriptions", entry_count);
        for _ in 0..entry_count {
            let size = buf.get_u32();
            let format = buf.get_u32().to_be_bytes();
            sample_descriptions.push(SampleDescription { size, format });
        }

        StsdBox {
            view,
            version,
            flags,
            entry_count,
            sample_descriptions,
        }
    }
}
