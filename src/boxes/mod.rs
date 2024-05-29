use bytes::Buf;

mod dinf;
mod dref;
mod ftyp;
mod hdlr;
mod iods;
mod mdhd;
mod mdia;
mod minf;
mod moov;
mod mvhd;
mod stbl;
mod stsd;
mod stts;
mod tkhd;
mod trak;
mod vmhd;

pub use dinf::*;
pub use dref::*;
pub use ftyp::*;
pub use hdlr::*;
pub use iods::*;
pub use mdhd::*;
pub use mdia::*;
pub use minf::*;
pub use moov::*;
pub use mvhd::*;
pub use stbl::*;
pub use stsd::*;
pub use stts::*;
pub use tkhd::*;
pub use trak::*;
pub use vmhd::*;

pub enum BoxType {
    Minf(MinfBox),
    Ftyp(FtypBox),
    Moov(MoovBox),
    Iods(IodsBox),
    Mvhd(MvhdBox),
    Trak(TrakBox),
    Tkhd(TkhdBox),
    Mdhd(MdhdBox),
    Hdlr(HdlrBox),
    Mdia(MdiaBox),
    Vmhd(VmhdBox),
    Dref(DrefBox),
    Dinf(DinfBox),
    Stbl(StblBox),
    Stts(SttsBox),
    Stsd(StsdBox),
    Mdat,
}

pub const SIZE_LEN: u32 = 4;
pub const TYPE_LEN: u32 = 4;
pub const BOX_HEADER_SIZE: usize = 8;

impl BoxType {
    pub(crate) fn parse_field_by_version(version: u8, buf: &mut bytes::Bytes) -> u64 {
        if version == 1 {
            return buf.get_u64();
        }
        buf.get_u32() as u64
    }
    pub fn parse(buf: &mut bytes::Bytes) -> BoxType {
        let size = buf.get_u32();
        let typ_bytes = buf.get_u32().to_be_bytes();
        // size of the box minus the size and type fields
        let box_buf_size = (size - SIZE_LEN - TYPE_LEN) as usize;
        let mut box_buf = buf.slice(0..box_buf_size);
        buf.advance(box_buf_size);
        match &typ_bytes {
            b"ftyp" => {
                println!("parsing ftyp, box buf size {}", box_buf_size);
                let ftyp = FtypBox::parse(&mut box_buf);
                return BoxType::Ftyp(ftyp);
            }
            b"stsd" => {
                println!("parsing stsd");
                let stsd = StsdBox::parse(&mut box_buf);
                return BoxType::Stsd(stsd);
            }
            b"dinf" => {
                println!("parsing dinf");
                let dinf = DinfBox::parse(&mut box_buf);
                return BoxType::Dinf(dinf);
            }
            b"moov" => {
                println!("parsing moov");
                let moov = MoovBox::parse(&mut box_buf);
                return BoxType::Moov(moov);
            }
            b"mvhd" => {
                println!("parsing mvhd");
                let mvhd = MvhdBox::parse(&mut box_buf);
                println!("parsed {}, ", mvhd);
                return BoxType::Mvhd(mvhd);
            }
            b"iods" => {
                println!("parsing iods");
                let iods = IodsBox::parse(&mut box_buf);
                println!("parsed {}", iods);
                return BoxType::Iods(iods);
            }
            b"stbl" => {
                println!("parsing stbl");
                let stbl = StblBox::parse(&mut box_buf);
                return BoxType::Stbl(stbl);
            }
            b"trak" => {
                println!("parsing trak");
                let trak = TrakBox::parse(&mut box_buf);
                return BoxType::Trak(trak);
            }
            b"stts" => {
                println!("parsing stts");
                let stts = SttsBox::parse(&mut box_buf);
                return BoxType::Stts(stts);
            }
            b"tkhd" => {
                println!("parsing tkhd");
                let tkhd = TkhdBox::parse(&mut box_buf);
                println!("parsed {}", tkhd);
                return BoxType::Tkhd(tkhd);
            }
            b"mdia" => {
                println!("parsing mdia");
                let mdia = MdiaBox::parse(&mut box_buf);
                return BoxType::Mdia(mdia);
            }
            b"mdhd" => {
                println!("parsing mdhd");
                let mdhd = MdhdBox::parse(&mut box_buf);
                return BoxType::Mdhd(mdhd);
            }
            b"hdlr" => {
                println!("parsing hdlr");
                let hdlr = HdlrBox::parse(&mut box_buf);
                return BoxType::Hdlr(hdlr);
            }
            b"vmhd" => {
                println!("parsing vmhd");
                let vmhd = VmhdBox::parse(&mut box_buf);
                return BoxType::Vmhd(vmhd);
            }
            b"minf" => {
                println!("parsing minf");
                let minf = MinfBox::parse(&mut box_buf);
                return BoxType::Minf(minf);
            }
            b"dref" => {
                println!("parsing dref");
                let dref = DrefBox::parse(&mut box_buf);
                return BoxType::Dref(dref);
            }
            _ => {
                println!("unkown box {}", std::str::from_utf8(&typ_bytes).unwrap());
                return BoxType::Mdat;
            }
        }
    }
}
