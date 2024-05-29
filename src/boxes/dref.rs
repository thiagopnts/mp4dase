use crate::buf_ext::BufExt;
use bytes::Buf;

pub struct DrefBox {
    pub view: bytes::Bytes,
    pub version: u8,
    pub flags: u32,
    pub entry_count: u32,
    pub entries: Vec<DrefEntry>,
}

pub struct DrefEntry {
    pub entry_size: u32,
    pub entry_type: u32,
    pub version: u8,
    pub flags: u32,
    pub url: Option<String>,
}

impl DrefBox {
    pub fn parse(buf: &mut bytes::Bytes) -> Self {
        let view = buf.slice(0..buf.len());
        let version = buf.get_u8();
        let flags = buf.get_u24();
        let entry_count = buf.get_u32();
        let mut entries = Vec::new();

        for _ in 0..entry_count {
            let entry_size = buf.get_u32();
            let entry_type = buf.get_u32();
            let version = buf.get_u8();
            let flags = buf.get_u24();

            let url = if &entry_type.to_be_bytes() == b"url " {
                let mut url_bytes = Vec::new();
                while buf.has_remaining() {
                    url_bytes.push(buf.get_u8());
                }
                Some(String::from_utf8_lossy(&url_bytes).into_owned())
            } else {
                None
            };

            entries.push(DrefEntry {
                entry_size,
                entry_type,
                version,
                flags,
                url,
            });
        }

        DrefBox {
            view,
            version,
            flags,
            entry_count,
            entries,
        }
    }
}
