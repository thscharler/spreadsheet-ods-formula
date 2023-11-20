//!
//! Financial functions.
//!

use crate::Any;
use std::fmt::Write;

pub use crate::generated::fin::*;

#[doc(inline)]
pub use crate::date::YearFracMethod;

/// Parameter for many fin functions.
pub enum Frequency {
    Annual,
    Semiannual,
    Quarterly,
    Monthly,
}

impl Any for Frequency {
    fn formula(&self, buf: &mut String) {
        let _ = write!(
            buf,
            "{}",
            match self {
                Frequency::Annual => 1,
                Frequency::Semiannual => 2,
                Frequency::Quarterly => 4,
                Frequency::Monthly => 12,
            }
        );
    }
}

/// Parameter for CUMIPMT()
pub enum MaturityDate {
    ///
    DueAtEnd,
    ///
    DueAtBeginning,
}

impl Any for MaturityDate {
    fn formula(&self, buf: &mut String) {
        buf.push_str(match self {
            MaturityDate::DueAtEnd => "0",
            MaturityDate::DueAtBeginning => "1",
        });
    }
}

/// Parameter for CUMIPMT()
pub enum PayType {
    ///
    DueAtEnd,
    ///
    DueAtBeginning,
}

impl Any for PayType {
    fn formula(&self, buf: &mut String) {
        buf.push_str(match self {
            PayType::DueAtEnd => "0",
            PayType::DueAtBeginning => "1",
        });
    }
}
