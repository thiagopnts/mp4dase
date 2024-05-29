use std::{fs::File, hash::Hash, io::Read};
use bytes::{Buf, Bytes};

mod boxes;
mod buf_ext;
use boxes::BoxType;

fn main() -> std::io::Result<()> {
    let mut file = File::open("./BigBuckBunny.mp4")?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let mut bytes = Bytes::from(buffer);
    println!("{}", bytes.len());
    BoxType::parse(&mut bytes);
    println!("{}", bytes.len());
    BoxType::parse(&mut bytes);
    return Ok(());
}
