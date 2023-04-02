mod generated;
pub use generated::*;

use crate::Any;
use std::fmt::Write;

pub(self) use crate::date::YearFracMethod;

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
