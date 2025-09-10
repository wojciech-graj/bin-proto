#[allow(unused)]
macro_rules! impl_read_map {
    (
        $(#[$attr:meta])?
        $ty:ident<K: $kbound0:ident $(+ $kbound1:ident)?, V
        $(, $h:ident : $hbound0:ident + $hbound1:ident)?>
    ) => {
        $(#[$attr])?
        impl<Tag, Ctx, K, V, $($h)?> $crate::BitDecode<Ctx, $crate::Tag<Tag>> for $ty<K, V, $($h)?>
        where
            K: $crate::BitDecode<Ctx> + $kbound0 + $($kbound1)?,
            V: $crate::BitDecode<Ctx>,
            Tag: ::core::convert::TryInto<usize>,
            $($h: $hbound0 + $hbound1)?
        {
            fn decode<R, E>(
                read: &mut R,
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
        impl<Ctx, K, V, $($h)?> $crate::BitDecode<Ctx, $crate::Untagged> for $ty<K, V, $($h)?>
        where
            K: $crate::BitDecode<Ctx> + $kbound0 $(+ $kbound1)?,
            V: $crate::BitDecode<Ctx>,
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
                $crate::util::decode_items_to_eof::<_, E, _, _>(read,  ctx).collect()
            }
        }
    };
}

#[allow(unused)]
macro_rules! impl_write_map {
    ( $(#[$attr:meta])? $ty:ident<K: $kbound0:ident $(+ $kbound1:ident)?, V $(, $h:ident)?> ) => {
        $(#[$attr])?
        impl<Ctx, K, V, $($h)?> $crate::BitEncode<Ctx, $crate::Untagged> for $ty<K, V, $($h)?>
        where
            K: $crate::BitEncode<Ctx> + $kbound0 $(+ $kbound1)?,
            V: $crate::BitEncode<Ctx>
        {
            fn encode<W, E>(
                &self,
                write: &mut W,
                ctx: &mut Ctx,
                _: $crate::Untagged,
            ) -> $crate::Result<()>
            where
                W: ::bitstream_io::BitWrite,
                E: ::bitstream_io::Endianness,
            {
                for (key, value) in self.iter() {
                    $crate::BitEncode::encode::<_, E>(key, write,  ctx, ())?;
                    $crate::BitEncode::encode::<_, E>(value, write,  ctx, ())?;
                }

                Ok(())
            }
        }
    }
}

#[cfg(feature = "std")]
mod hash_map {
    use core::hash::{BuildHasher, Hash};
    use std::collections::HashMap;

    impl_write_map!(#[cfg_attr(docsrs, doc(cfg(feature = "std")))] HashMap<K: Eq + Hash, V, H>);
    impl_read_map!(#[cfg_attr(docsrs, doc(cfg(feature = "std")))] HashMap<K: Eq + Hash, V, H: BuildHasher + Default>);

    #[cfg(test)]
    mod tests {
        use crate::{Tag, Untagged};

        use super::*;

        test_untagged_and_codec!(
            HashMap<u8, u8>| Untagged, Tag(1); [(1, 2)].into() => [0x01, 0x02]
        );
    }
}

#[cfg(feature = "alloc")]
mod b_tree_map {
    use alloc::collections::btree_map::BTreeMap;

    impl_write_map!(#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))] BTreeMap<K: Ord, V>);
    impl_read_map!(#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))] BTreeMap<K: Ord, V>);

    #[cfg(test)]
    mod tests {
        use crate::{Tag, Untagged};

        use super::*;

        test_untagged_and_codec!(
            BTreeMap<u8, u8>| Untagged, Tag(3);
            [(1, 2), (3, 4), (5, 6)].into() => [0x01, 0x02, 0x03, 0x04, 0x05, 0x06]
        );
    }
}
