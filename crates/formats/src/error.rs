use std::fmt::Display;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    UnknownVersion(String),
    IoError(std::io::Error),
    EnumUnparseable(String),
    InvalidStructure(String),
    InvalidString(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value)
    }
}

impl<E: num_enum::TryFromPrimitive> From<num_enum::TryFromPrimitiveError<E>> for Error {
    fn from(value: num_enum::TryFromPrimitiveError<E>) -> Self {
        Self::EnumUnparseable(format!("{}", value))
    }
}
