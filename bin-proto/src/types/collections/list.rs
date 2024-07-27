macro_rules! impl_list_type {
    ($ty:ident => T: $( $ty_pred:ident ),*) => {
        impl<Tag, Ctx, T> $crate::TaggedRead<Tag, Ctx> for $ty<T>
            where
                T: $crate::ProtocolRead<Ctx> $( + $ty_pred )*,
                Tag: TryInto<usize>,
        {
            fn read(read: &mut dyn $crate::BitRead,
                    byte_order: $crate::ByteOrder,
                    ctx: &mut Ctx,
                    tag: Tag,
                    ) -> $crate::Result<Self> {
                let elements = $crate::util::read_items(tag.try_into().map_err(|_| $crate::Error::TagConvert)?, read, byte_order, ctx)?;
                Ok(elements.into_iter().collect())
            }
        }

        impl<Ctx, T> $crate::UntaggedWrite<Ctx> for $ty<T>
            where T: $crate::ProtocolWrite<Ctx> $( + $ty_pred )*
        {
            fn write(&self,
                     write: &mut dyn $crate::BitWrite,
                     byte_order: $crate::ByteOrder,
                     ctx: &mut Ctx,
                     ) -> $crate::Result<()> {
                $crate::util::write_items(self.iter(), write, byte_order, ctx)
            }
        }

        impl<Ctx, T> $crate::FlexibleArrayMemberRead<Ctx> for $ty<T>
            where T: $crate::ProtocolRead<Ctx> $( + $ty_pred )*
        {
            fn read(read: &mut dyn $crate::BitRead, byte_order: $crate::ByteOrder, ctx: &mut Ctx) -> $crate::Result<Self> {
                Ok($crate::util::read_items_to_eof(read, byte_order, ctx)?.into_iter().collect())
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

            test_externally_tagged!($t<u16> => [[0x00, 0x01, 0x00, 0x02, 0x00, 0x03], $t::from([1, 2, 3])]);
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

    impl_list_type!(VecDeque => T: );
    test_list_type!(VecDeque);
}

mod b_tree_set {
    use std::collections::BTreeSet;

    impl_list_type!(BTreeSet => T: Ord);
    test_list_type!(BTreeSet);
}

mod hash_set {
    use std::collections::HashSet;
    use std::hash::Hash;

    impl_list_type!(HashSet => T: Hash, Eq);
}

mod binary_heap {
    use std::collections::BinaryHeap;

    impl_list_type!(BinaryHeap => T: Ord);
}
