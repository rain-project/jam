use std::io::{self, Write};

pub trait WriteExt {
    fn write_u8(&mut self, value: u8) -> io::Result<()>;
    fn write_signed_varint(&mut self, value: i64) -> io::Result<()>;
    fn write_unsigned_varint(&mut self, value: u64) -> io::Result<()>;
}

impl<W> WriteExt for W
where
    W: Write,
{
    fn write_u8(&mut self, value: u8) -> io::Result<()> {
        self.write_all(&[value])
    }

    fn write_signed_varint(&mut self, value: i64) -> io::Result<()> {
        let unsigned = ((value >> 63) ^ (value << 1)) as u64; // ZigZag encoding
        self.write_unsigned_varint(unsigned)
    }

    fn write_unsigned_varint(&mut self, mut value: u64) -> io::Result<()> {
        for _ in 0..8 {
            if value >= 0x80 {
                self.write_u8((value as u8) | 0x80)?;
                value >>= 7;
            } else {
                self.write_u8(value as u8)?;
                return Ok(());
            }
        }

        self.write_u8(value as u8)?;
        Ok(())
    }
}
