use std::io::{self, Read, Write};

pub trait Jam: Sized {
    fn read_from<R>(reader: &mut R) -> io::Result<Self>
    where
        R: Read;

    fn write_into<W>(&self, writer: &mut W) -> io::Result<()>
    where
        W: Write;
}

mod collections;
mod enums;
mod integers;
