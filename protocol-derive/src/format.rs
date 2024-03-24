//! Different protocol formats.

/// Represents a format.
pub trait Format: Clone {
    /// From a string.
    fn from_str(s: &str) -> Result<Self, ()>;
}

/// The enum protocol format.
#[derive(Clone, Default, Debug, PartialEq)]
pub enum Enum {
    /// The enum is transmitted by using the 1-based index of the enum variant.
    IntegerDiscriminator,
    /// The enum is transmitted by using the name of the variant.
    #[default]
    StringDiscriminator,
}

impl Format for Enum {
    fn from_str(s: &str) -> Result<Self, ()> {
        match s {
            "integer" => Ok(Enum::IntegerDiscriminator),
            "string" => Ok(Enum::StringDiscriminator),
            _ => Err(()),
        }
    }
}
