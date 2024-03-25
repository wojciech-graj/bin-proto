use std::{error, fmt};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
/// Copy of [TryFromIntError](https://doc.rust-lang.org/std/num/struct.TryFromIntError.html)
/// that works in stable rust
pub struct TryFromIntError {}

impl TryFromIntError {
    fn description(&self) -> &str {
        "out of range integral type conversion attempted"
    }
}

impl fmt::Display for TryFromIntError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.description().fmt(fmt)
    }
}

impl error::Error for TryFromIntError {
    fn description(&self) -> &str {
        self.description()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
/// Copy of [CharTryFromError](https://doc.rust-lang.org/std/char/struct.CharTryFromError.html)
/// that works in stable rust
pub struct CharTryFromError {}

impl CharTryFromError {
    fn description(&self) -> &str {
        "converted integer out of range for `char`"
    }
}

impl fmt::Display for CharTryFromError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.description().fmt(f)
    }
}

impl error::Error for CharTryFromError {
    fn description(&self) -> &str {
        self.description()
    }
}

error_chain! {
    types {
        Error, ErrorKind, ResultExt;
    }

    foreign_links {
        Io(std::io::Error);
        FromUtf8(std::string::FromUtf8Error);
        FromNulError(std::ffi::NulError);
        TryFromIntError(TryFromIntError);
        CharTryFromError(CharTryFromError);

        UuidParseError(::uuid::Error) #[cfg(feature = "uuid")];
    }

    errors {
        UnknownEnumDiscriminant(type_name: &'static str, discriminant: String) {
            description("received unknown enum discriminant")
            display("received unknown enum discriminant '{}' for type '{}'", discriminant, type_name)
        }

        NonZeroPad {
            description("nonzero pad")
            display("nonzero pad")
        }
    }
}
