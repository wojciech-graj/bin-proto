use crate::{BitRead, BitWrite, Error, Parcel, Settings};

pub trait BitField: Parcel {
    fn read_field(read: &mut dyn BitRead, settings: &Settings, bits: u32) -> Result<Self, Error>;

    fn write_field(
        &self,
        write: &mut dyn BitWrite,
        settings: &Settings,
        bits: u32,
    ) -> Result<(), Error>;
}

/// ```compile_fail
/// #[derive(protocol::Protocol)]
/// #[protocol(discriminant = "integer")]
/// #[protocol(bits = 1)]
/// #[repr(u8)]
/// enum WontFit {
///     Variant = 2,
/// }
/// ```
#[cfg(feature = "derive")]
#[allow(unused)]
#[doc(hidden)]
fn fail_compile_if_discriminant_wont_fit() {}
