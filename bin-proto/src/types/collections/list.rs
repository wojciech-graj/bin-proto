macro_rules! impl_list_type {
    ( $ty:ident => T: $( $ty_pred:ident ),* ) => {
        impl<T> crate::ExternallyLengthPrefixed for $ty<T>
            where T: $crate::Protocol $( + $ty_pred )*
        {
            fn read(read: &mut dyn crate::BitRead,
                    settings: &crate::Settings,
                    ctx: &mut dyn core::any::Any,
                    length: usize,
                    ) -> Result<Self, $crate::Error> {
                let elements = crate::util::read_items(length, read, settings, ctx)?;
                Ok(elements.into_iter().collect())
            }

            fn write(&self,
                     write: &mut dyn crate::BitWrite,
                     settings: &crate::Settings,
                     ctx: &mut dyn core::any::Any,
                     ) -> Result<(), $crate::Error> {
                crate::util::write_items(self.iter(), write, settings, ctx)
            }
        }
    }
}

macro_rules! test_list_type {
    ( $t:ident ) => {
        #[cfg(test)]
        #[allow(unused_imports)]
        mod tests {
            use super::*;

            test_externally_length_prefixed!($t<u16> => [[0x00, 0x01, 0x00, 0x02, 0x00, 0x03], $t::from([1, 2, 3])]);
        }
    }
}

mod vec {
    impl_list_type!(Vec => T: );
    test_list_type!(Vec);
}

mod linked_list {
    use std::collections::LinkedList;

    impl_list_type!(LinkedList => T: );
    test_list_type!(LinkedList);
}

mod vec_deque {
    use std::collections::VecDeque;

    impl_list_type!(VecDeque   => T: );
    test_list_type!(VecDeque);
}

mod b_tree_set {
    use std::collections::BTreeSet;

    impl_list_type!(BTreeSet   => T: Ord);
    test_list_type!(BTreeSet);
}

mod hash_set {
    use std::collections::HashSet;
    use std::hash::Hash;

    impl_list_type!(HashSet => T: Hash, Eq);
}
