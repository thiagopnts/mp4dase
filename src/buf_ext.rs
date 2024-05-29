use bytes::Buf;


// extend bytes::Bytes with a method to read a 24-bit unsigned integer.
pub trait BufExt: Buf {
    fn get_u24(&mut self) -> u32 {
        let b1 = self.get_u8() as u32;
        let b2 = self.get_u8() as u32;
        let b3 = self.get_u8() as u32;
        return b1 << 16 | b2 << 8 | b3;
    }
}

impl<T: Buf> BufExt for T {}

