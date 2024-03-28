pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    IO(#[from] std::io::Error),
    #[error("{0}")]
    FromUtf8(#[from] std::string::FromUtf8Error),
    #[error("{0}")]
    FromNulError(#[from] std::ffi::NulError),
    #[error("{0}")]
    TryFromIntError(#[from] std::num::TryFromIntError),
    #[cfg(feature = "uuid")]
    #[error("{0}")]
    UuidParseError(#[from] uuid::Error),
    #[error("received unknown enum discriminant '{0}'")]
    UnknownEnumDiscriminant(String),
}
