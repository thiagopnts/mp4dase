use bytes::Bytes;
use log::debug;

use crate::boxes::*;

// literal spec type representations
struct BoxHeader {
    kind: [u8; 4], // b'moov', b'trak', etc
    user_kind: Option<[u8; 16]>,
    large_size: Option<u64>,
    size: u32,
}

#[derive(Debug, Clone)]
pub enum BoxType {
    Smhd(SmhdBox),
    Stco(StcoBox),
    Co64(StcoBox),
    Stsz(StszBox),
    Stsc(StscBox),
    Stss(StssBox),
    Ilst(IlstBox),
    Meta(MetaBox),
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
    Udta(UdtaBox),
    Mdat(MdatBox),
    Unknown(String),
}

pub const SIZE_LEN: u32 = 4;
pub const TYPE_LEN: u32 = 4;
pub const BOX_HEADER_SIZE: usize = 8;

impl BoxType {
    pub fn filter_box<T>(boxes: &Vec<BoxType>, matcher: impl Fn(BoxType) -> Option<T>) -> Vec<T> {
        boxes.iter().filter_map(|b| matcher(b.clone())).collect()
    }

    pub fn find_box<T>(boxes: &Vec<BoxType>, matcher: impl Fn(BoxType) -> Option<T>) -> Option<T> {
        boxes.iter().find_map(|b| matcher(b.clone()))
    }

    pub fn parse_version(buf: &mut bytes::Bytes) -> u8 {
        buf.get_u8()
    }

    pub fn parse_flags(buf: &mut bytes::Bytes) -> u32 {
        buf.get_u24()
    }

    pub fn parse_field_by_version(version: u8, buf: &mut bytes::Bytes) -> u64 {
        if version == 1 {
            return buf.get_u64();
        }
        buf.get_u32() as u64
    }

    pub fn parse_header(buf: &mut bytes::Bytes) -> (usize, [u8; 4]) {
        let size = buf.get_u32();
        let typ = buf.get_u32().to_be_bytes();
        ((size - SIZE_LEN - TYPE_LEN) as usize, typ)
    }

    pub fn peek_header(buf: &bytes::Bytes) -> (usize, [u8; 4]) {
        let mut header_view = buf.slice(0..BOX_HEADER_SIZE);
        Self::parse_header(&mut header_view)
    }

    pub fn peek_box_buf(buf: &mut bytes::Bytes) -> (usize, Bytes, [u8; 4]) {
        let mut header_buf = buf.slice(0..BOX_HEADER_SIZE);
        let (size, typ) = Self::parse_header(&mut header_buf);
        let box_buf = buf.slice(0..size);
        (size, box_buf, typ)
    }

    pub fn parse_box_buf(buf: &mut bytes::Bytes) -> (Bytes, [u8; 4]) {
        let (size, box_buf, typ) = Self::peek_box_buf(buf);
        buf.advance(size);
        (box_buf, typ)
    }

