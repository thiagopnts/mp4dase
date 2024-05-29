use crate::buf_ext::BufExt;
use bytes::{Buf, Bytes};
use std::fmt::Display;

use super::BoxType;

pub struct MvhdBox {
    view: Bytes,
    version: u8,
    flags: u32,
    creation_time: u64,
    modification_time: u64,
    timescale: u32,
    duration: u64,
    // we'll drop the fractional part for now
    volume: u8,
    rate: u32,
    matrix: [u32; 9],
    next_track_id: u32,
}

impl Display for MvhdBox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "mvhd:\nversion: {}\nflags:{}\ncreation time: {}\nmodification time: {}\ntimescale: {}\nduration: {}\nrate: {}\nvolume: {}\nmatrix: {:?}\nnext track id: {}",
            self.version, self.flags, self.creation_time, self.modification_time, self.timescale, self.duration, self.rate, self.volume, self.matrix, self.next_track_id
        )
    }
}

impl MvhdBox {
    pub fn parse(buf: &mut Bytes) -> MvhdBox {
        let view = buf.slice(0..buf.len());
        let version = buf.get_u8();
        let flags = buf.get_u24();
        let creation_time = BoxType::parse_field_by_version(version, buf);
        let modification_time = BoxType::parse_field_by_version(version, buf);
        let timescale = buf.get_u32();
        let duration = BoxType::parse_field_by_version(version, buf);
        let rate = buf.get_u32();
        let volume = (buf.get_u16() >> 8) as u8;
        // skip 10 bytes from reserved field
        buf.advance(10);

        let mut matrix = [0u32; 9];
        for i in 0..9 {
            // TODO: format values to fixed points
            matrix[i] = buf.get_u32();
        }

        // skip 24 bytes from pre_defined field
        buf.advance(24);

        let next_track_id = buf.get_u32();

        return MvhdBox {
            view,
            version,
            flags,
            creation_time,
            modification_time,
            timescale,
            duration,
            rate,
            volume,
            matrix,
            next_track_id,
        };
    }
}
