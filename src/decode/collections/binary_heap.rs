use crate::PartialDecode;
use std::{
    collections::BinaryHeap,
    io::{self, Read},
};

impl<T> PartialDecode for BinaryHeap<T>
where
    T: Ord + PartialDecode,
    T::Partial: Ord,
{
    type Partial = BinaryHeap<T::Partial>;

    fn decode_from<R>(reader: &mut R) -> io::Result<BinaryHeap<T::Partial>>
    where
        R: Read,
    {
        let length = usize::decode_from(reader)?;
        let mut items = BinaryHeap::with_capacity(length);

        for _ in 0..length {
            let item = T::decode_from(reader)?;
            items.push(item);
        }

        Ok(items)
    }

    fn complete(partial: BinaryHeap<T::Partial>) -> BinaryHeap<T> {
        let mut items = BinaryHeap::with_capacity(partial.len());
        items.extend(partial.into_iter().map(T::complete));
        items
    }
}
