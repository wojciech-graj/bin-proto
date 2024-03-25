//! Contains newtypes over the standard library types
//! that support finer-grained serialization settings.

pub use self::numerics::Integer;

mod array;
mod char;
/// Definitions for the `std::collections` module.
mod collections;
mod cstring;
mod marker;
mod numerics;
mod option;
mod range;
/// Definitions for smart pointers in the `std` module.
mod smart_ptr;
mod string;
mod tuple;
#[cfg(feature = "uuid")]
mod uuid;
mod vec;
