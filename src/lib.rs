extern crate serde;
#[macro_use]
extern crate serde_derive;

mod error;
mod ser;
mod nc;

pub use nc::*;
pub use error::{Error, Result};
pub use ser::{to_string, Serializer};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_it_works() {
        let nc = Instruction::LinearInterp;
        assert_eq!(to_string(&nc).unwrap(), "Foobar");
    }
}
