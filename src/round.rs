//!
//! Rounding functions.
//!

pub use crate::generated::round::*;

use crate::Any;
use std::fmt::Write;

/// Parameter for CEILING() and FLOOR().
pub enum RoundingMode {
    AwayFrom0,
    TowardsPlusInf,
}

impl Any for RoundingMode {
    fn formula(&self, buf: &mut String) {
        let _ = write!(
            buf,
            "{}",
            match self {
                RoundingMode::AwayFrom0 => 1,
                RoundingMode::TowardsPlusInf => 0,
            }
        );
    }
}
