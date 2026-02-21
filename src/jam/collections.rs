use crate::Jam;
use std::{
    io::{self, Read, Write},
    mem::MaybeUninit,
};

impl<T, const N: usize> Jam for [T; N]
where
    T: Jam,
{
    fn decode_from<R>(reader: &mut R) -> io::Result<[T; N]>
    where
        R: Read,
    {
        let mut values: [MaybeUninit<T>; N] = [const { MaybeUninit::uninit() }; N];

        for index in 0..N {
            match T::decode_from(reader) {
                Ok(value) => {
                    values[index].write(value);
                }

                Err(error) => {
                    unsafe {
                        for value in &mut values[..index] {
                            value.assume_init_drop();
                        }
                    }

                    return Err(error);
                }
            }
        }

        let values = unsafe { (&values as *const _ as *const [T; N]).read() };

        Ok(values)
    }

    fn encode_unstable_into<W>(&self, writer: &mut W) -> io::Result<()>
    where
        W: Write,
    {
        for value in self {
            value.encode_unstable_into(writer)?;
        }

        Ok(())
    }
}

impl<T> Jam for Vec<T>
where
    T: Jam,
{
    fn decode_from<R>(reader: &mut R) -> io::Result<Vec<T>>
    where
        R: Read,
    {
        let length = usize::decode_from(reader)?;
        let mut values = Vec::with_capacity(length);

        for _ in 0..length {
            let value = T::decode_from(reader)?;
            values.push(value);
        }

        Ok(values)
    }

    fn encode_unstable_into<W>(&self, writer: &mut W) -> io::Result<()>
    where
        W: Write,
    {
        self.len().encode_unstable_into(writer)?;

        for value in self {
            value.encode_unstable_into(writer)?;
        }

        Ok(())
    }
}
