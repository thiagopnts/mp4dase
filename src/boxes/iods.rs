use crate::buf_ext::BufExt;
use bytes::Buf;

use std::fmt::{self, Display};

pub struct IodsBox {
    version: u8,
    flags: u32,
}

impl IodsBox {
    pub fn parse(buf: &mut bytes::Bytes) -> IodsBox {
        let version = buf.get_u8();
        let flags = buf.get_u24();
        //TODO: parse descriptors

        return IodsBox { version, flags };
    }
}

impl Display for IodsBox {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "IodsBox  version: {}\nflags: {}",
            self.version, self.flags
        )
    }
}
