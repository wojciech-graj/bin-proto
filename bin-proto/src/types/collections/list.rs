use crate::ExternallyLengthPrefixed;
use core::any::Any;
use std::collections::{BTreeSet, HashSet, LinkedList, VecDeque};
use std::hash::Hash;

macro_rules! impl_list_type {
    ( $ty:ident => T: $( $ty_pred:ident ),* ) => {
        impl<T> ExternallyLengthPrefixed for $ty<T>
            where T: $crate::Protocol $( + $ty_pred )*
        {
            fn read(read: &mut dyn crate::BitRead,
                          settings: &crate::Settings,
                          ctx: &mut dyn Any,
                          length: usize,
                          ) -> Result<Self, $crate::Error> {
                let elements = crate::util::read_items(length, read, settings, ctx)?;
                Ok(elements.into_iter().collect())
            }

            fn write(&self, write: &mut dyn crate::BitWrite,
                           settings: &crate::Settings,
                           ctx: &mut dyn core::any::Any,
                           )
                -> Result<(), $crate::Error> {
                crate::util::write_list(self.iter(), write, settings, ctx)
            }
        }
    }
}

impl_list_type!(Vec => T: );
impl_list_type!(LinkedList => T: );
impl_list_type!(VecDeque   => T: );
impl_list_type!(BTreeSet   => T: Ord);
impl_list_type!(HashSet => T: Hash, Eq);
