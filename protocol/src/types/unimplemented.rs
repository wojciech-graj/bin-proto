use crate::{hint, BitRead, BitWrite, Error, Parcel, Settings};

/// A type that does not have any protocol serialization implemented.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Unimplemented;

impl Parcel for Unimplemented {
    fn read_field(_: &mut dyn BitRead, _: &Settings, _: &mut hint::Hints) -> Result<Self, Error> {
        unimplemented!();
    }

    fn write_field(
        &self,
        _: &mut dyn BitWrite,
        _: &Settings,
        _: &mut hint::Hints,
    ) -> Result<(), Error> {
        unimplemented!();
    }
}
