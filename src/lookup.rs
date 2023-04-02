//!
//! Data lookup functions.
//!

mod generated;

pub use generated::*;

use crate::*;

pub enum AddressAbs {
    RowAbsColAbs,
    RowAbsColRel,
    RowRelColAbs,
    RowRelColRel,
}

impl Any for AddressAbs {
    fn formula(&self, buf: &mut String) {
        let _ = write!(
            buf,
            "{}",
            match self {
                AddressAbs::RowAbsColAbs => 1,
                AddressAbs::RowAbsColRel => 2,
                AddressAbs::RowRelColAbs => 3,
                AddressAbs::RowRelColRel => 4,
            }
        );
    }
}

pub enum MatchType {
    MaxInDescendingList,
    ExactMatch,
    MaxInAscendingList,
}

impl Any for MatchType {
    fn formula(&self, buf: &mut String) {
        let _ = write!(
            buf,
            "{}",
            match self {
                MatchType::MaxInDescendingList => -1,
                MatchType::ExactMatch => 0,
                MatchType::MaxInAscendingList => 1,
            }
        );
    }
}

/// Return a value from a data pilot table.
#[inline]
pub fn getpivotdata<A: Text, B: Reference>(datafield: A, table: B) -> FnAny2<A, B> {
    FnAny2("GETPIVOTDATA", datafield, table)
}

/// Return a value from a data pilot table.
#[inline]
pub fn getpivotdata_fields<
    A: Text + 'static,
    B: Reference + 'static,
    F: Text + 'static,
    S: Scalar + 'static,
    const N: usize,
>(
    datafield: A,
    table: B,
    fields: [(F, S); N],
) -> FnAnyVar {
    let mut param = Vec::new();

    param.push(Box::new(datafield) as Box<dyn Any>);
    param.push(Box::new(table) as Box<dyn Any>);

    for (n, sc) in fields {
        param.push(Box::new(n) as Box<dyn Any>);
        param.push(Box::new(sc) as Box<dyn Any>);
    }

    FnAnyVar("GETPIVOTDATA", param)
}