    pub fn parse(buf: &mut bytes::Bytes) -> BoxType {
        let (box_buf_size, typ_bytes) = Self::parse_header(buf);
        // size of the box minus the size and type fields
        let mut box_buf = buf.slice(0..box_buf_size);
        buf.advance(box_buf_size);
        let boxtype = match &typ_bytes {
            b"ftyp" => {
                debug!("parsing ftyp, box buf size {}", box_buf_size);
                let ftyp = FtypBox::parse(&mut box_buf);
                BoxType::Ftyp(ftyp)
            }
            b"stsd" => {
                debug!("parsing stsd");
                let stsd = StsdBox::parse(&mut box_buf);
                BoxType::Stsd(stsd)
            }
            b"dinf" => {
                debug!("parsing dinf");
                let dinf = DinfBox::parse(&mut box_buf);
                BoxType::Dinf(dinf)
            }
            b"moov" => {
                debug!("parsing moov");
                let moov = MoovBox::parse(&mut box_buf).expect("error parsing moov");
                BoxType::Moov(moov)
            }
            b"mvhd" => {
                debug!("parsing mvhd");
                let mvhd = MvhdBox::parse(&mut box_buf);
                debug!("parsed {}, ", mvhd);
                BoxType::Mvhd(mvhd)
            }
            b"iods" => {
                debug!("parsing iods");
                let iods = IodsBox::parse(&mut box_buf);
                debug!("parsed {}", iods);
                BoxType::Iods(iods)
            }
            b"stbl" => {
                debug!("parsing stbl");
                let stbl = StblBox::parse(&mut box_buf);
                BoxType::Stbl(stbl)
            }
            b"trak" => {
                debug!("parsing trak");
                let trak = TrakBox::parse(&mut box_buf);
                BoxType::Trak(trak)
            }
            b"stts" => {
                debug!("parsing stts");
                let stts = SttsBox::parse(&mut box_buf);
                BoxType::Stts(stts)
            }
            b"tkhd" => {
                debug!("parsing tkhd");
                let tkhd = TkhdBox::parse(&mut box_buf);
                debug!("parsed {}", tkhd);
                BoxType::Tkhd(tkhd)
            }
            b"mdia" => {
                debug!("parsing mdia");
                let mdia = MdiaBox::parse(&mut box_buf);
                BoxType::Mdia(mdia)
            }
            b"mdhd" => {
                debug!("parsing mdhd");
                let mdhd = MdhdBox::parse(&mut box_buf);
                BoxType::Mdhd(mdhd)
            }
            b"hdlr" => {
                debug!("parsing hdlr");
                let hdlr = HdlrBox::parse(&mut box_buf);
                BoxType::Hdlr(hdlr)
            }
            b"vmhd" => {
                debug!("parsing vmhd");
                let vmhd = VmhdBox::parse(&mut box_buf);
                BoxType::Vmhd(vmhd)
            }
            b"minf" => {
                debug!("parsing minf");
                let minf = MinfBox::parse(&mut box_buf);
                BoxType::Minf(minf)
            }
            b"dref" => {
                debug!("parsing dref");
                let dref = DrefBox::parse(&mut box_buf);
                BoxType::Dref(dref)
            }
            b"udta" => {
                debug!("parsing udta");
                let udta = UdtaBox::parse(&mut box_buf);
                BoxType::Udta(udta)
            }
            b"meta" => {
                debug!("parsing meta");
                let meta = MetaBox::parse(&mut box_buf);
                BoxType::Meta(meta)
            }
            b"ilst" => {
                debug!("parsing ilst");
                let ilst = IlstBox::parse(&mut box_buf);
                BoxType::Ilst(ilst)
            }
            b"stss" => {
                debug!("parsing stss");
                let stss = StssBox::parse(&mut box_buf);
                BoxType::Stss(stss)
            }
            b"stsc" => {
                debug!("parsing stsc");
                let stsc = StscBox::parse(&mut box_buf);
                BoxType::Stsc(stsc)
            }
            b"stsz" | b"stz2" => {
                debug!("parsing stsz");
                let stsz = stsz::StszBox::parse(&mut box_buf);
                BoxType::Stsz(stsz)
            }
            b"stco" => {
                debug!("parsing stco");
                let stco = stco::StcoBox::parse(&mut box_buf, ChunkSize::Regular);
                BoxType::Stco(stco)
            }
            b"co64" => {
                debug!("parsing co64");
                let co64 = stco::StcoBox::parse(&mut box_buf, ChunkSize::Large);
                BoxType::Co64(co64)
            }
            b"smhd" => {
                debug!("parsing smhd");
                let smhd = SmhdBox::parse(&mut box_buf);
                BoxType::Smhd(smhd)
            }
            b"mdat" => {
                debug!("parsing mdat");
                let mdat = MdatBox::parse(&mut box_buf);
                BoxType::Mdat(mdat)
            }
            _ => {
                debug!("unknown box {}", std::str::from_utf8(&typ_bytes).unwrap());
                BoxType::Unknown(std::str::from_utf8(&typ_bytes).unwrap().to_string())
            }
        };
        return boxtype;
    }
}
