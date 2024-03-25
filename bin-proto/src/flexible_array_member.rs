use crate::{BitRead, BitWrite, Error, Settings};

/// A trait for variable-length types without a length prefix.
pub trait FlexibleArrayMember: Sized {
    fn read(read: &mut dyn BitRead, settings: &Settings) -> Result<Self, Error>;

    fn write(&self, write: &mut dyn BitWrite, settings: &Settings) -> Result<(), Error>;
}
