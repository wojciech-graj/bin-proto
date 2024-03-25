use crate::{BitRead, BitWrite, Error, Settings};

pub trait FlexibleArrayMember: Sized {
    fn read_field(read: &mut dyn BitRead, settings: &Settings) -> Result<Self, Error>;

    fn write_field(&self, write: &mut dyn BitWrite, settings: &Settings) -> Result<(), Error>;
}
