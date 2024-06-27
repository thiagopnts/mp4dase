use crate::boxes::{IodsBox, MvhdBox, TrakBox, BOX_HEADER_SIZE, SIZE_LEN, TYPE_LEN};
use bytes::{Buf, Bytes};
use log::error;

use super::{error::BoxParsingError, BoxType};

#[derive(Debug, Clone)]
pub struct MoovBox {
    pub view: Bytes,
    pub mvhd: MvhdBox,
    pub traks: Vec<TrakBox>,
}

impl MoovBox {
    pub fn parse(buf: &mut Bytes) -> Result<MoovBox, BoxParsingError> {
        let view = buf.slice(0..buf.len());
        let mut boxes = Vec::new();
        while buf.remaining() >= BOX_HEADER_SIZE {
            boxes.push(BoxType::parse(buf));
        }
        let mvhd = BoxType::find_box(&boxes, |b| match b {
            BoxType::Mvhd(mvhd) => Some(mvhd),
            _ => None,

        }).expect("mvhd not found in moov");

        let traks = BoxType::filter_box(&boxes, |b| match b {
            BoxType::Trak(trak) => Some(trak),
            _ => None,
        });
        if traks.len() == 0 {
            error!("moov box has no traks");
            return Err(BoxParsingError::InvalidFormat("moov has no traks".into()));
        }
        return Ok(MoovBox { view, mvhd, traks });
    }
}
