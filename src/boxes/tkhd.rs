use crate::buf_ext::BufExt;
use bytes::Buf;

use std::fmt::{self, Display};

use super::{BoxType, BOX_HEADER_SIZE};

pub struct TkhdBox {
    pub view: bytes::Bytes,
    version: u8,
    flags: u32,
    creation_time: u64,
    modification_time: u64,
    track_id: u32,
    duration: u64,
    layer: u16,
    alternate_group: u16,
    volume: u16,
    matrix: [u32; 9],
    width: u32,
    height: u32,
}

impl Display for TkhdBox {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "tkhd:\nversion: {}\nflags:{}\ncreation time: {}\nmodification time: {}\ntrack id: {}\nduration: {}\nlayer: {}\nalternate group: {}\nvolume: {}\nmatrix: {:?}\nwidth: {}\nheight: {}",
            self.version,
            self.flags,
            self.creation_time,
            self.modification_time,
            self.track_id,
            self.duration,
            self.layer,
            self.alternate_group,
            self.volume,
            self.matrix,
            self.width,
            self.height
        )
    }
}

impl TkhdBox {
    pub fn parse(buf: &mut bytes::Bytes) -> Self {
        let view = buf.slice(0..buf.len());
        let version = buf.get_u8();
        let flags = buf.get_u24();
        let creation_time = BoxType::parse_field_by_version(version, buf);
        let modification_time = BoxType::parse_field_by_version(version, buf);
        let track_id = buf.get_u32();
        // skip 4 bytes from reserved field
        buf.advance(4);
        let duration = BoxType::parse_field_by_version(version, buf);
        // skip 8 bytes from reserved field
        buf.advance(8);
        let layer = buf.get_u16();
        let alternate_group = buf.get_u16();
        let volume = buf.get_u16();
        // skip 2 bytes from reserved field
        buf.advance(2);

        let mut matrix = [0u32; 9];
        for i in 0..9 {
            matrix[i] = buf.get_u32();
        }

        let width = buf.get_u32() >> 16;
        let height = buf.get_u32() >> 16;

        return TkhdBox {
            view,
            version,
            flags,
            creation_time,
            modification_time,
            track_id,
            duration,
            layer,
            alternate_group,
            volume,
            matrix,
            width,
            height,
        };
    }
}
