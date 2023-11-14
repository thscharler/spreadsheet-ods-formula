use crate::*;
#[allow(unused_imports)]
use crate::complex::*;

/// Creates a complex number from a given real coefficient and imaginary coefficient.
#[inline]
pub fn complex<A: Number, B: Number>(real: A, imag: B) -> FnNumber2<A, B> {
    FnNumber2("COMPLEX", real, imag)
}

/// Returns the absolute value of a complex number.
#[inline]
pub fn imabs<A: Number>(v: A) -> FnNumber1<A> {
    FnNumber1("IMABS", v)
}

/// Returns the imaginary coefficient of a complex number.
#[inline]
pub fn imaginary<A: Number>(v: A) -> FnNumber1<A> {
    FnNumber1("IMAGINARY", v)
}

/// Returns the complex argument of a complex number
#[inline]
pub fn imargument<A: Number>(v: A) -> FnNumber1<A> {
    FnNumber1("IMARGUMENT", v)
}

/// Returns the complex conjugate of a complex number.
#[inline]
pub fn imconjugate<A: Number>(v: A) -> FnNumber1<A> {
    FnNumber1("IMCONJUGATE", v)
}

/// Returns the cosine of a complex number.
#[inline]
pub fn imcos<A: Number>(v: A) -> FnNumber1<A> {
    FnNumber1("IMCOS", v)
}

/// Returns the hyperbolic cosine of a complex number.
#[inline]
pub fn imcosh<A: Number>(v: A) -> FnNumber1<A> {
    FnNumber1("IMCOSH", v)
}

/// Returns the cotangent of a complex number.
#[inline]
pub fn imcot<A: Number>(v: A) -> FnNumber1<A> {
    FnNumber1("IMCOT", v)
}

/// Returns the cosecant of a complex number
#[inline]
pub fn imcsc<A: Number>(v: A) -> FnNumber1<A> {
    FnNumber1("IMCSC", v)
}

/// Returns the hyperbolic cosecant of a complex number.
#[inline]
pub fn imcsch<A: Number>(v: A) -> FnNumber1<A> {
    FnNumber1("IMCSCH", v)
}

/// Divides the first number by the second.
#[inline]
pub fn imdiv<A: Number, B: Number>(x: A, y: B) -> FnNumber2<A, B> {
    FnNumber2("IMDIV", x, y)
}

/// Returns the exponent of e and a complex number.
#[inline]
pub fn imexp<A: Number>(v: A) -> FnNumber1<A> {
    FnNumber1("IMEXP", v)
}

/// Returns the natural logarithm of a complex number
#[inline]
pub fn imln<A: Number>(v: A) -> FnNumber1<A> {
    FnNumber1("IMLN", v)
}

/// Returns the common logarithm of a comp
#[inline]
pub fn imlog10<A: Number>(v: A) -> FnNumber1<A> {
    FnNumber1("IMLOG10", v)
}

/// Returns the binary logarithm of a complex number.
#[inline]
pub fn imlog2<A: Number>(v: A) -> FnNumber1<A> {
    FnNumber1("IMLOG2", v)
}

/// Returns the complex number X raised to the Yth power.
#[inline]
pub fn impower<A: Number, B: Number>(x: A, y: B) -> FnNumber2<A, B> {
    FnNumber2("IMPOWER", x, y)
}

/// Returns the product of complex numbers.
#[inline]
pub fn improduct<A: Sequence>(seq: A) -> FnNumber1<A> {
    FnNumber1("IMPRODUCT", seq)
}

/// Returns the real coefficient of a complex number.
#[inline]
pub fn imreal<A: Number>(v: A) -> FnNumber1<A> {
    FnNumber1("IMREAL", v)
}

/// Returns the secant of a complex number.
#[inline]
pub fn imsec<A: Number>(v: A) -> FnNumber1<A> {
    FnNumber1("IMSEC", v)
}

/// Returns the hyperbolic secant of a complex number.
#[inline]
pub fn imsech<A: Number>(v: A) -> FnNumber1<A> {
    FnNumber1("IMSECH", v)
}

/// Returns the sine of a complex number.
#[inline]
pub fn imsin<A: Number>(v: A) -> FnNumber1<A> {
    FnNumber1("IMSIN", v)
}

/// Returns the hyperbolic sine of a complex number.
#[inline]
pub fn imsinh<A: Number>(v: A) -> FnNumber1<A> {
    FnNumber1("IMSINH", v)
}

/// Returns the square root of a complex number
#[inline]
pub fn imsqrt<A: Number>(v: A) -> FnNumber1<A> {
    FnNumber1("IMSQRT", v)
}

/// Subtracts the second complex number from the first
#[inline]
pub fn imsub<A: Number, B: Number>(x: A, y: B) -> FnNumber2<A, B> {
    FnNumber2("IMSUB", x, y)
}

/// Sums (add) a set of complex numbers, including all numbers in ranges.
#[inline]
pub fn imsum<A: Sequence>(seq: A) -> FnNumber1<A> {
    FnNumber1("IMSUM", seq)
}

/// Returns the tangent of a complex number
#[inline]
pub fn imtan<A: Number>(v: A) -> FnNumber1<A> {
    FnNumber1("IMTAN", v)
}
