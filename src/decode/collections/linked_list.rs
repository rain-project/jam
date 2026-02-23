use crate::PartialDecode;
use std::{
    collections::LinkedList,
    io::{self, Read},
};

impl<T> PartialDecode for LinkedList<T>
where
    T: PartialDecode,
{
    type Partial = LinkedList<T::Partial>;

    fn decode_from<R>(reader: &mut R) -> io::Result<LinkedList<T::Partial>>
    where
        R: Read,
    {
        let length = usize::decode_from(reader)?;
        let mut items = LinkedList::new();

        for _ in 0..length {
            let item = T::decode_from(reader)?;
            items.push_back(item);
        }

        Ok(items)
    }

    fn complete(partial: LinkedList<T::Partial>) -> LinkedList<T> {
        partial
            .into_iter()
            .map(T::complete)
            .collect::<LinkedList<_>>()
    }
}
