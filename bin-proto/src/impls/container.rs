macro_rules! impl_container_write {
    (
        $ty:ident<$($a:lifetime,)? T $(: ?$tbound0:ident $(+ $tbound1:ident + $tbound2:lifetime)?)?>
        $(=> $f:ident)?
    ) => {
        impl<$($a,)? E, Ctx, Tag, T> $crate::BitEncode<E, Ctx, Tag> for $ty<$($a,)? T>
        where
            E: ::bitstream_io::Endianness,
            T: $crate::BitEncode<E, Ctx, Tag> $(+ ?$tbound0 $(+ $tbound1 + $tbound2)?)?,
        {
            fn encode<W>(
                &self,
                write: &mut W,
                ctx: &mut Ctx,
                tag: Tag,
            ) -> $crate::Result<()>
            where
                W: ::bitstream_io::BitWrite + ?Sized,
            {
                use core::ops::Deref;

                $crate::BitEncode::encode(
                    self $(.$f()?)? .deref(),
                    write,
                    ctx,
                    tag,
                )
            }
        }
    };
}

#[allow(unused)]
macro_rules! impl_container_read {
    ($ty:ident<T $(: ?$tbound:ident)?>) => {
        impl<E, Ctx, Tag, T> $crate::BitDecode<E, Ctx, Tag> for $ty<T>
        where
            E: ::bitstream_io::Endianness,
            T: $crate::BitDecode<E, Ctx, Tag> $(+ ?$tbound)?,
        {
            fn decode<R>(read: &mut R, ctx: &mut Ctx, tag: Tag) -> $crate::Result<Self>
            where
                R: ::bitstream_io::BitRead + ?Sized,
            {
                Ok($ty::new($crate::BitDecode::decode(read, ctx, tag)?))
            }
        }
    };
}

#[cfg(feature = "alloc")]
mod box_ {
    use alloc::boxed::Box;

    impl_container_write!(Box<T: ?Sized>);
    impl_container_read!(Box<T>);
    test_codec!(Box<u8>; Box::new(1) => [0x01]);
    test_roundtrip!(Box<u8>);
}

#[cfg(feature = "alloc")]
mod rc {
    use alloc::rc::Rc;

    impl_container_write!(Rc<T: ?Sized>);
    impl_container_read!(Rc<T>);
    test_codec!(Rc<u8>; Rc::new(1) => [0x01]);
    test_roundtrip!(Rc<u8>);
}

#[cfg(feature = "alloc")]
mod arc {
    use alloc::sync::Arc;

    impl_container_write!(Arc<T: ?Sized>);
    impl_container_read!(Arc<T>);
    test_codec!(Arc<u8>; Arc::new(1) => [0x01]);
    test_roundtrip!(Arc<u8>);
}

#[cfg(feature = "alloc")]
mod cow {
    use alloc::borrow::{Cow, ToOwned};

    impl_container_write!(Cow<'a, T: ?Sized + ToOwned + 'a>);
    test_encode!(Cow<u8>; Cow::Owned(1) => [0x01]);
}

mod cell {
    use core::cell::Cell;

    use bitstream_io::{BitWrite, Endianness};

    use crate::{BitEncode, Result};

    impl<E, Ctx, Tag, T> BitEncode<E, Ctx, Tag> for Cell<T>
    where
        E: Endianness,
        T: BitEncode<E, Ctx, Tag> + Copy,
    {
        fn encode<W>(&self, write: &mut W, ctx: &mut Ctx, tag: Tag) -> Result<()>
        where
            W: BitWrite + ?Sized,
            E: Endianness,
        {
            self.get().encode(write, ctx, tag)
        }
    }

    test_encode!(Cell<u8>; Cell::new(1) => [0x01]);
}

#[cfg(feature = "std")]
mod rwlock {
    use std::sync::RwLock;

    impl_container_write!(RwLock<T: ?Sized> => read);

    #[cfg(test)]
    mod tests {
        use alloc::vec::Vec;

        use bitstream_io::{BigEndian, BitWriter};

        use crate::BitEncode;

        use super::*;

        #[test]
        fn encode() {
            let mut buffer: Vec<u8> = Vec::new();
            BitEncode::<BigEndian>::encode(
                &RwLock::new(1u8),
                &mut BitWriter::endian(&mut buffer, BigEndian),
                &mut (),
                (),
            )
            .unwrap();
            assert_eq!([1], *buffer);
        }
    }
}

#[cfg(feature = "std")]
mod mutex {
    use std::sync::Mutex;

    impl_container_write!(Mutex<T: ?Sized> => lock);
    test_encode!(Mutex<u8>; Mutex::new(1) => [0x01]);
}

mod ref_cell {
    use core::cell::RefCell;

    impl_container_write!(RefCell<T: ?Sized> => try_borrow);
    test_encode!(RefCell<u8>; RefCell::new(1) => [0x01]);
}
