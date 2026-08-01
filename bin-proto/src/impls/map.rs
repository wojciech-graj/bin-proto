#[allow(unused)]
macro_rules! impl_read_map {
    (
        $ty:ident<K: $kbound0:ident $(+ $kbound1:ident)?, V
        $(, $h:ident : $hbound0:ident + $hbound1:ident)?>,
        |$item_count:ident| $new:expr
    ) => {
        impl<E, Ctx, Tag, K, V, $($h)?> $crate::BitDecode<E, Ctx, $crate::Tag<Tag>> for $ty<K, V, $($h)?>
        where
            E: ::bitstream_io::Endianness,
            K: $crate::BitDecode<E, Ctx> + $kbound0 + $($kbound1)?,
            V: $crate::BitDecode<E, Ctx>,
            Tag: ::core::convert::TryInto<usize>,
            $($h: $hbound0 + $hbound1)?
        {
            fn decode<R>(
                read: &mut R,
                ctx: &mut Ctx,
                tag: $crate::Tag<Tag>,
            ) -> $crate::Result<Self>
            where
                R: ::bitstream_io::BitRead + ?Sized,
            {
                let $item_count = ::core::convert::TryInto::try_into(tag.0)
                    .map_err(|_| $crate::Error::from_inner($crate::error::ErrorCause::TagConvert))?;
                let mut this = $new;
                for _ in 0..$item_count {
                    this.insert(
                        $crate::BitDecode::<_, _, _>::decode(read, ctx, ())?,
                        $crate::BitDecode::<_, _, _>::decode(read, ctx, ())?
                    );
                }
                ::core::result::Result::Ok(this)
            }
        }

        impl<E, Ctx, K, V, $($h)?> $crate::BitDecode<E, Ctx, $crate::Untagged> for $ty<K, V, $($h)?>
        where
            E: ::bitstream_io::Endianness,
            K: $crate::BitDecode<E, Ctx> + $kbound0 $(+ $kbound1)?,
            V: $crate::BitDecode<E, Ctx>,
            $($h: $hbound0 + $hbound1)?
        {
            fn decode<R>(
                read: &mut R,
                ctx: &mut Ctx,
                _: $crate::Untagged,
            ) -> $crate::Result<Self>
            where
                R: ::bitstream_io::BitRead + ?Sized,
            {
                $crate::util::decode_items_to_eof::<_, E, _, _>(read,  ctx).collect()
            }
        }

        #[cfg(feature = "prepend-tags")]
        impl<E, Ctx, K, V, $($h)?> $crate::BitDecode<E, Ctx> for $ty<K, V, $($h)?>
        where
            E: ::bitstream_io::Endianness,
            K: $crate::BitDecode<E, Ctx> + $kbound0 $(+ $kbound1)?,
            V: $crate::BitDecode<E, Ctx>,
            $($h: $hbound0 + $hbound1)?
        {
            fn decode<R>(
                read: &mut R,
                ctx: &mut Ctx,
                (): (),
            ) -> $crate::Result<Self>
            where
                R: ::bitstream_io::BitRead + ?Sized,
            {
                let tag: usize = $crate::BitDecode::<E, _, _>::decode(read, ctx, ())?;
                $crate::BitDecode::<E, _, _>::decode(read, ctx, $crate::Tag(tag))
            }
        }
    };
}

#[allow(unused)]
macro_rules! impl_write_map {
    ( $ty:ident<K: $kbound0:ident $(+ $kbound1:ident)?, V $(, $h:ident)?> ) => {
        impl<E, Ctx, K, V, $($h)?> $crate::BitEncode<E, Ctx, $crate::Untagged> for $ty<K, V, $($h)?>
        where
            E: ::bitstream_io::Endianness,
            K: $crate::BitEncode<E, Ctx> + $kbound0 $(+ $kbound1)?,
            V: $crate::BitEncode<E, Ctx>
        {
            fn encode<W>(
                &self,
                write: &mut W,
                ctx: &mut Ctx,
                _: $crate::Untagged,
            ) -> $crate::Result<()>
            where
                W: ::bitstream_io::BitWrite + ?Sized,
            {
                for (key, value) in self.iter() {
                    $crate::BitEncode::encode(key, write,  ctx, ())?;
                    $crate::BitEncode::encode(value, write,  ctx, ())?;
                }

                Ok(())
            }
        }

        #[cfg(feature = "prepend-tags")]
        impl<E, Ctx, K, V, $($h)?> $crate::BitEncode<E, Ctx> for $ty<K, V, $($h)?>
        where
            E: ::bitstream_io::Endianness,
            K: $crate::BitEncode<E, Ctx> + $kbound0 $(+ $kbound1)?,
            V: $crate::BitEncode<E, Ctx>
        {
            fn encode<W>(
                &self,
                write: &mut W,
                ctx: &mut Ctx,
                (): (),
            ) -> $crate::Result<()>
            where
                W: ::bitstream_io::BitWrite + ?Sized,
            {
                $crate::BitEncode::<E, _, _>::encode(&self.len(), write, ctx, ())?;
                $crate::BitEncode::<E, _, _>::encode(self, write, ctx, $crate::Untagged)
            }
        }
    }
}

#[cfg(feature = "std")]
mod hash_map {
    use core::hash::{BuildHasher, Hash};
    use std::collections::HashMap;

    impl_write_map!(HashMap<K: Eq + Hash, V, H>);
    impl_read_map!(
        HashMap<K: Eq + Hash, V, H: BuildHasher + Default>,
        |n| {
            let mut this = Self::with_hasher(H::default());
            this.try_reserve(n)?;
            this
        }
    );

    #[cfg(test)]
    mod tests {
        use crate::{Tag, Untagged};

        use super::*;

        test_untagged_and_codec!(
            HashMap<u8, u8>| Untagged, Tag(1); [(1, 2)].into() => [0x01, 0x02]
        );

        test_length_tag_decode!(HashMap<u8, u8>);

        #[cfg(feature = "prepend-tags")]
        test_roundtrip!(HashMap::<i32, i64>);
    }
}

#[cfg(feature = "alloc")]
mod b_tree_map {
    use alloc::collections::btree_map::BTreeMap;

    impl_write_map!(BTreeMap<K: Ord, V>);
    impl_read_map!(BTreeMap<K: Ord, V>, |n| Self::new());

    #[cfg(test)]
    mod tests {
        use crate::{Tag, Untagged};

        use super::*;

        test_untagged_and_codec!(
            BTreeMap<u8, u8>| Untagged, Tag(3);
            [(1, 2), (3, 4), (5, 6)].into() => [0x01, 0x02, 0x03, 0x04, 0x05, 0x06]
        );

        #[cfg(feature = "prepend-tags")]
        test_roundtrip!(BTreeMap::<i32, i64>);
    }
}
