macro_rules! impl_container_write {
    (
        $ty:ident<$($a:lifetime,)? T $(: $tbound0:ident $(+ ?$tbound1:ident + $tbound2:lifetime)?)?>
        $(=> $f:ident)?
    ) => {
        mod impl_write {
            use core::ops::Deref;

            use super::*;

            impl<$($a,)? Ctx, T> $crate::ProtocolWrite<Ctx> for $ty<$($a,)? T>
            where
                T: $crate::ProtocolWrite<Ctx> $(+ $tbound0 $(+ ?$tbound1 + $tbound2)?)?,
            {
                fn write(
                    &self,
                    write: &mut dyn $crate::BitWrite,
                    byte_order: $crate::ByteOrder,
                    ctx: &mut Ctx,
                ) -> $crate::Result<()> {
                    $crate::ProtocolWrite::write(
                        self $(.$f()?)? .deref(),
                        write,
                        byte_order,
                        ctx,
                    )
                }
            }
        }
    };
}

macro_rules! impl_container_read {
    ($ty:ident<T>) => {
        impl<Ctx, T> $crate::ProtocolRead<Ctx> for $ty<T>
        where
            T: $crate::ProtocolRead<Ctx>,
        {
            fn read(
                read: &mut dyn $crate::BitRead,
                byte_order: $crate::ByteOrder,
                ctx: &mut Ctx,
            ) -> $crate::Result<Self> {
                Ok($ty::new($crate::ProtocolRead::read(read, byte_order, ctx)?))
            }
        }
    };
}

mod box_ {
    use alloc::boxed::Box;

    impl_container_write!(Box<T>);
    impl_container_read!(Box<T>);
}

mod rc {
    use alloc::rc::Rc;

    impl_container_write!(Rc<T>);
    impl_container_read!(Rc<T>);
}

mod arc {
    use alloc::sync::Arc;

    impl_container_write!(Arc<T>);
    impl_container_read!(Arc<T>);
}

mod cow {
    use alloc::borrow::{Cow, ToOwned};

    impl_container_write!(Cow<'a, T: ToOwned + ?Sized + 'a>);
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
}

#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
#[cfg(feature = "std")]
mod rwlock {
    use std::sync::RwLock;

    impl_container_write!(RwLock<T> => read);
}

#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
#[cfg(feature = "std")]
mod mutex {
    use std::sync::Mutex;

    impl_container_write!(Mutex<T> => lock);
}

mod ref_cell {
    use core::cell::RefCell;

    impl_container_write!(RefCell<T> => try_borrow);
}
