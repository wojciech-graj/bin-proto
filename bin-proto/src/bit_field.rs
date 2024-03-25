use crate::{BitRead, BitWrite, Error, Protocol, Settings};

pub trait BitField: Protocol {
    fn read_field(read: &mut dyn BitRead, settings: &Settings, bits: u32) -> Result<Self, Error>;

    fn write_field(
        &self,
        write: &mut dyn BitWrite,
        settings: &Settings,
        bits: u32,
    ) -> Result<(), Error>;
}

/// ```compile_fail
/// #[derive(bin_proto::Protocol)]
/// #[protocol(discriminant = "integer")]
/// #[protocol(bits = 1)]
/// #[repr(u8)]
/// enum WontFit {
///     Variant = 2,
/// }
/// ```
#[cfg(all(feature = "derive", doctest))]
#[allow(unused)]
fn compile_fail_if_discriminant_wont_fit() {}
