//!
//! Rounding functions.
//!

mod generated;
pub use generated::*;

use crate::Any;
use std::fmt::Write;

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
