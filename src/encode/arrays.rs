use crate::{Encode, StableEncode};
use std::io::{self, Write};

impl<T, const N: usize> Encode for [T; N]
where
    T: Encode,
{
    fn encode_unstable_into<W>(&self, writer: &mut W) -> io::Result<()>
    where
        W: Write,
    {
        for item in self {
            item.encode_unstable_into(writer)?;
        }

        Ok(())
    }
}

impl<T, const N: usize> StableEncode for [T; N] where T: StableEncode {}
