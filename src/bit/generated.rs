use crate::*;
#[allow(unused_imports)]
use super::*;

/// Returns bitwise “and” of its parameters
#[inline]
pub fn bitand<A: Number, B: Number>(x: A, y: B) -> FnNumber2<A, B> {
    FnNumber2("BITAND", x, y)
}

/// Returns left shift of value X by N bits (“<<”)
#[inline]
pub fn bitlshift<A: Number, B: Number>(x: A, n: B) -> FnNumber2<A, B> {
    FnNumber2("BITLSHIFT", x, n)
}

/// Returns bitwise “or” of its parameters
#[inline]
pub fn bitor<A: Number, B: Number>(x: A, y: B) -> FnNumber2<A, B> {
    FnNumber2("BITOR", x, y)
}

/// Returns right shift of value X by N bits (“>>”)
#[inline]
pub fn bitrshift<A: Number, B: Number>(x: A, n: B) -> FnNumber2<A, B> {
    FnNumber2("BITRSHIFT", x, n)
}

/// Returns bitwise “exclusive or” of its parameters
#[inline]
pub fn bitxor<A: Number, B: Number>(x: A, y: B) -> FnNumber2<A, B> {
    FnNumber2("BITXOR", x, y)
}
