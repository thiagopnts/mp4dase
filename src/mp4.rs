use bytes::Buf;
use log::debug;
use std::{
    io::{self, ErrorKind, Read},
    str,
};

use crate::{boxes::{BoxType, FtypBox, MdatBox, MoovBox}, track::Content};
use crate::track::Track;

#[allow(dead_code)]
pub struct Mp4File {
    ftyp: FtypBox,
    moov: MoovBox,
    mdat: Option<MdatBox>,
    //imdat: Option<ImdatBox>,
}


impl Mp4File {
    pub fn tracks(&self) -> Vec<Track> {
        self.moov
            .traks
            .iter()
            .map(|trak| {
                let track_id = trak.tkhd.track_id;
                let duration = trak.tkhd.duration;
                let media_type = trak.mdia.hdlr.handler_type.to_be_bytes();
                let stsd = BoxType::find_box(&trak.mdia.minf.boxes, |b| match b {
                    BoxType::Stbl(stbl) => Some(BoxType::find_box(&stbl.boxes, |b| match b {
                        BoxType::Stsd(stsd) => Some(stsd),
                        _ => None,
                    })
                    .expect("missing stsd box")),
                    _ => None,
                }).unwrap();
                let format = stsd.format();
                return Track {
                    track_id,
                    format,
                    duration,
                    kind: Content::from(&media_type),
                };
            })
            .collect()
    }
    pub fn parse<R: Read>(reader: &mut R) -> io::Result<Mp4File> {
        let mut stream = bytes::BytesMut::new();
        let mut buffer = [0; 10 * 1024];
        loop {
            match reader.read(&mut buffer) {
                Ok(0) => break,
                Ok(n) => stream.extend_from_slice(&buffer[..n]),
                Err(ref e) if e.kind() == ErrorKind::Interrupted => continue,
                Err(e) => return Err(e),
            };
        }

        let mut stream = stream.freeze();
        let mut file_level_boxes = Vec::new();
        debug!("parsing top level boxes");
        while stream.has_remaining() {
            file_level_boxes.push(BoxType::parse(&mut stream));
        }
        let ftyp = BoxType::find_box(&file_level_boxes, |b| match b {
            BoxType::Ftyp(ftyp) => Some(ftyp),
            _ => None,
        })
        .expect("missing ftyp box");
        debug!("found ftyp");
        let moov = BoxType::find_box(&file_level_boxes, |b| match b {
            BoxType::Moov(moov) => Some(moov),
            _ => None,
        })
        .expect("missing moov box");
        debug!("found moov");

        let mdat = BoxType::find_box(&file_level_boxes, |b| match b {
            BoxType::Mdat(mdat) => Some(mdat),
            _ => None,
        });

        Ok(Mp4File { ftyp, moov, mdat })
    }
}
