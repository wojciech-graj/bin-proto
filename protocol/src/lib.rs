//! Simple packet-based protocol definitions in Rust.
//!
//! * The [Parcel] trait defines any type that can be serialized
//!   to a connection.
//!
//! # Examples
//!
//! ## Get the raw bytes representing a parcel.
//!
//! ```
//! #[derive(protocol::Protocol, Debug, PartialEq)]
//! pub struct Health(pub u8);
//!
//! #[derive(protocol::Protocol, Debug, PartialEq)]
//! pub struct Player {
//!     pub name: String,
//!     pub health: Health,
//!     pub position: (i16, i16, i16),
//!     pub velocity: (i16, i16, i16),
//! }
//!
//! use protocol::Parcel;
//!
//! assert_eq!(vec![
//!     0, 0, 0, 3, // "Bob" string length prefix
//!     b'B', b'o', b'b', // The string "Bob".
//!     100, // The health byte.
//!     0, 10, 0, 81, 255, 151, // 2-byte x-position, y-position, and z-position.
//!     0, 0, 0, 0, 0, 0, // 2-byte x-velocity, y-velocity, and z-velocity.
//! ], Player {
//!     name: "Bob".to_owned(),
//!     health: Health(100),
//!     position: (10, 81, -105),
//!     velocity: (0, 0, 0),
//! }.raw_bytes(&protocol::Settings::default()).unwrap());
//! ```
//!
//! ## Enums
//!
//! Every enum has a value for each variant used to distinguish
//! between each variant. This can be as simple as a 32-bit integer
//! representing the variant's index. This is called the discriminant.
//!
//! ### String discriminants
//!
//! ```
//! // This enum, like all enums, defaults to using String discriminants.
//! #[derive(protocol::Protocol, Debug, PartialEq)]
//! enum Foo { A, B, C }
//!
//! // This enum explicitly specifies a string discriminant.
//! //
//! // This is the default anyway and thus it is identical in layout
//! // to the previous enum.
//! #[derive(protocol::Protocol, Debug, PartialEq)]
//! #[protocol(discriminant = "string")]
//! enum Bar { A, B, C }
//! ```
//!
//! By default, enums have `String` discriminants. This means that
//! when serializing enum values to and from bytes, unless otherwise
//! specified the variant's full name will be transmitted as a string.
//!
//! This allows variants to be added, modified, or reordered in
//! a backwards-compatible way.
//!
//! ### Integer discriminants
//!
//! ```
//! // An enum with integer discriminants.
//! #[derive(protocol::Protocol, Debug, PartialEq)]
//! #[protocol(discriminant = "integer")]
//! enum Baz {
//!     Fizz = 0, // override the default first discriminant of 1
//!     Buzz, // defaults to `prior discriminant + 1`, therefore has a discriminant of 1.
//!     Bing, // defaults to `prior discriminant + 1`, therefore has a discriminant of 2.
//!     Boo = 1234,
//!     Beez, // defaults to `prior discriminant + 1`, therefore has a discriminant of 1235.
//! }
//!
//! use protocol::{Enum, Parcel};
//!
//! // By default, integer discriminants are 32-bits.
//! assert_eq!([0u8, 0, 0, 2], &Baz::Bing.discriminant().raw_bytes(&protocol::Settings::default()).unwrap()[..]);
//!
//! assert_eq!(0, Baz::Fizz.discriminant());
//! assert_eq!(1, Baz::Buzz.discriminant());
//! assert_eq!(2, Baz::Bing.discriminant());
//! assert_eq!(1234, Baz::Boo.discriminant());
//! assert_eq!(1235, Baz::Beez.discriminant());
//! ```
//!
//! It is possible to set the underlying integer type via the `#[repr(<type>)]` attribute.
//!
//! ```
//! #[derive(protocol::Protocol, Debug, PartialEq)]
//! #[protocol(discriminant = "integer")]
//! #[repr(u8)] // Force discriminants to be 8-bit.
//! pub enum Hello {
//!     World(String), SolarSystem(String), LocalGroup(String),
//!     Universe(String), Everything(String),
//! }
//!
//! use protocol::{Enum, Parcel};
//!
//! assert_eq!([2, // 1-byte discriminant
//!             0, 0, 0, 3, // string length
//!             b'f', b'o', b'o', // the string
//!             ], &Hello::SolarSystem("foo".to_owned()).raw_bytes(&protocol::Settings::default()).unwrap()[..]);
//! ```
//!
//! Discriminants can be overriden on a per-variant basis via
//! the `#[protocol(discriminant(<value>))]` attribute.
//!
//! In the case of trivial enums with no fields,
//! the `Variant = <discriminant>` syntax may be used.
//!
//! ```
//! #[derive(protocol::Protocol, Debug, PartialEq)]
//! #[protocol(discriminant = "integer")]
//! #[repr(u8)] // Force discriminants to be 8-bit.
//! enum Numbered {
//!     A = 1, B = 2, C = 111, D = 67,
//! }
//!
//! #[derive(protocol::Protocol, Debug, PartialEq)]
//! #[protocol(discriminant = "integer")]
//! #[repr(u8)] // Force discriminants to be 8-bit.
//! pub enum Hello {
//!     World(String), SolarSystem(String), LocalGroup(String),
//!     #[protocol(discriminant(100))]
//!     Universe(String),
//!     Everything(String),
//! }
//!
//! use protocol::{Enum, Parcel};
//!
//! assert_eq!(111, Numbered::C.discriminant());
//! assert_eq!(67, Numbered::D.discriminant());
//!
//! assert_eq!(1, Hello::World("foo".to_owned()).discriminant());
//! assert_eq!(100, Hello::Universe("foo".to_owned()).discriminant());
//! assert_eq!(101, Hello::Everything("foo".to_owned()).discriminant());
//!
//! ```

