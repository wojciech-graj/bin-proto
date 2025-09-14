#[allow(unused)]
macro_rules! impl_read_list {
    (
        $(#[$attr:meta])?
        $ty:ident<T $(: $tbound0:ident $(+ $tbound1:ident)?)?
        $(, $h:ident: $hbound0:ident + $hbound1:ident)?>
    ) => {
        $(#[$attr])?
        impl<Tag, Ctx, T, $($h)?> $crate::BitDecode<Ctx, $crate::Tag<Tag>> for $ty<T, $($h)?>
        where
            T: $crate::BitDecode<Ctx> $(+ $tbound0 $(+ $tbound1)?)?,
            Tag: ::core::convert::TryInto<usize>,
            $($h: $hbound0 + $hbound1)?
        {
            fn decode<R, E>(read: &mut R,
                ctx: &mut Ctx,
                tag: $crate::Tag<Tag>,
            ) -> $crate::Result<Self>
            where
                R: ::bitstream_io::BitRead,
                E: ::bitstream_io::Endianness,
            {
                $crate::util::decode_items::<_, E, _, _>(
                    ::core::convert::TryInto::try_into(tag.0)
                        .map_err(|_| $crate::Error::TagConvert)?,
                    read,

                    ctx
                ).collect()
            }
        }

        $(#[$attr])?
        impl<Ctx, T, $($h)?> $crate::BitDecode<Ctx, $crate::Untagged> for $ty<T, $($h)?>
        where
            T: $crate::BitDecode<Ctx> $(+ $tbound0 $(+ $tbound1)?)?,
            $($h: $hbound0 + $hbound1)?
        {
            fn decode<R, E>(
                read: &mut R,
                ctx: &mut Ctx,
                _: $crate::Untagged,
            ) -> $crate::Result<Self>
            where
                R: ::bitstream_io::BitRead,
                E: ::bitstream_io::Endianness,
            {
                $crate::util::decode_items_to_eof::<_, E, _, _>(read, ctx).collect()
            }
        }
    }
}

#[allow(unused)]
macro_rules! impl_write_list {
    ($(#[$attr:meta])? $ty:ident<T $(: $tbound0:ident $(+ $tbound1:ident)?)? $(, $h:ident)?> ) => {
        $(#[$attr])?
        impl<Ctx, T, $($h)?> $crate::BitEncode<Ctx, $crate::Untagged> for $ty<T, $($h)?>
        where
            T: $crate::BitEncode<Ctx> $(+ $tbound0 $(+ $tbound1)?)?
        {
            fn encode<W, E>(&self,
                write: &mut W,
                ctx: &mut Ctx,
                _: $crate::Untagged,
            ) -> $crate::Result<()>
            where
                W: ::bitstream_io::BitWrite,
                E: ::bitstream_io::Endianness,
            {
                $crate::util::encode_items::<_, E, _, _>(self.iter(), write,  ctx)
            }
        }
    }
}

#[cfg(feature = "alloc")]
mod vec {
    use alloc::vec::Vec;

    impl_read_list!(#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))] Vec<T>);
    impl_write_list!(#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))] Vec<T>);

    #[cfg(test)]
    mod tests {
        use crate::{Tag, Untagged};

        use super::*;

        test_untagged_and_codec!(
            Vec<u8>| Untagged, Tag(3); alloc::vec![1, 2, 3] => [0x01, 0x02, 0x03]
        );
    }
}

#[cfg(feature = "alloc")]
mod linked_list {
    use alloc::collections::linked_list::LinkedList;

    impl_read_list!(#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))] LinkedList<T>);
    impl_write_list!(#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))] LinkedList<T>);

    #[cfg(test)]
    mod tests {
        use crate::{Tag, Untagged};

        use super::*;

        test_untagged_and_codec!(
            LinkedList<u8>| Untagged, Tag(3); [1, 2, 3].into() => [0x01, 0x02, 0x03]
        );
    }
}

#[cfg(feature = "alloc")]
mod vec_deque {
    use alloc::collections::vec_deque::VecDeque;

    impl_read_list!(#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))] VecDeque<T>);
    impl_write_list!(#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))] VecDeque<T>);

    #[cfg(test)]
    mod tests {
        use crate::{Tag, Untagged};

        use super::*;

        test_untagged_and_codec!(
            VecDeque<u8>| Untagged, Tag(3); [1, 2, 3].into() => [0x01, 0x02, 0x03]
        );
    }
}

#[cfg(feature = "alloc")]
mod b_tree_set {
    use alloc::collections::btree_set::BTreeSet;

    impl_read_list!(#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))] BTreeSet<T: Ord>);
    impl_write_list!(#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))] BTreeSet<T: Ord>);

    #[cfg(test)]
    mod tests {
        use crate::{Tag, Untagged};

        use super::*;

        test_untagged_and_codec!(
            BTreeSet<u8>| Untagged, Tag(3); [1, 2, 3].into() => [0x01, 0x02, 0x03]
        );
    }
}

#[cfg(feature = "alloc")]
mod binary_heap {
    use alloc::collections::binary_heap::BinaryHeap;

    impl_read_list!(#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))] BinaryHeap<T: Ord>);
    impl_write_list!(#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))] BinaryHeap<T: Ord>);

    #[cfg(test)]
    mod tests {
        use alloc::vec::Vec;

        use bitstream_io::{BigEndian, BitReader};

        use crate::{BitDecode, Tag, Untagged};

        use super::*;

        #[test]
        fn decode() {
            let bytes: &[u8] = &[0x01];
            let exp: BinaryHeap<u8> = [1].into();
            let read: BinaryHeap<u8> = BitDecode::decode::<_, BigEndian>(
                &mut BitReader::endian(bytes, BigEndian),
                &mut (),
                Tag(1),
            )
            .unwrap();
            assert_eq!(
                exp.into_iter().collect::<Vec<_>>(),
                read.into_iter().collect::<Vec<_>>()
            );
        }

        test_encode!(BinaryHeap<u8>| Untagged; [1].into() => [0x01]);
    }
}

#[cfg(feature = "std")]
mod hash_set {
    use core::hash::{BuildHasher, Hash};
    use std::collections::HashSet;

    impl_read_list!(#[cfg_attr(docsrs, doc(cfg(feature = "std")))] HashSet<T: Hash + Eq, H: BuildHasher + Default>);
    impl_write_list!(#[cfg_attr(docsrs, doc(cfg(feature = "std")))] HashSet<T: Hash + Eq, H>);

    #[cfg(test)]
    mod tests {
        use crate::{Tag, Untagged};

        use super::*;

        test_untagged_and_codec!(HashSet<u8>| Untagged, Tag(1); [1].into() => [0x01]);
    }
}
