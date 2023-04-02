//!
//! Information functions.
//!

mod generated;
pub use generated::*;

use crate::{Any, Criterion, FnNumberVar, Reference};

/// Count the number of cells that meet multiple criteria in multiple ranges.
#[inline]
pub fn countifs<R: Reference + 'static, C: Criterion + 'static, const N: usize>(
    list: [(R, C); N],
) -> FnNumberVar {
    let mut param = Vec::new();

    for (r, c) in list {
        param.push(Box::new(r) as Box<dyn Any>);
        param.push(Box::new(c) as Box<dyn Any>);
    }

    FnNumberVar("COUNTIFS", param)
}
