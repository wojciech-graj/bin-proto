macro_rules! impl_smart_ptr_type {
    ($ty:ident) => {
        #[allow(unused_imports)]
        mod impl_protocol {
            use super::*;
            use std::ops::Deref;

            impl<T: crate::Protocol> crate::Protocol for $ty<T> {
                fn read(
                    read: &mut dyn crate::BitRead,
                    settings: &crate::Settings,
                    ctx: &mut dyn core::any::Any,
                ) -> Result<Self, crate::Error> {
                    let value = T::read(read, settings, ctx)?;
                    Ok($ty::new(value))
                }

                fn write(
                    &self,
                    write: &mut dyn crate::BitWrite,
                    settings: &crate::Settings,
                    ctx: &mut dyn core::any::Any,
                ) -> Result<(), crate::Error> {
                    self.deref().write(write, settings, ctx)
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
                    <$ty<u8> as crate::Protocol>::read(
                        &mut bitstream_io::BitReader::endian(
                            [7u8].as_slice(),
                            bitstream_io::BigEndian
                        ),
                        &crate::Settings::default(),
                        &mut ()
                    )
                    .unwrap(),
                    $ty::new(7)
                )
            }

            #[test]
            fn write_protocol() {
                let mut data: Vec<u8> = Vec::new();
                crate::Protocol::write(
                    &$ty::new(7u8),
                    &mut bitstream_io::BitWriter::endian(&mut data, bitstream_io::BigEndian),
                    &crate::Settings::default(),
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
