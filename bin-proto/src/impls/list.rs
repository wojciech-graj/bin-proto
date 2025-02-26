macro_rules! impl_read_list {
    (
        $ty:ident<T $(: $tbound0:ident $(+ $tbound1:ident)?)?
        $(, $h:ident: $hbound0:ident + $hbound1:ident)?>
    ) => {
        impl<Tag, Ctx, T, $($h)?> $crate::TaggedRead<Tag, Ctx> for $ty<T, $($h)?>
        where
            T: $crate::ProtocolRead<Ctx> $(+ $tbound0 $(+ $tbound1)?)?,
            Tag: ::core::convert::TryInto<usize>,
            $($h: $hbound0 + $hbound1)?
        {
            fn read(read: &mut dyn $crate::BitRead,
                byte_order: $crate::ByteOrder,
                ctx: &mut Ctx,
                tag: Tag,
            ) -> $crate::Result<Self> {
                let elements = $crate::util::read_items(
                    ::core::convert::TryInto::try_into(tag)
                        .map_err(|_| $crate::Error::TagConvert)?,
                    read,
                    byte_order,
                    ctx
                )?;
                Ok(::core::iter::IntoIterator::into_iter(elements).collect())
            }
        }


        impl<Ctx, T, $($h)?> $crate::FlexibleArrayMemberRead<Ctx> for $ty<T, $($h)?>
        where
            T: $crate::ProtocolRead<Ctx> $(+ $tbound0 $(+ $tbound1)?)?,
            $($h: $hbound0 + $hbound1)?
        {
            fn read(
                read: &mut dyn $crate::BitRead,
                byte_order: $crate::ByteOrder,
                ctx: &mut Ctx
            ) -> $crate::Result<Self> {
                Ok(::core::iter::IntoIterator::into_iter(
                    $crate::util::read_items_to_eof(read, byte_order, ctx)?
                ).collect())
            }
        }
    }
}

macro_rules! impl_write_list {
    ( $ty:ident<T $(: $tbound0:ident $(+ $tbound1:ident)?)? $(, $h:ident)?> ) => {
        impl<Ctx, T, $($h)?> $crate::UntaggedWrite<Ctx> for $ty<T, $($h)?>
        where
            T: $crate::ProtocolWrite<Ctx> $(+ $tbound0 $(+ $tbound1)?)?
        {
            fn write(&self,
                write: &mut dyn $crate::BitWrite,
                byte_order: $crate::ByteOrder,
                ctx: &mut Ctx,
            ) -> $crate::Result<()> {
                $crate::util::write_items(self.iter(), write, byte_order, ctx)
            }
        }
    }
}

mod vec {
    use alloc::vec::Vec;

    impl_read_list!(Vec<T>);
    impl_write_list!(Vec<T>);
    test_flexible_array_member_read_and_tagged!(Vec<u8>| 3: alloc::vec![1, 2, 3] => [0x01, 0x02, 0x03]);
}

mod linked_list {
    use alloc::collections::linked_list::LinkedList;

    impl_read_list!(LinkedList<T>);
    impl_write_list!(LinkedList<T>);
    test_flexible_array_member_read_and_tagged!(LinkedList<u8>| 3: [1, 2, 3].into() => [0x01, 0x02, 0x03]);
}

mod vec_deque {
    use alloc::collections::vec_deque::VecDeque;

    impl_read_list!(VecDeque<T>);
    impl_write_list!(VecDeque<T>);
    test_flexible_array_member_read_and_tagged!(VecDeque<u8>| 3: [1, 2, 3].into() => [0x01, 0x02, 0x03]);
}

mod b_tree_set {
    use alloc::collections::btree_set::BTreeSet;

    impl_read_list!(BTreeSet<T: Ord>);
    impl_write_list!(BTreeSet<T: Ord>);
    test_flexible_array_member_read_and_tagged!(BTreeSet<u8>| 3: [1, 2, 3].into() => [0x01, 0x02, 0x03]);
}

mod binary_heap {
    use alloc::collections::binary_heap::BinaryHeap;

    impl_read_list!(BinaryHeap<T: Ord>);
    impl_write_list!(BinaryHeap<T: Ord>);
}

#[cfg(feature = "std")]
mod hash_set {
    use core::hash::{BuildHasher, Hash};
    use std::collections::HashSet;

    impl_read_list!(HashSet<T: Hash + Eq, H: BuildHasher + Default>);
    impl_write_list!(HashSet<T: Hash + Eq, H>);
    test_flexible_array_member_read_and_tagged!(HashSet<u8>| 1: [1].into() => [0x01]);
}
