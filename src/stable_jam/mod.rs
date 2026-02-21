use crate::Jam;
use std::io::{self, Write};

pub trait StableJam: Jam {
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

mod collections;
mod enums;
mod integers;
