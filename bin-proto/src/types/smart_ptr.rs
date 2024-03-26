use crate::{BitRead, BitWrite, Error, Protocol, Settings};
use core::any::Any;

use std::ops::Deref;
use std::rc::Rc;
use std::sync::Arc;

macro_rules! impl_smart_ptr_type {
    ($ty:ident) => {
        impl<T: Protocol> Protocol for $ty<T> {
            fn read(
                read: &mut dyn BitRead,
                settings: &Settings,
                ctx: &mut dyn Any,
            ) -> Result<Self, Error> {
                let value = T::read(read, settings, ctx)?;
                Ok($ty::new(value))
            }

            fn write(
                &self,
                write: &mut dyn BitWrite,
                settings: &Settings,
                ctx: &mut dyn Any,
            ) -> Result<(), Error> {
                self.deref().write(write, settings, ctx)
            }
        }
    };
}

impl_smart_ptr_type!(Rc);
impl_smart_ptr_type!(Arc);
