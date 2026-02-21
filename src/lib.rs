pub mod extensions;

mod decode;
mod encode;

pub use self::{
    decode::{Decode, PartialDecode},
    encode::{Encode, StableEncode},
};
