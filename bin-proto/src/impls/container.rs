macro_rules! impl_container_write {
    (
        $ty:ident<$($a:lifetime,)? T $(: $tbound0:ident $(+ ?$tbound1:ident + $tbound2:lifetime)?)?>
        $(=> $f:ident)?
    ) => {
        impl<$($a,)? Tag, Ctx, T> $crate::ProtocolWrite<Ctx, Tag> for $ty<$($a,)? T>
        where
            T: $crate::ProtocolWrite<Ctx, Tag> $(+ $tbound0 $(+ ?$tbound1 + $tbound2)?)?,
        {
            fn write(
                &self,
                write: &mut dyn $crate::BitWrite,
                byte_order: $crate::ByteOrder,
                ctx: &mut Ctx,
            ) -> $crate::Result<()> {
                use core::ops::Deref;

                $crate::ProtocolWrite::write(
                    self $(.$f()?)? .deref(),
                    write,
                    byte_order,
                    ctx,
                )
            }
        }
    };
}

macro_rules! impl_container_read {
    ($ty:ident<T>) => {
        impl<Ctx, Tag, T> $crate::ProtocolRead<Ctx, Tag> for $ty<T>
        where
            T: $crate::ProtocolRead<Ctx, Tag>,
        {
            fn read(
                read: &mut dyn $crate::BitRead,
                byte_order: $crate::ByteOrder,
                ctx: &mut Ctx,
                tag: Tag,
            ) -> $crate::Result<Self> {
                Ok($ty::new($crate::ProtocolRead::read(
                    read, byte_order, ctx, tag,
                )?))
            }
        }
    };
}

mod box_ {
    use alloc::boxed::Box;

    impl_container_write!(Box<T>);
    impl_container_read!(Box<T>);
    test_protocol!(Box<u8>: Box::new(1) => [0x01]);
}

mod rc {
    use alloc::rc::Rc;

    impl_container_write!(Rc<T>);
    impl_container_read!(Rc<T>);
    test_protocol!(Rc<u8>: Rc::new(1) => [0x01]);
}

mod arc {
    use alloc::sync::Arc;

    impl_container_write!(Arc<T>);
    impl_container_read!(Arc<T>);
    test_protocol!(Arc<u8>: Arc::new(1) => [0x01]);
}

mod cow {
    use alloc::borrow::{Cow, ToOwned};

    impl_container_write!(Cow<'a, T: ToOwned + ?Sized + 'a>);
    test_protocol_write!(Cow<u8>: Cow::Owned(1) => [0x01]);
}

mod cell {
    use core::cell::Cell;

    use crate::{BitWrite, ByteOrder, ProtocolWrite, Result};

    impl<Ctx, T> ProtocolWrite<Ctx> for Cell<T>
    where
        T: ProtocolWrite<Ctx> + Copy,
    {
        fn write(
            &self,
            write: &mut dyn BitWrite,
            byte_order: ByteOrder,
            ctx: &mut Ctx,
        ) -> Result<()> {
            self.get().write(write, byte_order, ctx)
        }
    }

    test_protocol_write!(Cell<u8>: Cell::new(1) => [0x01]);
}

#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
#[cfg(feature = "std")]
mod rwlock {
    use std::sync::RwLock;

    impl_container_write!(RwLock<T> => read);

    #[cfg(test)]
    mod tests {
        use alloc::vec::Vec;

        use bitstream_io::{BigEndian, BitWriter};

        use crate::{ByteOrder, ProtocolWrite};

        use super::*;

        #[test]
        fn protocol_write() {
            let mut buffer: Vec<u8> = Vec::new();
            ProtocolWrite::write(
                &RwLock::new(1u8),
                &mut BitWriter::endian(&mut buffer, BigEndian),
                ByteOrder::BigEndian,
                &mut (),
            )
            .unwrap();
            assert_eq!([1], *buffer);
        }
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
#[cfg(feature = "std")]
mod mutex {
    use std::sync::Mutex;

    impl_container_write!(Mutex<T> => lock);
    test_protocol_write!(Mutex<u8>: Mutex::new(1) => [0x01]);
}

mod ref_cell {
    use core::cell::RefCell;

    impl_container_write!(RefCell<T> => try_borrow);
    test_protocol_write!(RefCell<u8>: RefCell::new(1) => [0x01]);
}
