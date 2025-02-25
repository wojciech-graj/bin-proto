use crate::{
    BitRead, BitWrite, ByteOrder, Error, ProtocolRead, ProtocolWrite, Result, TaggedRead,
    UntaggedWrite,
};

impl<Tag, Ctx, T> TaggedRead<Tag, Ctx> for Option<T>
where
    T: ProtocolRead<Ctx>,
    Tag: TryInto<bool>,
{
    fn read(
        read: &mut dyn BitRead,
        byte_order: ByteOrder,
        ctx: &mut Ctx,
        tag: Tag,
    ) -> Result<Self> {
        if tag.try_into().map_err(|_| Error::TagConvert)? {
            let value = T::read(read, byte_order, ctx)?;
            Ok(Some(value))
        } else {
            Ok(None)
        }
    }
}

impl<Ctx, T> UntaggedWrite<Ctx> for Option<T>
where
    T: ProtocolWrite<Ctx>,
{
    fn write(&self, write: &mut dyn BitWrite, byte_order: ByteOrder, ctx: &mut Ctx) -> Result<()> {
        if let Some(ref value) = *self {
            value.write(write, byte_order, ctx)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use alloc::{vec, vec::Vec};

    use bitstream_io::{BigEndian, BitReader, BitWriter};

    use super::*;

    #[test]
    fn can_read_some() {
        assert_eq!(
            <Option<u8> as TaggedRead<_, _>>::read(
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
            <Option<u8> as TaggedRead<_, _>>::read(
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
        UntaggedWrite::write(
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
        UntaggedWrite::write(
            &None::<u8>,
            &mut BitWriter::endian(&mut data, BigEndian),
            ByteOrder::BigEndian,
            &mut (),
        )
        .unwrap();
        assert_eq!(data, vec![])
    }
}
