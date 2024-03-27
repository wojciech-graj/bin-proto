use crate::{util, BitRead, BitWrite, Error, FlexibleArrayMember, Protocol, Settings};
use core::any::Any;

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
