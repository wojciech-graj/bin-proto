use crate::{
    externally_length_prefixed, util, BitRead, BitWrite, Error, ExternallyLengthPrefixed,
    FlexibleArrayMember, Protocol, Settings,
};

impl<T: Protocol> Protocol for Vec<T> {
    fn read(read: &mut dyn BitRead, settings: &Settings) -> Result<Self, Error> {
        util::read_list(read, settings)
    }

    fn write(&self, write: &mut dyn BitWrite, settings: &Settings) -> Result<(), Error> {
        util::write_list_length_prefixed(self.iter(), write, settings)
    }
}

impl<T: Protocol> ExternallyLengthPrefixed for Vec<T> {
    fn read(
        read: &mut dyn BitRead,
        settings: &Settings,
        hints: &mut externally_length_prefixed::Hints,
    ) -> Result<Self, Error> {
        util::read_list_with_hints(read, settings, hints)
    }

    fn write(
        &self,
        write: &mut dyn BitWrite,
        settings: &Settings,
        _: &mut externally_length_prefixed::Hints,
    ) -> Result<(), Error> {
        util::write_list(self.iter(), write, settings)
    }
}

impl<T: Protocol> FlexibleArrayMember for Vec<T> {
    fn read(read: &mut dyn BitRead, settings: &Settings) -> Result<Self, Error> {
        util::read_list_to_eof(read, settings)
    }

    fn write(&self, write: &mut dyn BitWrite, settings: &Settings) -> Result<(), Error> {
        util::write_list(self.iter(), write, settings)
    }
}
