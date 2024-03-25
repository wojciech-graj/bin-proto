use crate::{BitRead, BitWrite, Error, Protocol, Settings};
use std::marker::PhantomData;

impl<T> Protocol for PhantomData<T> {
    fn read(_: &mut dyn BitRead, _: &Settings) -> Result<Self, Error> {
        Ok(PhantomData)
    }

    fn write(&self, _: &mut dyn BitWrite, _: &Settings) -> Result<(), Error> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_read_phantom_data() {
        assert_eq!(
            PhantomData::<u8>::from_bytes(&[], &Settings::default()).unwrap(),
            PhantomData
        )
    }

    #[test]
    fn can_write_phantom_data() {
        assert_eq!(
            PhantomData::<u8>.bytes(&Settings::default()).unwrap(),
            &[]
        )
    }
}