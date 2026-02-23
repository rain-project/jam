use crate::PartialDecode;
use std::{
    collections::HashSet,
    hash::Hash,
    io::{self, Read},
};

impl<T> PartialDecode for HashSet<T>
where
    T: Eq + Hash + PartialDecode,
    T::Partial: Eq + Hash,
{
    type Partial = HashSet<T::Partial>;

    fn decode_from<R>(reader: &mut R) -> io::Result<HashSet<T::Partial>>
    where
        R: Read,
    {
        let length = usize::decode_from(reader)?;
        let mut items = HashSet::with_capacity(length);

        for _ in 0..length {
            let item = T::decode_from(reader)?;
            items.insert(item);
        }

        Ok(items)
    }

    fn complete(partial: HashSet<T::Partial>) -> HashSet<T> {
        let mut items = HashSet::with_capacity(partial.len());
        items.extend(partial.into_iter().map(T::complete));
        items
    }
}
