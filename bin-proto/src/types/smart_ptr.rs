macro_rules! impl_smart_ptr_type {
    ($ty:ident) => {
        #[allow(unused_imports)]
        mod impl_protocol {
            use super::*;
            use core::ops::Deref;

            impl<Ctx, T> $crate::ProtocolRead<Ctx> for $ty<T>
            where
                T: $crate::ProtocolRead<Ctx>,
            {
                fn read(
                    read: &mut dyn $crate::BitRead,
                    byte_order: $crate::ByteOrder,
                    ctx: &mut Ctx,
                ) -> $crate::Result<Self> {
                    let value = T::read(read, byte_order, ctx)?;
                    Ok($ty::new(value))
                }
            }

            impl<Ctx, T> $crate::ProtocolWrite<Ctx> for $ty<T>
            where
                T: $crate::ProtocolWrite<Ctx>,
            {
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
                    <$ty<u8> as $crate::ProtocolRead<()>>::read(
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
                let mut data: ::alloc::vec::Vec<u8> = ::alloc::vec::Vec::new();
                $crate::ProtocolWrite::write(
                    &$ty::new(7u8),
                    &mut ::bitstream_io::BitWriter::endian(&mut data, ::bitstream_io::BigEndian),
                    $crate::ByteOrder::BigEndian,
                    &mut (),
                )
                .unwrap();
                assert_eq!(::alloc::vec![7], data);
            }
        }
    };
}

mod box_ {
    use alloc::boxed::Box;

    impl_smart_ptr_type!(Box);
}

mod rc {
    use alloc::rc::Rc;

    impl_smart_ptr_type!(Rc);
}

mod arc {
    use alloc::sync::Arc;

    impl_smart_ptr_type!(Arc);
}
