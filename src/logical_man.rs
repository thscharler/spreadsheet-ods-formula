use crate::{Any, FnAny2, FnAny3, Logical};

/// Return one of two values, depending on a condition.
#[inline]
pub fn if_then_else<A: Logical, B: Any, C: Any>(
    condition: A,
    if_true: B,
    if_false: C,
) -> FnAny3<A, B, C> {
    FnAny3("IF", condition, if_true, if_false)
}

/// Return one of two values, depending on a condition.
#[inline]
pub fn if_then<A: Logical, B: Any>(condition: A, if_true: B) -> FnAny2<A, B> {
    FnAny2("IF", condition, if_true)
}

/// Return one of two values, depending on a condition.
#[inline]
pub fn if_else<A: Logical, B: Any>(condition: A, if_false: B) -> FnAny3<A, (), B> {
    FnAny3("IF", condition, (), if_false)
}
