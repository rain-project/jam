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
