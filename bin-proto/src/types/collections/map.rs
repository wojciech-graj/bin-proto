macro_rules! impl_map_type {
    ( $ty:ident => K: $( $k_pred:ident ),+ ) => {
        impl<Ctx, K, V> crate::ExternallyTagged<usize, Ctx> for $ty<K, V>
            where K: crate::Protocol<Ctx> + $( $k_pred +)+,
                  V: crate::Protocol<Ctx>
        {
            fn read(read: &mut dyn crate::BitRead,
                    byte_order: crate::ByteOrder,
                    ctx: &mut Ctx,
                    tag: usize,
                    ) -> crate::Result<Self> {
                let mut map = $ty::new();

                for _ in 0..tag {
                    let key = K::read(read, byte_order, ctx)?;
                    let value = V::read(read, byte_order, ctx)?;

                    map.insert(key, value);
                }

                Ok(map)
            }

            fn write(&self, write: &mut dyn crate::BitWrite,
                    byte_order: crate::ByteOrder,
                    ctx: &mut Ctx,
                    ) -> crate::Result<()> {
                for (key, value) in self.iter() {
                    key.write(write, byte_order, ctx)?;
                    value.write(write, byte_order, ctx)?;
                }

                Ok(())
            }
        }
    }
}

macro_rules! test_map_type {
    ( $t:ident ) => {
        #[cfg(test)]
        #[allow(unused_imports)]
        mod tests {
            use super::*;

            test_externally_tagged!($t<u8, u16> => [[0x01, 0x00, 0x02, 0x03, 0x00, 0x04], $t::from([(1, 2), (3, 4)])]);
        }
    }
}

mod hash_map {
    use std::collections::HashMap;
    use std::hash::Hash;

    impl_map_type!(HashMap => K: Hash, Eq);
}

mod b_tree_map {
    use std::collections::BTreeMap;

    impl_map_type!(BTreeMap => K: Ord);
    test_map_type!(BTreeMap);
}
