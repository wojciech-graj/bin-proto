macro_rules! impl_list_type {
    ( $ty:ident => T: $( $ty_pred:ident ),* ) => {
        impl<T> $crate::Protocol for ::std::collections::$ty<T>
            where T: $crate::Protocol $( + $ty_pred )*
        {
            fn read(read: &mut dyn crate::BitRead,
                          settings: &crate::Settings,
                          ) -> Result<Self, $crate::Error> {
                let elements = crate::util::read_list(read, settings)?;
                Ok(elements.into_iter().collect())
            }

            fn write(&self, write: &mut dyn crate::BitWrite,
                           settings: &crate::Settings,
                           )
                -> Result<(), $crate::Error> {
                crate::util::write_list_length_prefixed(self.iter(), write, settings)
            }
        }

        #[cfg(test)]
        mod test
        {
            pub use crate::{Protocol, Settings};
            pub use std::collections::$ty;

            #[test]
            fn can_be_written_and_read_back_correctly() {
                let original: $ty<u32> = [1, 2, 3, 4, 5].iter().cloned().collect();

                let settings = Settings::default();
                let bytes = original.bytes(&settings).unwrap();
                let read_deque = $ty::<u32>::from_bytes(&bytes, &settings).unwrap();

                assert_eq!(original, read_deque);
            }
        }
    }
}

pub mod linked_list {
    impl_list_type!(LinkedList => T: );
}
pub mod vec_deque {
    impl_list_type!(VecDeque   => T: );
}

pub mod btree_set {
    impl_list_type!(BTreeSet   => T: Ord);
}

pub mod hash_set {
    use std::hash::Hash;
    impl_list_type!(HashSet => T: Hash, Eq);
}
