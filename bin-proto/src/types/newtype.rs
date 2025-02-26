macro_rules! impl_newtype {
    ($ty:ident) => {
        impl<Ctx, T> $crate::ProtocolRead<Ctx> for $ty<T>
        where
            T: $crate::ProtocolRead<Ctx>,
        {
            fn read(
                read: &mut dyn $crate::BitRead,
                byte_order: $crate::ByteOrder,
                ctx: &mut Ctx,
            ) -> $crate::Result<Self> {
                Ok(Self($crate::ProtocolRead::read(read, byte_order, ctx)?))
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
                $crate::ProtocolWrite::write(&self.0, write, byte_order, ctx)
            }
        }

        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn can_read() {
                let mut buffer = ::bitstream_io::BitReader::endian(
                    [0x1u8].as_slice(),
                    ::bitstream_io::BigEndian,
                );
                let read_back: $ty<u8> =
                    $crate::ProtocolRead::read(&mut buffer, $crate::ByteOrder::BigEndian, &mut ())
                        .unwrap();
                assert_eq!(read_back, $ty(1u8));
            }

            #[test]
            fn can_write() {
                let mut buffer: ::alloc::vec::Vec<u8> = ::alloc::vec::Vec::new();
                let value: $ty<u8> = $ty(1u8);
                $crate::ProtocolWrite::write(
                    &value,
                    &mut ::bitstream_io::BitWriter::endian(&mut buffer, ::bitstream_io::BigEndian),
                    $crate::ByteOrder::BigEndian,
                    &mut (),
                )
                .unwrap();
                assert_eq!(::alloc::vec![1u8], buffer);
            }
        }
    };
}

mod wrapping {
    use core::num::Wrapping;

    impl_newtype!(Wrapping);
}

mod saturating {
    use core::num::Saturating;

    impl_newtype!(Saturating);
}
