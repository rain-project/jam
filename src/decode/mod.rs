use std::io::{self, Read};

pub trait Decode: PartialDecode {
    fn decode(mut bytes: &[u8]) -> io::Result<Self> {
        <Self as Decode>::decode_from(&mut bytes)
    }

    fn decode_from<R>(reader: &mut R) -> io::Result<Self>
    where
        R: Read;
}

pub trait PartialDecode: Sized {
    type Partial;

    fn decode(mut bytes: &[u8]) -> io::Result<Self::Partial> {
        Self::decode_from(&mut bytes)
    }

    fn decode_from<R>(reader: &mut R) -> io::Result<Self::Partial>
    where
        R: Read;

    fn complete(partial: Self::Partial) -> Self;
}

impl<T> Decode for T
where
    T: PartialDecode<Partial = T>,
{
    fn decode_from<R>(reader: &mut R) -> io::Result<Self>
    where
        R: Read,
    {
        <Self as PartialDecode>::decode_from(reader)
    }
}

mod arrays;
mod integers;
mod string;
