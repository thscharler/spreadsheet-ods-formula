use crate::*;
#[allow(unused_imports)]
use super::*;

///  Calculates the average of the absolute deviations of the values in list.
#[inline]
pub fn avedev<A: Sequence>(n: A) -> FnNumber1<A> {
    FnNumber1("AVEDEV", n)
}
