pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    #[error(transparent)]
    IO(#[from] std::io::Error),
    #[error(transparent)]
    FromUtf8(#[from] std::string::FromUtf8Error),
    #[error(transparent)]
    FromNulError(#[from] std::ffi::NulError),
    #[error(transparent)]
    TryFromIntError(#[from] std::num::TryFromIntError),
    #[error("Unknown enum discriminant: '{0}'")]
    UnknownEnumDiscriminant(String),
    #[error("Failed to convert tag")]
    TagConvert,
    #[error(transparent)]
    Other(Box<dyn std::error::Error + Send + Sync>),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused)]
    trait IsSizedSendSync: Sized + Send + Sync {}

    impl IsSizedSendSync for Error {}
}
