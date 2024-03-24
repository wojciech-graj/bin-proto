use crate::{hint, BitRead, Error, Parcel, Settings};
use std::marker::PhantomData;

use std::io::prelude::*;

impl<T> Parcel for PhantomData<T> {
    const TYPE_NAME: &'static str = "PhantomData<T>";

    fn read_field(_: &mut dyn BitRead, _: &Settings, _: &mut hint::Hints) -> Result<Self, Error> {
        Ok(PhantomData)
    }

    fn write_field(
        &self,
        _: &mut dyn Write,
        _: &Settings,
        _: &mut hint::Hints,
    ) -> Result<(), Error> {
        Ok(())
    }
}
