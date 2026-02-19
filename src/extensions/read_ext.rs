use std::io::{self, Read};

pub trait ReadExt {
    fn read_u8(&mut self) -> io::Result<u8>;
    fn read_signed_varint(&mut self) -> io::Result<i64>;
    fn read_unsigned_varint(&mut self) -> io::Result<u64>;
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

    fn read_signed_varint(&mut self) -> io::Result<i64> {
        let unsigned = self.read_unsigned_varint()?;
        Ok(((unsigned >> 1) as i64) ^ (-((unsigned & 1) as i64))) // ZigZag decoding
    }

    fn read_unsigned_varint(&mut self) -> io::Result<u64> {
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
    use std::{
        cmp::{max, min},
        iter,
    };

    #[test]
    fn signed_bijection() {
        // Positive branch

        let mut value = 0;

        loop {
            let mut buffer = Vec::new();
            buffer.write_signed_varint(value).unwrap();
            buffer.extend(iter::repeat_with(rand::random::<u8>).take(rand::random_range(0..32)));

            assert_eq!(buffer.as_slice().read_signed_varint().unwrap(), value);

            if value == i64::MAX {
                break;
            }

            value = max(value.saturating_add(value / 16), value + 1);
        }

        // Negative branch

        let mut value = 0;

        loop {
            let mut buffer = Vec::new();
            buffer.write_signed_varint(value).unwrap();
            buffer.extend(iter::repeat_with(rand::random::<u8>).take(rand::random_range(0..32)));

            assert_eq!(buffer.as_slice().read_signed_varint().unwrap(), value);

            if value == i64::MIN {
                break;
            }

            value = min(value.saturating_add(value / 16), value - 1);
        }
    }

    #[test]
    fn unsigned_bijection() {
        // Positive branch only

        let mut value = 0;

        loop {
            let mut buffer = Vec::new();
            buffer.write_unsigned_varint(value).unwrap();
            buffer.extend(iter::repeat_with(rand::random::<u8>).take(rand::random_range(0..32)));

            assert_eq!(buffer.as_slice().read_unsigned_varint().unwrap(), value);

            if value == u64::MAX {
                break;
            }

            value = max(value.saturating_add(value / 16), value + 1);
        }
    }
}
