use crate::{
    externally_length_prefixed::FieldLength, util, BitRead, BitWrite, Error,
    ExternallyLengthPrefixed, FlexibleArrayMember, Protocol, Settings,
};
use core::any::Any;

impl<T: Protocol> Protocol for Vec<T> {
    fn read(read: &mut dyn BitRead, settings: &Settings, ctx: &mut dyn Any) -> Result<Self, Error> {
        util::read_list(read, settings, ctx)
    }

    fn write(
        &self,
        write: &mut dyn BitWrite,
        settings: &Settings,
        ctx: &mut dyn Any,
    ) -> Result<(), Error> {
        util::write_list_length_prefixed(self.iter(), write, settings, ctx)
    }
}

impl<T: Protocol> ExternallyLengthPrefixed for Vec<T> {
    fn read(
        read: &mut dyn BitRead,
        settings: &Settings,
        ctx: &mut dyn Any,
        length: &FieldLength,
    ) -> Result<Self, Error> {
        util::read_list_with_hints(read, settings, ctx, length)
    }

    fn write(
        &self,
        write: &mut dyn BitWrite,
        settings: &Settings,
        ctx: &mut dyn Any,
    ) -> Result<(), Error> {
        util::write_list(self.iter(), write, settings, ctx)
    }
}

impl<T: Protocol> FlexibleArrayMember for Vec<T> {
    fn read(read: &mut dyn BitRead, settings: &Settings, ctx: &mut dyn Any) -> Result<Self, Error> {
        util::read_list_to_eof(read, settings, ctx)
    }

    fn write(
        &self,
        write: &mut dyn BitWrite,
        settings: &Settings,
        ctx: &mut dyn Any,
    ) -> Result<(), Error> {
        util::write_list(self.iter(), write, settings, ctx)
    }
}
