use crate::{hint, BitRead, BitWrite, Error, Parcel, Settings};

pub trait BitField: Parcel {
    fn read_field(
        read: &mut dyn BitRead,
        bits: u32,
        settings: &Settings,
        hints: &mut hint::Hints,
    ) -> Result<Self, Error>;

    fn write_field(
        &self,
        write: &mut dyn BitWrite,
        bits: u32,
        settings: &Settings,
        hints: &mut hint::Hints,
    ) -> Result<(), Error>;
}
