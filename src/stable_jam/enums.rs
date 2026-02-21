use crate::StableJam;

impl<T> StableJam for Option<T> where T: StableJam {}

impl<T, E> StableJam for Result<T, E>
where
    T: StableJam,
    E: StableJam,
{
}
