use std::collections::HashMap;

pub type FieldIndex = usize;

/// Hints given when reading parcels.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Hints {
    pub current_field_index: FieldIndex,
    /// The fields for which a length prefix
    /// was already present earlier in the layout.
    pub known_field_lengths: HashMap<FieldIndex, FieldLength>,
}

/// Information about the length of a field.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct FieldLength {
    pub length: usize,
    pub kind: LengthPrefixKind,
}

/// Specifies what kind of data the length prefix captures.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum LengthPrefixKind {
    /// The length prefix stores the total number of bytes making up another field.
    Bytes,
    /// The length prefix stores the total number of elements inside another field.
    Elements,
}

impl Hints {
    /// Gets the length of the field currently being
    /// read, if known.
    pub fn current_field_length(&self) -> Option<FieldLength> {
        self.known_field_lengths
            .get(&self.current_field_index)
            .cloned()
    }
}

/// Helpers for the `protocol-derive` crate.
mod protocol_derive_helpers {
    use super::*;

    impl Hints {
        // Updates the hints to indicate a field was just read.
        #[doc(hidden)]
        pub fn next_field(&mut self) {
            self.current_field_index += 1;
        }

        #[doc(hidden)]
        pub fn new_nested(&self) -> Self {
            Self {
                current_field_index: 0,
                ..Default::default()
            }
        }

        // Sets the length of a variable-sized field by its 0-based index.
        #[doc(hidden)]
        pub fn set_field_length(
            &mut self,
            field_index: FieldIndex,
            length: usize,
            kind: LengthPrefixKind,
        ) {
            self.known_field_lengths
                .insert(field_index, FieldLength { kind, length });
        }
    }
}
