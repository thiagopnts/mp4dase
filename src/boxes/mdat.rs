
use log::debug;

#[derive(Debug, Clone)]
pub struct MdatBox {
    pub view: bytes::Bytes,
    pub data: bytes::Bytes,
}

impl MdatBox {
    pub fn parse(buf: &mut bytes::Bytes) -> Self {
        //157828719
        debug!("MdatBox::parse with size {}", buf.len());
        let view = buf.slice(0..buf.len());
        let data = buf.slice(0..buf.len());
        MdatBox { view, data }
    }
}
