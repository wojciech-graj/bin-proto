macro_rules! impl_smart_ptr_type {
    ($ty:ident) => {
        #[allow(unused_imports)]
        mod impl_protocol {
            use super::*;
            use std::ops::Deref;

            impl<Ctx, T> $crate::Protocol<Ctx> for $ty<T>
            where
                T: $crate::Protocol<Ctx>,
            {
                fn read(
                    read: &mut dyn $crate::BitRead,
                    byte_order: $crate::ByteOrder,
                    ctx: &mut Ctx,
                ) -> $crate::Result<Self> {
                    let value = T::read(read, byte_order, ctx)?;
                    Ok($ty::new(value))
                }

                fn write(
                    &self,
                    write: &mut dyn $crate::BitWrite,
                    byte_order: $crate::ByteOrder,
                    ctx: &mut Ctx,
                ) -> $crate::Result<()> {
                    self.deref().write(write, byte_order, ctx)
                }
            }
        }

        #[cfg(test)]
        #[allow(unused_imports)]
        mod tests {
            use super::*;

            #[test]
            fn read_protocol() {
                assert_eq!(
                    <$ty<u8> as $crate::Protocol<()>>::read(
                        &mut ::bitstream_io::BitReader::endian(
                            [7u8].as_slice(),
                            ::bitstream_io::BigEndian
                        ),
                        $crate::ByteOrder::BigEndian,
                        &mut ()
                    )
                    .unwrap(),
                    $ty::new(7)
                )
            }

            #[test]
            fn write_protocol() {
                let mut data: Vec<u8> = Vec::new();
                $crate::Protocol::write(
                    &$ty::new(7u8),
                    &mut ::bitstream_io::BitWriter::endian(&mut data, bitstream_io::BigEndian),
                    $crate::ByteOrder::BigEndian,
                    &mut (),
                )
                .unwrap();
                assert_eq!(vec![7], data);
            }
        }
    };
}

mod box_ {
    impl_smart_ptr_type!(Box);
}

mod rc {
    use std::rc::Rc;

    impl_smart_ptr_type!(Rc);
}

mod arc {
    use std::sync::Arc;

    impl_smart_ptr_type!(Arc);
}
