use crate::{BitRead, BitWrite, ByteOrder, ExternallyTagged, Protocol, Result};

impl<Ctx, T> ExternallyTagged<bool, Ctx> for Option<T>
where
    T: Protocol<Ctx>,
{
    fn read(
        read: &mut dyn BitRead,
        byte_order: ByteOrder,
        ctx: &mut Ctx,
        tag: bool,
    ) -> Result<Self> {
        if tag {
            let value = T::read(read, byte_order, ctx)?;
            Ok(Some(value))
        } else {
            Ok(None)
        }
    }

    fn write(&self, write: &mut dyn BitWrite, byte_order: ByteOrder, ctx: &mut Ctx) -> Result<()> {
        if let Some(ref value) = *self {
            value.write(write, byte_order, ctx)?;
        }
        Ok(())
    }
}

macro_rules! impl_externally_tagged_for_option {
    ($ty:ty) => {
        impl<Ctx, T> crate::ExternallyTagged<$ty, Ctx> for Option<T>
        where
            T: crate::Protocol<Ctx>,
        {
            fn read(
                read: &mut dyn crate::BitRead,
                byte_order: crate::ByteOrder,
                ctx: &mut Ctx,
                tag: $ty,
            ) -> Result<Self> {
                <Option<T> as crate::ExternallyTagged<bool, Ctx>>::read(
                    read,
                    byte_order,
                    ctx,
                    tag != 0,
                )
            }

            fn write(
                &self,
                write: &mut dyn crate::BitWrite,
                byte_order: crate::ByteOrder,
                ctx: &mut Ctx,
            ) -> Result<()> {
                <Option<T> as crate::ExternallyTagged<bool, Ctx>>::write(
                    self, write, byte_order, ctx,
                )
            }
        }
    };
}

impl_externally_tagged_for_option!(u8);
impl_externally_tagged_for_option!(u16);
impl_externally_tagged_for_option!(u32);
impl_externally_tagged_for_option!(u64);
impl_externally_tagged_for_option!(u128);
impl_externally_tagged_for_option!(usize);

#[cfg(test)]
mod tests {
    use bitstream_io::{BigEndian, BitReader, BitWriter};

    use super::*;

    #[test]
    fn can_read_some() {
        assert_eq!(
            <Option<u8> as ExternallyTagged<_, _>>::read(
                &mut BitReader::endian([5].as_slice(), BigEndian),
                ByteOrder::BigEndian,
                &mut (),
                true
            )
            .unwrap(),
            Some(5)
        )
    }

    #[test]
    fn can_read_none() {
        assert_eq!(
            <Option<u8> as ExternallyTagged<_, _>>::read(
                &mut BitReader::endian([].as_slice(), BigEndian),
                ByteOrder::BigEndian,
                &mut (),
                false
            )
            .unwrap(),
            None
        )
    }

    #[test]
    fn can_write_some() {
        let mut data: Vec<u8> = Vec::new();
        ExternallyTagged::<bool, _>::write(
            &Some(5u8),
            &mut BitWriter::endian(&mut data, BigEndian),
            ByteOrder::BigEndian,
            &mut (),
        )
        .unwrap();
        assert_eq!(data, vec![5])
    }

    #[test]
    fn can_write_none() {
        let mut data: Vec<u8> = Vec::new();
        ExternallyTagged::<bool, _>::write(
            &None::<u8>,
            &mut BitWriter::endian(&mut data, BigEndian),
            ByteOrder::BigEndian,
            &mut (),
        )
        .unwrap();
        assert_eq!(data, vec![])
    }
}
