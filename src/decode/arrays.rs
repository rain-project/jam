use crate::PartialDecode;
use std::{
    io::{self, Read},
    mem::MaybeUninit,
};

impl<T, const N: usize> PartialDecode for [T; N]
where
    T: PartialDecode,
{
    type Partial = [T::Partial; N];

    fn decode_from<R>(reader: &mut R) -> io::Result<[T::Partial; N]>
    where
        R: Read,
    {
        let mut items: [MaybeUninit<T::Partial>; N] = [const { MaybeUninit::uninit() }; N];

        for index in 0..N {
            match T::decode_from(reader) {
                Ok(item) => {
                    items[index].write(item);
                }

                Err(error) => {
                    unsafe {
                        // Safety: the first `index` elements of `items` have all been written to

                        for item in &mut items[..index] {
                            item.assume_init_drop();
                        }
                    }

                    return Err(error);
                }
            }
        }

        let items = unsafe {
            // Safety: all `items` have been written to
            (&items as *const _ as *const [T::Partial; N]).read()
        };

        Ok(items)
    }

    fn complete(partial: [T::Partial; N]) -> [T; N] {
        partial.map(T::complete)
    }
}
