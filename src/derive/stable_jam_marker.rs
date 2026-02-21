use crate::StableJam;

pub trait StableJamMarker<'dummy>: StableJam {}

impl<'dummy, T> StableJamMarker<'dummy> for T where T: StableJam {}
