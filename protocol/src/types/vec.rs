use crate::{
    externally_length_prefixed, util, BitRead, BitWrite, Error, ExternallyLengthPrefixed,
    FlexibleArrayMember, Parcel, Settings,
};

impl<T: Parcel> Parcel for Vec<T> {
    fn read_field(read: &mut dyn BitRead, settings: &Settings) -> Result<Self, Error> {
        util::read_list(read, settings)
    }

    fn write_field(&self, write: &mut dyn BitWrite, settings: &Settings) -> Result<(), Error> {
        util::write_list_length_prefixed(self.iter(), write, settings)
    }
}

impl<T: Parcel> ExternallyLengthPrefixed for Vec<T> {
    fn read_field(
        read: &mut dyn BitRead,
        settings: &Settings,
        hints: &mut externally_length_prefixed::Hints,
    ) -> Result<Self, Error> {
        util::read_list_with_hints(read, settings, hints)
    }

    fn write_field(
        &self,
        write: &mut dyn BitWrite,
        settings: &Settings,
        _: &mut externally_length_prefixed::Hints,
    ) -> Result<(), Error> {
        util::write_list(self.iter(), write, settings)
    }
}

impl<T: Parcel> FlexibleArrayMember for Vec<T> {
    fn read_field(read: &mut dyn BitRead, settings: &Settings) -> Result<Self, Error> {
        util::read_list_to_eof(read, settings)
    }

    fn write_field(&self, write: &mut dyn BitWrite, settings: &Settings) -> Result<(), Error> {
        util::write_list(self.iter(), write, settings)
    }
}
