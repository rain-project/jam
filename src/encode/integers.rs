use crate::{Encode, StableEncode, extensions::WriteExt};
use std::io::{self, Write};

impl Encode for i8 {
    fn encode_unstable_into<W>(&self, writer: &mut W) -> io::Result<()>
    where
        W: Write,
    {
        writer.write_u8(*self as u8)
    }
}

impl StableEncode for i8 {}

macro_rules! signed_implementations {
    ($($type: ty), *) => {
        $(
            impl Encode for $type {
                fn encode_unstable_into<W>(&self, writer: &mut W) -> io::Result<()>
                where
                    W: Write,
                {
                    writer.write_signed_varint(*self as i64)
                }
            }

            impl StableEncode for $type {}
        )*
    };
}

signed_implementations!(i16, i32, i64);

impl Encode for u8 {
    fn encode_unstable_into<W>(&self, writer: &mut W) -> io::Result<()>
    where
        W: Write,
    {
        writer.write_u8(*self)
    }
}

impl StableEncode for u8 {}

macro_rules! unsigned_implementations {
    ($($type: ty), *) => {
        $(
            impl Encode for $type {
                fn encode_unstable_into<W>(&self, writer: &mut W) -> io::Result<()>
                where
                    W: Write,
                {
                    writer.write_unsigned_varint(*self as u64)
                }
            }

            impl StableEncode for $type {}
        )*
    };
}

unsigned_implementations!(u16, u32, u64);
