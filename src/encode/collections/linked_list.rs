use crate::{Encode, StableEncode};
use std::{
    collections::LinkedList,
    io::{self, Write},
};

impl<T> Encode for LinkedList<T>
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

impl<T> StableEncode for LinkedList<T> where T: StableEncode {}
