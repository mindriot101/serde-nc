extern crate serde;

mod error;
mod ser;

pub use error::{Error, Result};
pub use ser::{to_string, Serializer};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_it_works() {
        assert_eq!(1, 1);
    }
}
