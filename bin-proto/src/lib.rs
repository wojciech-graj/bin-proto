//! Simple bit-level protocol definitions in Rust.
//!
//! # Example
//!
//! ```
//! # use bin_proto::Protocol;
//! #[derive(Debug, Protocol, PartialEq)]
//! #[protocol(discriminant = "integer")]
//! #[protocol(bits = 4)]
//! #[repr(u8)]
//! enum Version {
//!     V4 = 4,
//! }
//!
//! #[derive(Debug, Protocol, PartialEq)]
//! struct Flags {
//!     #[protocol(bits = 1)]
//!     reserved: bool,
//!     #[protocol(bits = 1)]
//!     dont_fragment: bool,
//!     #[protocol(bits = 1)]
//!     more_fragments: bool,
//! }
//!
//! #[derive(Debug, Protocol, PartialEq)]
//! struct IPv4 {
//!     version: Version,
//!     #[protocol(bits = 4)]
//!     internet_header_length: u8,
//!     #[protocol(bits = 6)]
//!     differentiated_services_code_point: u8,
//!     #[protocol(bits = 2)]
//!     explicit_congestion_notification: u8,
//!     total_length: u16,
//!     identification: u16,
//!     flags: Flags,
//!     #[protocol(bits = 13)]
//!     fragment_offset: u16,
//!     time_to_live: u8,
//!     protocol: u8,
//!     header_checksum: u16,
//!     source_address: [u8; 4],
//!     destination_address: [u8; 4],
//! }
//!
//! assert_eq!(
//!     IPv4::from_raw_bytes(&[
//!             0b0100_0000 // Version: 4
//!             |    0b0101, // Header Length: 5,
//!             0x00, // Differentiated Services Codepoint: 0, Explicit Congestion Notification: 0
//!             0x05, 0x94, // Total Length: 1428
//!             0x83, 0xf6, // Identification: 0x83f6
//!             0b0100_0000 // Flags: Don't Fragment
//!             |  0b0_0000, 0x00, // Fragment Offset: 0
//!             0x40, // Time to Live: 64
//!             0x01, // Protocol: 1
//!             0xeb, 0x6e, // Header Checksum: 0xeb6e
//!             0x02, 0x01, 0x01, 0x01, // Source Address: 2.1.1.1
//!             0x02, 0x01, 0x01, 0x02, // Destination Address: 2.1.1.2
//!         ], &bin_proto::Settings::default()).unwrap(),
//!     IPv4 {
//!         version: Version::V4,
//!         internet_header_length: 5,
//!         differentiated_services_code_point: 0,
//!         explicit_congestion_notification: 0,
//!         total_length: 1428,
//!         identification: 0x83f6,
//!         flags: Flags {
//!             reserved: false,
//!             dont_fragment: true,
//!             more_fragments: false,
//!         },
//!         fragment_offset: 0x0,
//!         time_to_live: 64,
//!         protocol: 1,
//!         header_checksum: 0xeb6e,
//!         source_address: [2, 1, 1, 1],
//!         destination_address: [2, 1, 1, 2],
//!     }
//! );
//! ```

pub use self::bit_field::BitField;
pub use self::bit_read::BitRead;
pub use self::bit_write::BitWrite;
pub use self::enum_ty::Enum;
pub use self::error::{Error, Result};
#[doc(inline)]
pub use self::externally_length_prefixed::ExternallyLengthPrefixed;
pub use self::flexible_array_member::FlexibleArrayMember;
pub use self::parcel::Protocol;
pub use self::settings::*;

/// Derive `Protocol` trait.
///
/// # Attributes
///
/// ## `#[protocol(discriminant = <kind>)]`
/// - Applies to: `enum` with `#[derive(Protocol)]`.
/// - `<kind>`: `"integer"`, `"string"`
///
/// Specify is enum variant should be determined by a string or interger
/// representation of its discriminant. The width of the integer discriminant
/// can be changed with e.g. `#[repr(u8)]`.
///
/// ```
/// # use bin_proto::Protocol;
/// #[derive(Protocol)]
/// #[protocol(discriminant = "integer")]
/// #[repr(u8)]
/// enum Example {
///     Variant1 = 1,
///     Variant5 = 5,
/// }
/// ```
///
/// ## `#[protocol(bits = <width>)]`
/// - Applies to: `impl BitField`, `enum` with integer discriminant
///
/// Determine width of field in bits.
///
/// **WARNING**: Bitfields disregard Settings and instead have the same
/// endianness as the underlying `BitRead` / `BitWrite` instance. If you're
/// using bitfields, you almost always want a big endian stream.
///
/// ```
/// # use bin_proto::Protocol;
/// #[derive(Protocol)]
/// struct Nibble(#[protocol(bits = 4)] u8);
/// ```
///
/// ## `#[protocol(flexible_array_member)]`
/// - Applies to: `impl FlexibleArrayMember`
///
/// Variable-length field is final field in container, hence lacks a length
/// prefix and should be read until eof.
///
/// ```
/// # use bin_proto::Protocol;
/// #[derive(Protocol)]
/// struct ReadToEnd(#[protocol(flexible_array_member)] Vec<u8>);
/// ```
///
/// ## `#[protocol(length_prefix(<kind>(<field>)))]`
/// - Applies to: `impl ExternallyLengthPrefixed`
/// - `<kind>`: `bytes`, `elements`
/// - `<field>`: field in parent container storing length prefix
///
/// Specify variable holding length of variable-length field, instead
/// of prepending it by a `u32` holding the length.
///
/// ```
/// # use bin_proto::Protocol;
/// #[derive(Protocol)]
/// pub struct WithElementsLength {
///     pub count: u32,
///     pub foo: bool,
///     #[protocol(length_prefix(elements(count)))]
///     pub data: Vec<u32>,
/// }
/// ```
#[cfg(feature = "derive")]
pub use bin_proto_derive::Protocol;

mod bit_field;
mod bit_read;
mod bit_write;
pub mod externally_length_prefixed;
mod flexible_array_member;
mod settings;
#[macro_use]
pub mod types;

mod enum_ty;
mod error;
mod parcel;
mod util;

#[cfg(feature = "uuid")]
extern crate uuid;
