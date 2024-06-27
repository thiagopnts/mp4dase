use std::fs::File;
use log::debug;

mod boxes;
mod buf_ext;
mod mp4;
mod track;

use crate::mp4::Mp4File;

fn main() -> std::io::Result<()> {
    env_logger::init();
    let mut file = File::open("/Users/thiago/Downloads/BigBuckBunny.mp4")?;
    let f = Mp4File::parse(&mut file)?;
    debug!("{}", f.tracks().get(0).unwrap().kind);
    debug!("{}", f.tracks().get(1).unwrap().kind);
    Ok(())
}
