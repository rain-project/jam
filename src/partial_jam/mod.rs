use std::io::{self, Read, Write};

pub trait PartialJam {
    type Partial;

    fn complete(partial: Self::Partial) -> Self;

    fn decode_from<R>(reader: &mut R) -> io::Result<Self::Partial>
    where
        R: Read;

    fn encode_unstable_into<W>(&self, writer: &mut W) -> io::Result<()>
    where
        W: Write;
}

mod collections;
