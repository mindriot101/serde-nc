extern crate serde;

#[cfg(test)]
#[macro_use]
extern crate serde_derive;

mod error;
mod ser;

pub use error::{Error, Result};
pub use ser::{to_string, Serializer};
