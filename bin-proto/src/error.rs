use alloc::{boxed::Box, string::String};
#[cfg(feature = "std")]
use std::io;

#[cfg(not(feature = "std"))]
use core2::io;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    FromUtf8(#[from] alloc::string::FromUtf8Error),
    #[error(transparent)]
    Nul(#[from] alloc::ffi::NulError),
    #[error(transparent)]
    TryFromInt(#[from] core::num::TryFromIntError),
    #[error("unknown enum discriminant: '{0}'")]
    UnknownEnumDiscriminant(String),
    #[error("failed to convert tag")]
    TagConvert,
    #[error("failed to convert Vec to slice")]
    SliceTryFromVec,
    #[error(transparent)]
    Other(Box<dyn core::error::Error + Send + Sync>),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused)]
    trait IsSizedSendSync: Sized + Send + Sync {}

    impl IsSizedSendSync for Error {}
}
