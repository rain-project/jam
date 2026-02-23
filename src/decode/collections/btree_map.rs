use crate::PartialDecode;
use std::{
    collections::BTreeMap,
    io::{self, Read},
};

impl<K, V> PartialDecode for BTreeMap<K, V>
where
    K: Ord + PartialDecode,
    K::Partial: Ord,
    V: PartialDecode,
{
    type Partial = BTreeMap<K::Partial, V::Partial>;

    fn decode_from<R>(reader: &mut R) -> io::Result<BTreeMap<K::Partial, V::Partial>>
    where
        R: Read,
    {
        let length = usize::decode_from(reader)?;
        let mut items = BTreeMap::new();

        for _ in 0..length {
            let key = K::decode_from(reader)?;
            let value = V::decode_from(reader)?;

            items.insert(key, value);
        }

        Ok(items)
    }

    fn complete(partial: BTreeMap<K::Partial, V::Partial>) -> BTreeMap<K, V> {
        let mut items = BTreeMap::new();
        items.extend(partial.into_iter().map(<(K, V)>::complete));
        items
    }
}
