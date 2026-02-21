use crate::{PartialDecode, extensions::ReadExt};
use std::io::{self, Read};

impl PartialDecode for i8 {
    type Partial = i8;

    fn decode_from<R>(reader: &mut R) -> io::Result<i8>
    where
        R: Read,
    {
        let byte = reader.read_u8()?;
        Ok(byte as i8)
    }

    fn complete(partial: i8) -> i8 {
        partial
    }
}

macro_rules! signed_implementations {
    ($($type: ty), *) => {
        $(
            impl PartialDecode for $type {
                type Partial = $type;

                fn decode_from<R>(reader: &mut R) -> io::Result<$type>
                where
                    R: Read,
                {
                    let value = reader.read_signed_varint()?;
                    value.try_into().map_err(|_| io::ErrorKind::InvalidData.into())
                }

                fn complete(partial: $type) -> $type {
                    partial
                }
            }
        )*
    };
}

signed_implementations!(i16, i32, i64, isize);

impl PartialDecode for u8 {
    type Partial = u8;

    fn decode_from<R>(reader: &mut R) -> io::Result<u8>
    where
        R: Read,
    {
        reader.read_u8()
    }

    fn complete(partial: u8) -> u8 {
        partial
    }
}

macro_rules! unsigned_implementations {
    ($($type: ty), *) => {
        $(
            impl PartialDecode for $type {
                type Partial = $type;

                fn decode_from<R>(reader: &mut R) -> io::Result<$type>
                where
                    R: Read,
                {
                    let value = reader.read_unsigned_varint()?;
                    value.try_into().map_err(|_| io::ErrorKind::InvalidData.into())
                }

                fn complete(partial: $type) -> $type {
                    partial
                }
            }
        )*
    };
}

unsigned_implementations!(u16, u32, u64, usize);
