use crate::{BitRead, BitWrite, Error, Protocol, Settings};

use core::any::Any;
use std::collections::{BTreeMap, HashMap};
use std::hash::Hash;

pub type SizeType = u32;

macro_rules! impl_map_type {
    ( $ty:ident => K: $( $k_pred:ident ),+ ) => {
        impl<K, V> Protocol for $ty<K, V>
            where K: Protocol + $( $k_pred +)+,
                  V: Protocol
        {
            fn read(read: &mut dyn BitRead,
                          settings: &Settings, ctx: &mut dyn Any,
                          ) -> Result<Self, Error> {
                let mut map = $ty::new();

                let length = SizeType::read(read, settings, ctx)?;

                for _ in 0..length {
                    let key = K::read(read, settings, ctx)?;
                    let value = V::read(read, settings, ctx)?;

                    map.insert(key, value);
                }

                Ok(map)
            }

            fn write(&self, write: &mut dyn BitWrite,
                           settings: &Settings, ctx: &mut dyn Any,
                           ) -> Result<(), Error> {
                (self.len() as SizeType).write(write, settings, ctx)?;

                for (key, value) in self.iter() {
                    key.write(write, settings, ctx)?;
                    value.write(write, settings, ctx)?;
                }

                Ok(())
            }
        }
    }
}

impl_map_type!(HashMap => K: Hash, Eq);
impl_map_type!(BTreeMap => K: Ord);
