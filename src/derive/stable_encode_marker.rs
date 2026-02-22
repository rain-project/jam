use crate::StableEncode;

pub trait StableEncodeMarker<'dummy>: StableEncode {}

impl<'dummy, T> StableEncodeMarker<'dummy> for T where T: StableEncode {}
