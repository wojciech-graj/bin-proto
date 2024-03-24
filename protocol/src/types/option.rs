use crate::{hint, BitRead, BitWrite, Error, Parcel, Settings};

impl<T: Parcel> Parcel for Option<T> {
    fn read_field(
        read: &mut dyn BitRead,
        settings: &Settings,
        _: &mut hint::Hints,
    ) -> Result<Self, Error> {
        let is_some = bool::read(read, settings)?;

        if is_some {
            let value = T::read(read, settings)?;
            Ok(Some(value))
        } else {
            Ok(None)
        }
    }

    fn write_field(
        &self,
        write: &mut dyn BitWrite,
        settings: &Settings,
        _: &mut hint::Hints,
    ) -> Result<(), Error> {
        self.is_some().write(write, settings)?;

        if let Some(ref value) = *self {
            value.write(write, settings)?;
        }

        Ok(())
    }
}
