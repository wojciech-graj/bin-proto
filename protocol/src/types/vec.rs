use crate::{hint, util, BitRead, BitWrite, Error, FlexibleArrayMember, Parcel, Settings};

impl<T: Parcel> Parcel for Vec<T> {
    fn read_field(
        read: &mut dyn BitRead,
        settings: &Settings,
        hints: &mut hint::Hints,
    ) -> Result<Self, Error> {
        util::read_list(read, settings, hints)
    }

    fn write_field(
        &self,
        write: &mut dyn BitWrite,
        settings: &Settings,
        hints: &mut hint::Hints,
    ) -> Result<(), Error> {
        util::write_list(self.iter(), write, settings, hints)
    }
}

impl<T: Parcel> FlexibleArrayMember for Vec<T> {}
