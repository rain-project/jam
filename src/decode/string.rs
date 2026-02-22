use crate::PartialDecode;
use std::io::{self, Read};

impl PartialDecode for String {
    type Partial = String;

    fn decode_from<R>(reader: &mut R) -> io::Result<String>
    where
        R: Read,
    {
        let length = usize::decode_from(reader)?;

        let mut bytes = vec![0u8; length];
        reader.read_exact(&mut bytes)?;

        String::from_utf8(bytes).map_err(|_| io::ErrorKind::InvalidData.into())
    }

    fn complete(partial: String) -> String {
        partial
    }
}
