//!
//! Comparison operators as functions.
//!

use crate::{Any, OpLogical};

/// equal
pub fn eq<'a, A: Any, B: Any>(a: A, b: B) -> OpLogical<A, B> {
    OpLogical(a, "=", b)
}

/// inequal
pub fn ne<'a, A: Any, B: Any>(a: A, b: B) -> OpLogical<A, B> {
    OpLogical(a, "<>", b)
}

/// less than
pub fn lt<'a, A: Any, B: Any>(a: A, b: B) -> OpLogical<A, B> {
    OpLogical(a, "<", b)
}

/// less than or equal
pub fn le<'a, A: Any, B: Any>(a: A, b: B) -> OpLogical<A, B> {
    OpLogical(a, "<=", b)
}

/// greater than
pub fn gt<'a, A: Any, B: Any>(a: A, b: B) -> OpLogical<A, B> {
    OpLogical(a, ">", b)
}

/// greater than or equal
pub fn ge<'a, A: Any, B: Any>(a: A, b: B) -> OpLogical<A, B> {
    OpLogical(a, ">=", b)
}
