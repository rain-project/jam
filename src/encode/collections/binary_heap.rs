use crate::Encode;
use std::{
    collections::BinaryHeap,
    io::{self, Write},
};

impl<T> Encode for BinaryHeap<T>
where
    T: Encode,
{
    fn encode_unstable_into<W>(&self, writer: &mut W) -> io::Result<()>
    where
        W: Write,
    {
        self.len().encode_unstable_into(writer)?;

        for item in self {
            item.encode_unstable_into(writer)?;
        }

        Ok(())
    }
}
