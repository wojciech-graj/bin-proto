//! Different bin_proto formats.

/// Represents a format.
pub trait Format: Clone {
    /// From a string.
    fn from_str(s: &str) -> Result<Self, ()>;
}

/// The enum bin_proto format.
#[derive(Clone, Default, Debug, PartialEq)]
pub enum Enum {
    /// The enum is transmitted by using the 1-based index of the enum variant.
    IntegerDiscriminant,
    /// The enum is transmitted by using the name of the variant.
    #[default]
    StringDiscriminant,
}

impl Format for Enum {
    fn from_str(s: &str) -> Result<Self, ()> {
        match s {
            "integer" => Ok(Enum::IntegerDiscriminant),
            "string" => Ok(Enum::StringDiscriminant),
            _ => Err(()),
        }
    }
}
