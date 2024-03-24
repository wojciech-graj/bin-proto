use crate::{hint, BitRead, BitWrite, Error, Parcel, Settings};
use std::marker::PhantomData;

impl<T> Parcel for PhantomData<T> {
    fn read_field(_: &mut dyn BitRead, _: &Settings, _: &mut hint::Hints) -> Result<Self, Error> {
        Ok(PhantomData)
    }

    fn write_field(
        &self,
        _: &mut dyn BitWrite,
        _: &Settings,
        _: &mut hint::Hints,
    ) -> Result<(), Error> {
        Ok(())
    }
}
