use std::io::{self, Read};

#[allow(dead_code)]
pub(crate) trait ReadExt {
    fn read_u8(&mut self) -> io::Result<u8>;
    fn read_varint(&mut self) -> io::Result<u64>;
}

impl<R> ReadExt for R
where
    R: Read,
{
    fn read_u8(&mut self) -> io::Result<u8> {
        let mut buffer = [0u8];
        self.read_exact(&mut buffer)?;

        let [byte] = buffer;
        Ok(byte)
    }

    fn read_varint(&mut self) -> io::Result<u64> {
        let mut value = 0u64;
        let mut shift = 0;

        for _ in 0..8 {
            let byte = self.read_u8()?;
            value |= ((byte & 0x7f) as u64) << shift;

            if byte & 0x80 == 0 {
                return Ok(value);
            }

            shift += 7;
        }

        let byte = self.read_u8()?;
        value |= (byte as u64) << shift;

        Ok(value)
    }
}
