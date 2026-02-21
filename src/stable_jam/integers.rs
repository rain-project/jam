use crate::StableJam;

macro_rules! batch_impl {
    ($($type: ty),*) => {
        $(
            impl StableJam for $type {}
        )*
    }
}

batch_impl!(i8, u8, i16, u16, i32, u32, i64, u64, isize, usize);
