//!
//! Text functions.
//!

use crate::{Any, FnTextVar, Text};

pub use crate::generated::text::*;

/// Count the number of cells that meet multiple criteria in multiple ranges.
#[inline]
pub fn concatenate_array<R: Text + 'static, const N: usize>(list: [R; N]) -> FnTextVar {
    let mut param: Vec<Box<dyn Any>> = Vec::new();

    for r in list {
        param.push(Box::new(r));
    }

    FnTextVar("CONCATENATE", param)
}
