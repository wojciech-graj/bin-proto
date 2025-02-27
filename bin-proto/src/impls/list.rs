macro_rules! impl_read_list {
    (
        $ty:ident<T $(: $tbound0:ident $(+ $tbound1:ident)?)?
        $(, $h:ident: $hbound0:ident + $hbound1:ident)?>
    ) => {
        impl<Tag, Ctx, T, $($h)?> $crate::BitDecode<Ctx, $crate::Tag<Tag>> for $ty<T, $($h)?>
        where
            T: $crate::BitDecode<Ctx> $(+ $tbound0 $(+ $tbound1)?)?,
            Tag: ::core::convert::TryInto<usize>,
            $($h: $hbound0 + $hbound1)?
        {
            fn decode(read: &mut dyn $crate::BitRead,
                byte_order: $crate::ByteOrder,
                ctx: &mut Ctx,
                tag: $crate::Tag<Tag>,
            ) -> $crate::Result<Self> {
                let elements = $crate::util::decode_items(
                    ::core::convert::TryInto::try_into(tag.0)
                        .map_err(|_| $crate::Error::TagConvert)?,
                    read,
                    byte_order,
                    ctx
                )?;
                Ok(::core::iter::IntoIterator::into_iter(elements).collect())
            }
        }

        impl<Ctx, T, $($h)?> $crate::BitDecode<Ctx, $crate::Untagged> for $ty<T, $($h)?>
        where
            T: $crate::BitDecode<Ctx> $(+ $tbound0 $(+ $tbound1)?)?,
            $($h: $hbound0 + $hbound1)?
        {
            fn decode(
                read: &mut dyn $crate::BitRead,
                byte_order: $crate::ByteOrder,
                ctx: &mut Ctx,
                _: $crate::Untagged,
            ) -> $crate::Result<Self> {
                Ok(::core::iter::IntoIterator::into_iter(
                    $crate::util::decode_items_to_eof(read, byte_order, ctx)?
                ).collect())
            }
        }
    }
}

macro_rules! impl_write_list {
    ( $ty:ident<T $(: $tbound0:ident $(+ $tbound1:ident)?)? $(, $h:ident)?> ) => {
        impl<Ctx, T, $($h)?> $crate::BitEncode<Ctx, $crate::Untagged> for $ty<T, $($h)?>
        where
            T: $crate::BitEncode<Ctx> $(+ $tbound0 $(+ $tbound1)?)?
        {
            fn encode(&self,
                write: &mut dyn $crate::BitWrite,
                byte_order: $crate::ByteOrder,
                ctx: &mut Ctx,
                _: $crate::Untagged,
            ) -> $crate::Result<()> {
                $crate::util::encode_items(self.iter(), write, byte_order, ctx)
            }
        }
    }
}

mod vec {
    use alloc::vec::Vec;

    impl_read_list!(Vec<T>);
    impl_write_list!(Vec<T>);

    #[cfg(test)]
    mod tests {
        use crate::{Tag, Untagged};

        use super::*;

        test_untagged_and_codec!(Vec<u8>| Untagged, Tag(3); alloc::vec![1, 2, 3] => [0x01, 0x02, 0x03]);
    }
}

mod linked_list {
    use alloc::collections::linked_list::LinkedList;

    impl_read_list!(LinkedList<T>);
    impl_write_list!(LinkedList<T>);

    #[cfg(test)]
    mod tests {
        use crate::{Tag, Untagged};

        use super::*;

        test_untagged_and_codec!(LinkedList<u8>| Untagged, Tag(3); [1, 2, 3].into() => [0x01, 0x02, 0x03]);
    }
}

mod vec_deque {
    use alloc::collections::vec_deque::VecDeque;

    impl_read_list!(VecDeque<T>);
    impl_write_list!(VecDeque<T>);

    #[cfg(test)]
    mod tests {
        use crate::{Tag, Untagged};

        use super::*;

        test_untagged_and_codec!(VecDeque<u8>| Untagged, Tag(3); [1, 2, 3].into() => [0x01, 0x02, 0x03]);
    }
}

mod b_tree_set {
    use alloc::collections::btree_set::BTreeSet;

    impl_read_list!(BTreeSet<T: Ord>);
    impl_write_list!(BTreeSet<T: Ord>);

    #[cfg(test)]
    mod tests {
        use crate::{Tag, Untagged};

        use super::*;

        test_untagged_and_codec!(BTreeSet<u8>| Untagged, Tag(3); [1, 2, 3].into() => [0x01, 0x02, 0x03]);
    }
}

mod binary_heap {
    use alloc::collections::binary_heap::BinaryHeap;

    impl_read_list!(BinaryHeap<T: Ord>);
    impl_write_list!(BinaryHeap<T: Ord>);

    // TODO
}

#[cfg(feature = "std")]
mod hash_set {
    use core::hash::{BuildHasher, Hash};
    use std::collections::HashSet;

    impl_read_list!(HashSet<T: Hash + Eq, H: BuildHasher + Default>);
    impl_write_list!(HashSet<T: Hash + Eq, H>);

    #[cfg(test)]
    mod tests {
        use crate::{Tag, Untagged};

        use super::*;

        test_untagged_and_codec!(HashSet<u8>| Untagged, Tag(1); [1].into() => [0x01]);
    }
}
