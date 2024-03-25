use crate::{BitRead, BitWrite, Error, Parcel, Settings};

use std::ops::Deref;
use std::rc::Rc;
use std::sync::Arc;

macro_rules! impl_smart_ptr_type {
    ($ty:ident) => {
        impl<T: Parcel> Parcel for $ty<T> {
            fn read_field(read: &mut dyn BitRead, settings: &Settings) -> Result<Self, Error> {
                let value = T::read(read, settings)?;
                Ok($ty::new(value))
            }

            fn write_field(
                &self,
                write: &mut dyn BitWrite,
                settings: &Settings,
            ) -> Result<(), Error> {
                self.deref().write(write, settings)
            }
        }
    };
}

impl_smart_ptr_type!(Rc);
impl_smart_ptr_type!(Arc);
