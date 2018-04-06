use core::result;

#[derive(Copy, Clone, Hash, Debug, PartialEq, Eq)]
/// Indicates that something bad happened.
pub struct Error;

/// A specialized Result type for video encoding operations.
pub type Result<T> = result::Result<T, Error>;
