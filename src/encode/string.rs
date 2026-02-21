use crate::{Encode, StableEncode};
use std::io::{self, Write};

impl Encode for String {
    fn encode_unstable_into<W>(&self, writer: &mut W) -> io::Result<()>
    where
        W: Write,
    {
        let bytes = self.as_bytes();
        bytes.len().encode_unstable_into(writer)?;
        writer.write_all(bytes)?;

        Ok(())
    }
}

impl StableEncode for String {}
