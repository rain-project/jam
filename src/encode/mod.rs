use std::io::{self, Write};

pub trait Encode: Sized {
    fn encode_unstable_into<W>(&self, writer: &mut W) -> io::Result<()>
    where
        W: Write;
}

pub trait StableEncode: Encode {
    fn encode_into<W>(&self, writer: &mut W) -> io::Result<()>
    where
        W: Write,
    {
        self.encode_stable_into(writer)
    }

    fn encode_stable_into<W>(&self, writer: &mut W) -> io::Result<()>
    where
        W: Write,
    {
        self.encode_unstable_into(writer)
    }
}

mod arrays;
mod integers;
mod string;
