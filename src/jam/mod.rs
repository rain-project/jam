use std::io::{self, Read, Write};

pub trait Jam: Sized {
    fn decode_from<R>(reader: &mut R) -> io::Result<Self>
    where
        R: Read;

    fn encode_unstable_into<W>(&self, writer: &mut W) -> io::Result<()>
    where
        W: Write;
}

mod collections;
mod enums;
mod integers;
