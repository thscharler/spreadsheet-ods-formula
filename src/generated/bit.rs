use crate::*;
#[allow(unused_imports)]
use crate::bit::*;

#[inline]
pub fn bitand<A: Number, B: Number>(x: A, y: B) -> FnNumber2<A, B> {
    FnNumber2("BITAND", x, y)
}

#[inline]
pub fn bitlshift<A: Number, B: Number>(x: A, n: B) -> FnNumber2<A, B> {
    FnNumber2("BITLSHIFT", x, n)
}

#[inline]
pub fn bitor<A: Number, B: Number>(x: A, y: B) -> FnNumber2<A, B> {
    FnNumber2("BITOR", x, y)
}

#[inline]
pub fn bitrshift<A: Number, B: Number>(x: A, n: B) -> FnNumber2<A, B> {
    FnNumber2("BITRSHIFT", x, n)
}

#[inline]
pub fn bitxor<A: Number, B: Number>(x: A, y: B) -> FnNumber2<A, B> {
    FnNumber2("BITXOR", x, y)
}
