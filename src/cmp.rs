//!
//! Comparison operators as functions.
//!

use crate::{Any, OpLogical};

/// equal
#[inline]
pub fn eq<'a, A: Any, B: Any>(a: A, b: B) -> OpLogical<A, B> {
    OpLogical(a, "=", b)
}

/// inequal
#[inline]
pub fn ne<'a, A: Any, B: Any>(a: A, b: B) -> OpLogical<A, B> {
    OpLogical(a, "<>", b)
}

/// less than
#[inline]
pub fn lt<'a, A: Any, B: Any>(a: A, b: B) -> OpLogical<A, B> {
    OpLogical(a, "<", b)
}

/// less than or equal
#[inline]
pub fn le<'a, A: Any, B: Any>(a: A, b: B) -> OpLogical<A, B> {
    OpLogical(a, "<=", b)
}

/// greater than
#[inline]
pub fn gt<'a, A: Any, B: Any>(a: A, b: B) -> OpLogical<A, B> {
    OpLogical(a, ">", b)
}

/// greater than or equal
#[inline]
pub fn ge<'a, A: Any, B: Any>(a: A, b: B) -> OpLogical<A, B> {
    OpLogical(a, ">=", b)
}
