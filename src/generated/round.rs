use crate::*;
#[allow(unused_imports)]
use crate::round::*;

///  Round a number N up to the nearest multiple of the second parameter, significance.
#[inline]
pub fn ceiling<A: Number, B: Number>(n: A, significance: B, rounding: RoundingMode) -> FnNumber3<A, B, RoundingMode> {
    FnNumber3("CEILING", n, significance, rounding)
}

///  Round a number N down to the nearest multiple of the second parameter,
#[inline]
pub fn floor<A: Number, B: Number>(n: A, significance: B, rounding: RoundingMode) -> FnNumber3<A, B, RoundingMode> {
    FnNumber3("FLOOR", n, significance, rounding)
}

///  Rounds a number down to the nearest integer.
#[inline]
pub fn int<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("INT", n)
}

///  Rounds the number to given multiple.
#[inline]
pub fn mround<A: Number, B: Number>(a: A, b: B) -> FnNumber2<A, B> {
    FnNumber2("MROUND", a, b)
}

///  Rounds the value X to the nearest multiple of the power of 10 specified by Digits.
#[inline]
pub fn round<A: Number, B: Number>(x: A, digits: Option<B>) -> FnNumber2<A, Option<B>> {
    FnNumber2("ROUND", x, digits)
}

///  Rounds the value X towards zero to the number of digits specified by Digits.
#[inline]
pub fn rounddown<A: Number, B: Number>(x: A, digits: Option<B>) -> FnNumber2<A, Option<B>> {
    FnNumber2("ROUNDDOWN", x, digits)
}

///  Rounds the value X away from zero to the number of digits specified by Digits
#[inline]
pub fn roundup<A: Number, B: Number>(x: A, digits: Option<B>) -> FnNumber2<A, Option<B>> {
    FnNumber2("ROUNDUP", x, digits)
}

/// Truncate a number to a specified number of digits.
#[inline]
pub fn trunc<A: Number, B: Number>(a: A, b: B) -> FnNumber2<A, B> {
    FnNumber2("TRUNC", a, b)
}
