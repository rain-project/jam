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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::extensions::WriteExt;
    use std::{cmp::max, iter};

    #[test]
    fn bijection() {
        let mut value = 0;

        loop {
            let mut buffer = Vec::new();
            buffer.write_varint(value).unwrap();
            buffer.extend(iter::repeat_with(rand::random::<u8>).take(rand::random_range(0..32)));

            assert_eq!(buffer.as_slice().read_varint().unwrap(), value);

            if value == u64::MAX {
                break;
            }

            value = max(value.saturating_add(value / 16), value + 1);
        }
    }
}
