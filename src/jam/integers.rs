use crate::{
    Jam,
    extensions::{ReadExt, WriteExt},
};
use std::io::{self, Read, Write};

impl Jam for i8 {
    fn read_from<R>(reader: &mut R) -> io::Result<Self>
    where
        R: Read,
    {
        let byte = reader.read_u8()?;
        Ok(byte as i8)
    }

    fn write_into<W>(&self, writer: &mut W) -> io::Result<()>
    where
        W: Write,
    {
        writer.write_u8(*self as u8)
    }
}

impl Jam for u8 {
    fn read_from<R>(reader: &mut R) -> io::Result<Self>
    where
        R: Read,
    {
        reader.read_u8()
    }

    fn write_into<W>(&self, writer: &mut W) -> io::Result<()>
    where
        W: Write,
    {
        writer.write_u8(*self)
    }
}

macro_rules! signed_impl {
    ($type: ty) => {
        impl Jam for $type {
            fn read_from<R>(reader: &mut R) -> io::Result<Self>
            where
                R: Read,
            {
                reader
                    .read_signed_varint()?
                    .try_into()
                    .map_err(|_| io::ErrorKind::InvalidData.into())
            }

            fn write_into<W>(&self, writer: &mut W) -> io::Result<()>
            where
                W: Write,
            {
                writer.write_signed_varint(*self as i64)
            }
        }
    };
}

macro_rules! unsigned_impl {
    ($type: ty) => {
        impl Jam for $type {
            fn read_from<R>(reader: &mut R) -> io::Result<Self>
            where
                R: Read,
            {
                reader
                    .read_unsigned_varint()?
                    .try_into()
                    .map_err(|_| io::ErrorKind::InvalidData.into())
            }

            fn write_into<W>(&self, writer: &mut W) -> io::Result<()>
            where
                W: Write,
            {
                writer.write_unsigned_varint(*self as u64)
            }
        }
    };
}

signed_impl!(i16);
unsigned_impl!(u16);

signed_impl!(i32);
unsigned_impl!(u32);

signed_impl!(i64);
unsigned_impl!(u64);

signed_impl!(isize);
unsigned_impl!(usize);
