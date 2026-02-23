use crate::PartialDecode;
use std::{
    collections::BTreeSet,
    io::{self, Read},
};

impl<T> PartialDecode for BTreeSet<T>
where
    T: Ord + PartialDecode,
    T::Partial: Ord,
{
    type Partial = BTreeSet<T::Partial>;

    fn decode_from<R>(reader: &mut R) -> io::Result<BTreeSet<T::Partial>>
    where
        R: Read,
    {
        let length = usize::decode_from(reader)?;
        let mut items = BTreeSet::new();

        for _ in 0..length {
            let item = T::decode_from(reader)?;
            items.insert(item);
        }

        Ok(items)
    }

    fn complete(partial: BTreeSet<T::Partial>) -> BTreeSet<T> {
        let mut items = BTreeSet::new();
        items.extend(partial.into_iter().map(T::complete));
        items
    }
}
