use crate::{Encode, StableEncode};
use std::{
    collections::BTreeMap,
    io::{self, Write},
};

impl<K, V> Encode for BTreeMap<K, V>
where
    K: Encode,
    V: Encode,
{
    fn encode_unstable_into<W>(&self, writer: &mut W) -> io::Result<()>
    where
        W: Write,
    {
        self.len().encode_unstable_into(writer)?;

        for (key, value) in self {
            key.encode_unstable_into(writer)?;
            value.encode_unstable_into(writer)?;
        }

        Ok(())
    }
}

impl<K, V> StableEncode for BTreeMap<K, V>
where
    K: StableEncode,
    V: StableEncode,
{
}
