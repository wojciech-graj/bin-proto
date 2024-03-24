use crate::{hint, BitRead, BitWrite, Error, Parcel, Settings};

pub trait FlexibleArrayMember: Parcel {
    fn read_field(
        read: &mut dyn BitRead,
        settings: &Settings,
        hints: &mut hint::Hints,
    ) -> Result<Self, Error> {
        hints
            .known_field_lengths
            .insert(hints.current_field_index, hint::FieldLength::Flexible);
        Parcel::read_field(read, settings, hints)
    }

    fn write_field(
        &self,
        write: &mut dyn BitWrite,
        settings: &Settings,
        hints: &mut hint::Hints,
    ) -> Result<(), Error> {
        hints
            .known_field_lengths
            .insert(hints.current_field_index, hint::FieldLength::Flexible);
        Parcel::write_field(self, write, settings, hints)
    }
}
