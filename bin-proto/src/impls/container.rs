#[allow(unused)]
macro_rules! impl_container_write {
    (
        $ty:ident<$($a:lifetime,)? T $(: $tbound0:ident $(+ ?$tbound1:ident + $tbound2:lifetime)?)?>
        $(=> $f:ident)?
    ) => {
        impl<$($a,)? Ctx, Tag, T> $crate::BitEncode<Ctx, Tag> for $ty<$($a,)? T>
        where
            T: $crate::BitEncode<Ctx, Tag> $(+ $tbound0 $(+ ?$tbound1 + $tbound2)?)?,
        {
            fn encode<W, E>(
                &self,
                write: &mut W,
                ctx: &mut Ctx,
                tag: Tag,
            ) -> $crate::Result<()>
            where
                W: ::bitstream_io::BitWrite,
                E: ::bitstream_io::Endianness,
            {
                use core::ops::Deref;

                $crate::BitEncode::encode::<_, E>(
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
    ($ty:ident<T>) => {
        impl<Ctx, Tag, T> $crate::BitDecode<Ctx, Tag> for $ty<T>
        where
            T: $crate::BitDecode<Ctx, Tag>,
        {
            fn decode<R, E>(read: &mut R, ctx: &mut Ctx, tag: Tag) -> $crate::Result<Self>
            where
                R: ::bitstream_io::BitRead,
                E: ::bitstream_io::Endianness,
            {
                Ok($ty::new($crate::BitDecode::decode::<_, E>(read, ctx, tag)?))
            }
        }
    };
}

#[cfg(feature = "alloc")]
mod box_ {
    use alloc::boxed::Box;

    impl_container_write!(Box<T>);
    impl_container_read!(Box<T>);
    test_codec!(Box<u8>; Box::new(1) => [0x01]);
    test_roundtrip!(Box<u8>);
}

#[cfg(feature = "alloc")]
mod rc {
    use alloc::rc::Rc;

    impl_container_write!(Rc<T>);
    impl_container_read!(Rc<T>);
    test_codec!(Rc<u8>; Rc::new(1) => [0x01]);
    test_roundtrip!(Rc<u8>);
}

#[cfg(feature = "alloc")]
mod arc {
    use alloc::sync::Arc;

    impl_container_write!(Arc<T>);
    impl_container_read!(Arc<T>);
    test_codec!(Arc<u8>; Arc::new(1) => [0x01]);
    test_roundtrip!(Arc<u8>);
}

#[cfg(feature = "alloc")]
mod cow {
    use alloc::borrow::{Cow, ToOwned};

    impl_container_write!(Cow<'a, T: ToOwned + ?Sized + 'a>);
    test_encode!(Cow<u8>; Cow::Owned(1) => [0x01]);
}

mod cell {
    use core::cell::Cell;

    use bitstream_io::{BitWrite, Endianness};

    use crate::{BitEncode, Result};

    impl<Ctx, Tag, T> BitEncode<Ctx, Tag> for Cell<T>
    where
        T: BitEncode<Ctx, Tag> + Copy,
    {
        fn encode<W, E>(&self, write: &mut W, ctx: &mut Ctx, tag: Tag) -> Result<()>
        where
            W: BitWrite,
            E: Endianness,
        {
            self.get().encode::<_, E>(write, ctx, tag)
        }
    }

    test_encode!(Cell<u8>; Cell::new(1) => [0x01]);
}

#[cfg(feature = "std")]
mod rwlock {
    use std::sync::RwLock;

    impl_container_write!(RwLock<T> => read);

    #[cfg(test)]
    mod tests {
        use alloc::vec::Vec;

        use bitstream_io::{BigEndian, BitWriter};

        use crate::BitEncode;

        use super::*;

        #[test]
        fn encode() {
            let mut buffer: Vec<u8> = Vec::new();
            BitEncode::encode::<_, BigEndian>(
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

    impl_container_write!(Mutex<T> => lock);
    test_encode!(Mutex<u8>; Mutex::new(1) => [0x01]);
}

mod ref_cell {
    use core::cell::RefCell;

    impl_container_write!(RefCell<T> => try_borrow);
    test_encode!(RefCell<u8>; RefCell::new(1) => [0x01]);
}
