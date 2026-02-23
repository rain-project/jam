use crate::PartialDecode;
use std::{
    collections::HashMap,
    hash::Hash,
    io::{self, Read},
};

impl<K, V> PartialDecode for HashMap<K, V>
where
    K: Eq + Hash + PartialDecode,
    K::Partial: Eq + Hash,
    V: Eq + PartialDecode,
    V::Partial: Eq,
{
    type Partial = HashMap<K::Partial, V::Partial>;

    fn decode_from<R>(reader: &mut R) -> io::Result<HashMap<K::Partial, V::Partial>>
    where
        R: Read,
    {
        let length = usize::decode_from(reader)?;
        let mut items = HashMap::with_capacity(length);

        for _ in 0..length {
            let key = K::decode_from(reader)?;
            let value = V::decode_from(reader)?;

            items.insert(key, value);
        }

        Ok(items)
    }

    fn complete(partial: HashMap<K::Partial, V::Partial>) -> HashMap<K, V> {
        let mut items = HashMap::with_capacity(partial.len());
        items.extend(partial.into_iter().map(<(K, V)>::complete));
        items
    }
}
