use alloc::{boxed::Box, string::String};
use core::{convert::Infallible, fmt};
#[cfg(feature = "std")]
use std::io;

#[cfg(not(feature = "std"))]
use core2::io;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    Io(io::Error),
    FromUtf8(alloc::string::FromUtf8Error),
    Nul(alloc::ffi::NulError),
    TryFromInt(core::num::TryFromIntError),
    Borrow(core::cell::BorrowError),
    UnknownEnumDiscriminant(String),
    TagConvert,
    SliceTryFromVec,
    #[cfg_attr(docsrs, doc(cfg(feature = "std")))]
    #[cfg(feature = "std")]
    Poison,
    Other(Box<dyn core::error::Error + Send + Sync>),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(e) => write!(f, "{e}"),
            Self::FromUtf8(e) => write!(f, "{e}"),
            Self::Nul(e) => write!(f, "{e}"),
            Self::TryFromInt(e) => write!(f, "{e}"),
            Self::Borrow(e) => write!(f, "{e}"),
            Self::UnknownEnumDiscriminant(discriminant) => {
                write!(f, "unknown enum discriminant: '{discriminant}'")
            }
            Self::TagConvert => write!(f, "failed to convert tag"),
            Self::SliceTryFromVec => write!(f, "failed to convert Vec to slice"),
            #[cfg_attr(docsrs, doc(cfg(feature = "std")))]
            #[cfg(feature = "std")]
            Self::Poison => write!(f, "poisoned lock: another task failed inside"),
            Self::Other(e) => write!(f, "{e}"),
        }
    }
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<alloc::string::FromUtf8Error> for Error {
    fn from(value: alloc::string::FromUtf8Error) -> Self {
        Self::FromUtf8(value)
    }
}

impl From<alloc::ffi::NulError> for Error {
    fn from(value: alloc::ffi::NulError) -> Self {
        Self::Nul(value)
    }
}

impl From<core::num::TryFromIntError> for Error {
    fn from(value: core::num::TryFromIntError) -> Self {
        Self::TryFromInt(value)
    }
}

impl From<core::cell::BorrowError> for Error {
    fn from(value: core::cell::BorrowError) -> Self {
        Self::Borrow(value)
    }
}

impl From<Infallible> for Error {
    fn from(_: Infallible) -> Self {
        unreachable!()
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
#[cfg(feature = "std")]
impl<T> From<std::sync::PoisonError<T>> for Error {
    fn from(_: std::sync::PoisonError<T>) -> Self {
        Self::Poison
    }
}

impl core::error::Error for Error {}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused)]
    trait IsSizedSendSync: Sized + Send + Sync {}

    impl IsSizedSendSync for Error {}
}
