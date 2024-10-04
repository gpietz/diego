use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

// *** Public modules ***
pub mod application;
pub mod string_utils;
pub mod main_loop;
pub mod application_context;
pub mod delta_time;
pub mod scene;
pub mod runtime_info;
pub mod runtime_error;
pub mod diego_runtime;

// *** Internal modules ***
mod diego_runtime_internal;

/// `ConversionError` is a generic enum used to handle errors during conversion processes.
/// # Parameters
/// * `T` - The type of the invalid value that caused the conversion error.
#[derive(Debug, Clone)]
pub enum ConversionError<T> {
    InvalidValue(T),
    UnsupportedType(T),
}

impl<T: Display> Display for ConversionError<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ConversionError::InvalidValue(val) => write!(f, "Invalid value: {}", val),
            ConversionError::UnsupportedType(val) => write!(f, "Unsupported type: {}", val),
        }
    }
}

impl<T: Debug + Display> Error for ConversionError<T> {}

/// `SizeOf` is a trait used to determine the size of an object in bytes.
/// # Methods
/// * `size(&self) -> usize` - Returns the size of the object in bytes as an unsigned integer.
pub trait SizeOf {
    /// Returns the size of the object in bytes as an unsigned integer.
    fn size(&self) -> usize;
}
