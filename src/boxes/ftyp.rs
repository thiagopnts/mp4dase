use bytes::{Buf, Bytes};
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct FtypBox {
    major_brand: String,
    minor_version: u32,
    compatible_brands: Vec<String>,
    view: Bytes,
}

impl Display for FtypBox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ftyp:\nMajor brand: {}\nMinor version: {}\nCompatible brands: {:?}",
            self.major_brand, self.minor_version, self.compatible_brands
        )
    }
}

impl FtypBox {
    pub fn parse(buf: &mut Bytes) -> FtypBox {
        let view = buf.slice(0..buf.len());
        let bytes = &buf.get_u32().to_be_bytes();
        let major_brand = std::str::from_utf8(bytes).unwrap();
        let minor_version = buf.get_u32();
        let mut compatible_brands = Vec::with_capacity(buf.len() / 4);
        while buf.remaining() >= 4 {
            compatible_brands.push(
                std::str::from_utf8(&buf.get_u32().to_be_bytes())
                    .unwrap()
                    .to_string(),
            );
        }
        return FtypBox {
            major_brand: major_brand.to_string(),
            minor_version,
            compatible_brands,
            view,
        };
    }
}

#[test]
fn test_parse() {
    let ftyp_bytes: &[u8] = &[
        0x6d, 0x70, 0x34, 0x32, 0x00, 0x00, 0x00, 0x00, 0x69, 0x73, 0x6f, 0x6d, 0x61, 0x76, 0x63,
        0x31, 0x6d, 0x70, 0x34, 0x32,
    ];
    let mut box_bytes: bytes::Bytes = Bytes::from_static(ftyp_bytes);
    let ftyp = FtypBox::parse(&mut box_bytes);
    assert_eq!(ftyp.major_brand, "mp42");
    assert_eq!(ftyp.minor_version, 0);
    assert_eq!(ftyp.compatible_brands, vec!["isom", "avc1", "mp42"]);
    assert_eq!(ftyp.view, Bytes::from_static(ftyp_bytes));
}
