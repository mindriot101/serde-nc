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
    fn test_enable_load_monitor() {
        let nc = Instruction::EnableLoadMonitor;
        assert_eq!(to_string(&nc).unwrap(), "M216");
    }

    #[test]
    fn test_set_inch_units() {
        let nc = Instruction::SetInchUnits;
        assert_eq!(to_string(&nc).unwrap(), "G20");
    }
}
