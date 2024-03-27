use core::any::Any;

use crate::{BitRead, BitWrite, Error, Settings};

/// A trait for variable-width bit-level co/dec.
///
/// **WARNING**: This trait can and often will ignore the endianness settings.
pub trait BitField: Sized {
    fn read(
        read: &mut dyn BitRead,
        settings: &Settings,
        ctx: &mut dyn Any,
        bits: u32,
    ) -> Result<Self, Error>;

    fn write(
        &self,
        write: &mut dyn BitWrite,
        settings: &Settings,
        ctx: &mut dyn Any,
        bits: u32,
    ) -> Result<(), Error>;
}

/// ```compile_fail
/// #[derive(bin_proto::Protocol)]
/// #[protocol(discriminant_type = "u8")]
/// #[protocol(bits = 1)]
/// enum WontFit {
///     Variant = 2,
/// }
/// ```
#[cfg(all(feature = "derive", doctest))]
#[allow(unused)]
fn compile_fail_if_discriminant_wont_fit() {}
