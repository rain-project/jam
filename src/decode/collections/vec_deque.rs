use crate::PartialDecode;
use std::{
    collections::VecDeque,
    io::{self, Read},
};

impl<T> PartialDecode for VecDeque<T>
where
    T: PartialDecode,
{
    type Partial = VecDeque<T::Partial>;

    fn decode_from<R>(reader: &mut R) -> io::Result<VecDeque<T::Partial>>
    where
        R: Read,
    {
        let length = usize::decode_from(reader)?;
        let mut items = VecDeque::with_capacity(length);

        for _ in 0..length {
            let item = T::decode_from(reader)?;
            items.push_back(item);
        }

        Ok(items)
    }

    fn complete(partial: VecDeque<T::Partial>) -> VecDeque<T> {
        let mut items = VecDeque::with_capacity(partial.len());
        items.extend(partial.into_iter().map(T::complete));
        items
    }
}
