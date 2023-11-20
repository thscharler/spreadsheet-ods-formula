use crate::*;
#[allow(unused_imports)]
use crate::complex::*;

#[inline]
pub fn complex<A: Number, B: Number>(real: A, imaginary: B) -> FnNumber2<A, B> {
    FnNumber2("COMPLEX", real, imaginary)
}

#[inline]
pub fn complex_<A: Number, B: Number, C: Text>(real: A, imaginary: B, suffix: C) -> FnNumber3<A, B, C> {
    FnNumber3("COMPLEX", real, imaginary, suffix)
}

#[inline]
pub fn imabs<A: Number>(x: A) -> FnNumber1<A> {
    FnNumber1("IMABS", x)
}

#[inline]
pub fn imaginary<A: Number>(x: A) -> FnNumber1<A> {
    FnNumber1("IMAGINARY", x)
}

#[inline]
pub fn imargument<A: Number>(x: A) -> FnNumber1<A> {
    FnNumber1("IMARGUMENT", x)
}

#[inline]
pub fn imconjugate<A: Number>(x: A) -> FnNumber1<A> {
    FnNumber1("IMCONJUGATE", x)
}

#[inline]
pub fn imcos<A: Number>(x: A) -> FnNumber1<A> {
    FnNumber1("IMCOS", x)
}

#[inline]
pub fn imcosh<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("IMCOSH", n)
}

#[inline]
pub fn imcot<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("IMCOT", n)
}

#[inline]
pub fn imcsc<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("IMCSC", n)
}

#[inline]
pub fn imcsch<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("IMCSCH", n)
}

#[inline]
pub fn imdiv<A: Number, B: Number>(x: A, y: B) -> FnNumber2<A, B> {
    FnNumber2("IMDIV", x, y)
}

#[inline]
pub fn imexp<A: Number>(x: A) -> FnNumber1<A> {
    FnNumber1("IMEXP", x)
}

#[inline]
pub fn imln<A: Number>(x: A) -> FnNumber1<A> {
    FnNumber1("IMLN", x)
}

#[inline]
pub fn imlog10<A: Number>(x: A) -> FnNumber1<A> {
    FnNumber1("IMLOG10", x)
}

#[inline]
pub fn imlog2<A: Number>(x: A) -> FnNumber1<A> {
    FnNumber1("IMLOG2", x)
}

#[inline]
pub fn impower<A: Number, B: Number>(x: A, y: B) -> FnNumber2<A, B> {
    FnNumber2("IMPOWER", x, y)
}

#[inline]
pub fn improduct<A: Sequence>(n: A) -> FnNumber1<A> {
    FnNumber1("IMPRODUCT", n)
}

#[inline]
pub fn imreal<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("IMREAL", n)
}

#[inline]
pub fn imsin<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("IMSIN", n)
}

#[inline]
pub fn imsinh<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("IMSINH", n)
}

#[inline]
pub fn imsec<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("IMSEC", n)
}

#[inline]
pub fn imsech<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("IMSECH", n)
}

#[inline]
pub fn imsqrt<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("IMSQRT", n)
}

#[inline]
pub fn imsub<A: Number, B: Number>(x: A, y: B) -> FnNumber2<A, B> {
    FnNumber2("IMSUB", x, y)
}

#[inline]
pub fn imsum<A: Sequence>(n: A) -> FnNumber1<A> {
    FnNumber1("IMSUM", n)
}

#[inline]
pub fn imtan<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("IMTAN", n)
}
