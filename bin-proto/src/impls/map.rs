macro_rules! impl_read_map {
    (
        $ty:ident<K: $kbound0:ident $(+ $kbound1:ident)?, V
        $(, $h:ident : $hbound0:ident + $hbound1:ident)?>
    ) => {
        impl<Tag, Ctx, K, V, $($h)?> $crate::ProtocolRead<Ctx, (Tag,)> for $ty<K, V, $($h)?>
        where
            K: $crate::ProtocolRead<Ctx> + $kbound0 + $($kbound1)?,
            V: $crate::ProtocolRead<Ctx>,
            Tag: ::core::convert::TryInto<usize>,
            $($h: $hbound0 + $hbound1)?
        {
            fn read(read: &mut dyn $crate::BitRead,
                byte_order: $crate::ByteOrder,
                ctx: &mut Ctx,
                tag: (Tag,),
            ) -> $crate::Result<Self> {
                let elements = $crate::util::read_items(
                    ::core::convert::TryInto::try_into(tag.0).map_err(|_| $crate::Error::TagConvert)?,
                    read,
                    byte_order,
                    ctx
                )?;
                Ok(::core::iter::IntoIterator::into_iter(elements).collect())
            }
        }

        impl<Ctx, K, V, $($h)?> $crate::ProtocolRead<Ctx, $crate::Untagged> for $ty<K, V, $($h)?>
        where
            K: $crate::ProtocolRead<Ctx> + $kbound0 $(+ $kbound1)?,
            V: $crate::ProtocolRead<Ctx>,
            $($h: $hbound0 + $hbound1)?
        {
            fn read(read: &mut dyn $crate::BitRead,
                byte_order: $crate::ByteOrder,
                ctx: &mut Ctx,
                _: $crate::Untagged,
            ) -> $crate::Result<Self> {
                Ok(::core::iter::IntoIterator::into_iter(
                    $crate::util::read_items_to_eof(read, byte_order, ctx)?
                ).collect())
            }
        }
    };
}

macro_rules! impl_write_map {
    ( $ty:ident<K: $kbound0:ident $(+ $kbound1:ident)?, V $(, $h:ident)?> ) => {
        impl<Ctx, K, V, $($h)?> $crate::ProtocolWrite<Ctx, $crate::Untagged> for $ty<K, V, $($h)?>
        where
            K: $crate::ProtocolWrite<Ctx> + $kbound0 $(+ $kbound1)?,
            V: $crate::ProtocolWrite<Ctx>
        {
            fn write(&self, write: &mut dyn $crate::BitWrite,
                    byte_order: $crate::ByteOrder,
                    ctx: &mut Ctx,
                    ) -> $crate::Result<()> {
                for (key, value) in self.iter() {
                    $crate::ProtocolWrite::write(key, write, byte_order, ctx)?;
                    $crate::ProtocolWrite::write(value, write, byte_order, ctx)?;
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

    impl_write_map!(HashMap<K: Eq + Hash, V, H>);
    impl_read_map!(HashMap<K: Eq + Hash, V, H: BuildHasher + Default>);
    test_flexible_array_member_read_and_protocol!(HashMap<u8, u8>| 1: [(1, 2)].into() => [0x01, 0x02]);
}

mod b_tree_map {
    use alloc::collections::btree_map::BTreeMap;

    impl_write_map!(BTreeMap<K: Ord, V>);
    impl_read_map!(BTreeMap<K: Ord, V>);
    test_flexible_array_member_read_and_protocol!(BTreeMap<u8, u8>| 3: [(1, 2), (3, 4), (5, 6)].into() => [0x01, 0x02, 0x03, 0x04, 0x05, 0x06]);
}
