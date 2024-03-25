use crate::{hint, BitRead, BitWrite, Error, Parcel, Settings};

pub trait WithLengthPrefix: Parcel {
    fn read_field(
        read: &mut dyn BitRead,
        settings: &Settings,
        hints: &mut hint::Hints,
    ) -> Result<Self, Error>;

    fn write_field(
        &self,
        write: &mut dyn BitWrite,
        settings: &Settings,
        hints: &mut hint::Hints,
    ) -> Result<(), Error>;
}
