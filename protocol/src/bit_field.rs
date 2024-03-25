use crate::{hint, BitRead, BitWrite, Error, Parcel, Settings};

/// ```compile_fail
/// #[derive(protocol::Protocol)]
/// #[protocol(discriminant = "integer")]
/// #[protocol(bits = 1)]
/// #[repr(u8)]
/// enum WontFit {
///     Variant = 2,
/// }
/// ```
pub trait BitField: Parcel {
    fn read_field(
        read: &mut dyn BitRead,
        settings: &Settings,
        hints: &mut hint::Hints,
        bits: u32,
    ) -> Result<Self, Error>;

    fn write_field(
        &self,
        write: &mut dyn BitWrite,
        settings: &Settings,
        hints: &mut hint::Hints,
        bits: u32,
    ) -> Result<(), Error>;
}
