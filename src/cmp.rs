//!
//! Comparison operators as functions.
//!

use crate::{Any, OpLogical};

/// equal
#[inline]
pub fn eq<A: Any, B: Any>(a: A, b: B) -> OpLogical<A, B> {
    OpLogical(a, "=", b)
}

/// inequal
#[inline]
pub fn ne<A: Any, B: Any>(a: A, b: B) -> OpLogical<A, B> {
    OpLogical(a, "<>", b)
}

/// less than
#[inline]
pub fn lt<A: Any, B: Any>(a: A, b: B) -> OpLogical<A, B> {
    OpLogical(a, "<", b)
}

/// less than or equal
#[inline]
pub fn le<A: Any, B: Any>(a: A, b: B) -> OpLogical<A, B> {
    OpLogical(a, "<=", b)
}

/// greater than
#[inline]
pub fn gt<A: Any, B: Any>(a: A, b: B) -> OpLogical<A, B> {
    OpLogical(a, ">", b)
}

/// greater than or equal
#[inline]
pub fn ge<A: Any, B: Any>(a: A, b: B) -> OpLogical<A, B> {
    OpLogical(a, ">=", b)
}