pub use self::bit_field::BitField;
pub use self::bit_read::BitRead;
pub use self::bit_write::BitWrite;
pub use self::enum_ty::Enum;
pub use self::errors::{Error, Result};
pub use self::flexible_array_member::FlexibleArrayMember;
pub use self::parcel::Parcel;
pub use self::settings::*;
pub use self::with_length_prefix::WithLengthPrefix;

/// Custom derive to implement `Parcel` for a type that contains other `Parcel` types.
///
/// Example:
///
/// ```
/// #[derive(protocol::Protocol)]
/// struct Hello {
///   foo: u8,
///   bar: String,
/// }
/// ```
/// # Attributes
///
/// ## `#[protocol(discriminant = <kind>)]` "integer" "string"
/// - Applies to: `enum`.
/// - `<kind>`: `"integer"`, `"string"`
///
/// ## `#[protocol(bits = <width>)]`
/// - Applies to: `enum` with integer discriminant, `impl BitField`
///
/// ## `#[protocol(flexible_array_member)]`
/// - Applies to: `impl FlexibleArrayMember`
///
/// ## `#[protocol(length_prefix(<kind>(<length prefix field name>)))]`
/// - Applies to: `Vec`, `String`
/// - `<kind>`: `bytes`, `elements`
///
/// This attribute allows variable-sized fields to have their sizes
/// specified by an arbitrary integer field in the same struct or enum.
///
/// Without this attribute, variable-sized fields default to having 32-bit
/// unsigned integer length prefixes prefixed immediately before the field
/// itself.
///
/// ### Length prefix kinds
///
/// #### `bytes`
///
/// When the length prefix type is `bytes`, the length prefix
/// represents the total number of bytes that make up a field.
///
/// ```
/// #[derive(protocol::Protocol)]
/// pub struct Foo {
///     /// This field specifes the length of the last field `reason`.
///     ///
///     /// When values of this type are read, the size of `reason` is
///     /// assumed to be `reason_length` bytes.
///     pub reason_length: u16,
///     pub other_stuff_inbetween: [u16; 16],
///     pub thingy: bool,
///     /// This field
///     #[protocol(length_prefix(bytes(reason_length)))]
///     pub reason: String,
/// }
/// ```
///
/// #### `elements`
///
/// When the length prefix type is 'elements', the length prefix
/// represents the number of elements in a collection or list.
///
/// ```
/// #[derive(protocol::Protocol)]
/// pub struct Bar {
///     /// This field specifes the number of elements in 'data'.
///     pub reason_length: u16,
///     pub other_stuff_inbetween: [u16; 16],
///     pub thingy: bool,
///     /// This field
///     #[protocol(length_prefix(elements(reason_length)))]
///     pub reason: Vec<(u32, u32)>,
/// }
/// ```
///
/// # Notes
///
/// This attribute can only be used with named fields. This means structs like
/// `struct Hello(u32)` cannot be supported. This is because the length prefix
/// field must be specified by a name, and therefore only items with named fields
/// can ever have length prefixes.
///
/// ## Length prefixes placed different structs
///
/// It is possible for a field one one struct to specify the length of a field
/// in another struct, so long as both structs are fields within a parent struct
/// and the struct defining the length appears earlier than the one whose length
/// is being described.
///
/// In this case, the length prefix field must be double quoted.
///
/// `#[protocol(length_prefix(bytes("sibling_field.nested_field.value")))]`
///
/// Example:
///
/// ```
/// #[derive(protocol::Protocol)]
/// struct Packet {
///     /// The length of the adjacent 'reason' field is nested under this field.
///     pub packet_header: PacketHeader,
///     /// The length of this field is specified by the packet header.
///     #[protocol(length_prefix(bytes("packet_header.reason_length")))]
///     pub reason: String,
/// }
///
/// #[derive(protocol::Protocol)]
/// pub struct PacketHeader {
///     pub reason_length: u16,
/// }
/// ```
#[cfg(feature = "derive")]
pub use protocol_derive::Protocol;

mod bit_field;
mod bit_read;
mod bit_write;
mod flexible_array_member;
mod settings;
mod with_length_prefix;
#[macro_use]
pub mod types;

mod enum_ty;
mod errors;
pub mod hint;
mod parcel;
pub mod util;

#[cfg(feature = "uuid")]
extern crate uuid;
