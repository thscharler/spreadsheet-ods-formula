#[allow(unused_imports)]
use super::*;
use crate::*;

/// Compute logic AND of all parameters.
#[inline]
pub fn and<A: Sequence>(seq: A) -> FnLogical1<A> {
    FnLogical1("AND", seq)
}

/// Return X unless it is an Error, in which case return an alternative value.
#[inline]
pub fn iferror<A: Any, B: Any>(x: A, alt: B) -> FnAny2<A, B> {
    FnAny2("IFERROR", x, alt)
}

/// Compute logic NOT.
#[inline]
pub fn not<A: Logical>(v: A) -> FnLogical1<A> {
    FnLogical1("NOT", v)
}

/// Compute logic OR of all parameters.
#[inline]
pub fn or<A: Sequence>(v: A) -> FnLogical1<A> {
    FnLogical1("OR", v)
}

/// Compute logic XOR of all parameters.
#[inline]
pub fn xor<A: Sequence>(v: A) -> FnLogical1<A> {
    FnLogical1("XOR", v)
}

/// Return X unless it is #N/A, in which case return an alternative value.
#[inline]
pub fn ifna<A: Any, B: Any>(x: A, alt: B) -> FnAny2<A, B> {
    FnAny2("IFNA", x, alt)
}
