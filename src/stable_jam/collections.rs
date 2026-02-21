use crate::StableJam;

impl<T, const N: usize> StableJam for [T; N] where T: StableJam {}

impl<T> StableJam for Vec<T> where T: StableJam {}
