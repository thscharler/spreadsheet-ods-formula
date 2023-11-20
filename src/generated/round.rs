use crate::*;
#[allow(unused_imports)]
use crate::round::*;

#[inline]
pub fn ceiling<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("CEILING", n)
}

#[inline]
pub fn ceiling_<A: Number, B: Number>(n: A, significance: B) -> FnNumber2<A, B> {
    FnNumber2("CEILING", n, significance)
}

#[inline]
pub fn ceiling__<A: Number, B: Number>(n: A, significance: B, mode: RoundingMode) -> FnNumber3<A, B, RoundingMode> {
    FnNumber3("CEILING", n, significance, mode)
}

#[inline]
pub fn int<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("INT", n)
}

#[inline]
pub fn floor<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("FLOOR", n)
}

#[inline]
pub fn floor_<A: Number, B: Number>(n: A, significance: B) -> FnNumber2<A, B> {
    FnNumber2("FLOOR", n, significance)
}

#[inline]
pub fn floor__<A: Number, B: Number>(n: A, significance: B, mode: RoundingMode) -> FnNumber3<A, B, RoundingMode> {
    FnNumber3("FLOOR", n, significance, mode)
}

#[inline]
pub fn mround<A: Number, B: Number>(a: A, b: B) -> FnNumber2<A, B> {
    FnNumber2("MROUND", a, b)
}

#[inline]
pub fn round<A: Number>(x: A) -> FnNumber1<A> {
    FnNumber1("ROUND", x)
}

#[inline]
pub fn round_<A: Number, B: Number>(x: A, digits: B) -> FnNumber2<A, B> {
    FnNumber2("ROUND", x, digits)
}

#[inline]
pub fn rounddown<A: Number>(x: A) -> FnNumber1<A> {
    FnNumber1("ROUNDDOWN", x)
}

#[inline]
pub fn rounddown_<A: Number, B: Number>(x: A, digits: B) -> FnNumber2<A, B> {
    FnNumber2("ROUNDDOWN", x, digits)
}

#[inline]
pub fn roundup<A: Number>(x: A) -> FnNumber1<A> {
    FnNumber1("ROUNDUP", x)
}

#[inline]
pub fn roundup_<A: Number, B: Number>(x: A, digits: B) -> FnNumber2<A, B> {
    FnNumber2("ROUNDUP", x, digits)
}

#[inline]
pub fn trunc<A: Number, B: Number>(a: A, b: B) -> FnNumber2<A, B> {
    FnNumber2("TRUNC", a, b)
}
