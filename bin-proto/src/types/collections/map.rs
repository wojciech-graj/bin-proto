use crate::{BitRead, BitWrite, Error, Protocol, Settings};

use std::collections::{BTreeMap, HashMap};
use std::hash::Hash;

pub type SizeType = u32;

macro_rules! impl_map_type {
    ( $ty:ident => K: $( $k_pred:ident ),+ ) => {
        impl<K, V> Protocol for $ty<K, V>
            where K: Protocol + $( $k_pred +)+,
                  V: Protocol
        {
            fn read_field(read: &mut dyn BitRead,
                          settings: &Settings,
                          ) -> Result<Self, Error> {
                let mut map = $ty::new();

                let length = SizeType::read(read, settings)?;

                for _ in 0..length {
                    let key = K::read(read, settings)?;
                    let value = V::read(read, settings)?;

                    map.insert(key, value);
                }

                Ok(map)
            }

            fn write_field(&self, write: &mut dyn BitWrite,
                           settings: &Settings,
                           ) -> Result<(), Error> {
                (self.len() as SizeType).write(write, settings)?;

                for (key, value) in self.iter() {
                    key.write(write, settings)?;
                    value.write(write, settings)?;
                }

                Ok(())
            }
        }
    }
}

impl_map_type!(HashMap => K: Hash, Eq);
impl_map_type!(BTreeMap => K: Ord);
