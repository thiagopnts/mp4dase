use log::debug;

use crate::boxes::{BoxType, DrefBox};

#[derive(Debug, Clone)]
pub struct DinfBox {
    pub view: bytes::Bytes,
    pub dref: DrefBox,
}

impl DinfBox {
    pub fn parse(buf: &mut bytes::Bytes) -> Self {
        let view = buf.slice(0..buf.len());
        let dref = match BoxType::parse(buf) {
            BoxType::Dref(dref) => dref,
            _ => panic!("DinfBox::parse: expected dref box"),
        };
        // parse 'url ' | 'urn '
        // parse optinal 'imdt'
        // parse optinoal snim

        debug!("DinfBox::parse with size {}", buf.len());
        DinfBox { view, dref }
    }
}
