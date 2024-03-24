use crate::{hint, BitRead, BitWrite, Error, Parcel, Settings};

/// A type that does not have any protocol serialization implemented.
///
/// # Behaviour
///
/// If any unimplemented parcel is read, an error of type
/// `UnimplementedParcel` is returned. This allows clients to
/// handle unimplemented data gracefully.
///
/// If you attempt to write an unimplemented parcel, the
/// program panics. It makes sense to do error handling on
/// unimplemented types that are read from remote machines,
/// but it does not make sense to allow undefined data to be sent.
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
