use crate::{BitRead, BitWrite, Error, Parcel, Settings};
use std::marker::PhantomData;

impl<T> Parcel for PhantomData<T> {
    fn read_field(_: &mut dyn BitRead, _: &Settings) -> Result<Self, Error> {
        Ok(PhantomData)
    }

    fn write_field(&self, _: &mut dyn BitWrite, _: &Settings) -> Result<(), Error> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_read_phantom_data() {
        assert_eq!(
            PhantomData::<u8>::from_raw_bytes(&[], &Settings::default()).unwrap(),
            PhantomData
        )
    }

    #[test]
    fn can_write_phantom_data() {
        assert_eq!(
            PhantomData::<u8>.raw_bytes(&Settings::default()).unwrap(),
            &[]
        )
    }
}
