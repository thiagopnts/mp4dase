use crate::boxes::DrefBox;

use super::BoxType;

pub struct DinfBox {
    pub view: bytes::Bytes,
    pub dref: DrefBox,
}

impl DinfBox {
    pub fn parse(buf: &mut bytes::Bytes) -> Self {
        let view = buf.slice(0..buf.len());
        let atom = BoxType::parse(buf);
        // todo: parse dref box directly from buf instead of through BoxType.
        // DrefBox::parse expects size and type to have been out of the buffer
        // already.
        let BoxType::Dref(dref) = atom else { todo!() };

        DinfBox { view, dref }
    }
}

