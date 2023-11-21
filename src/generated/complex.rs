//! 
//! Functions for complex numbers.

use crate::*;
#[allow(unused_imports)]
use crate::complex::*;

/// Creates a complex number from a given real coefficient and imaginary 
/// coefficient.
///
/// [documentfoundation->COMPLEX](https://wiki.documentfoundation.org/Documentation/Calc_Functions/COMPLEX)
///
/// __Syntax__: 
/// ```ods
///     COMPLEX( Real: Number; Imaginary: Number )
/// ```
///
/// __Constraints__:
/// None
///
/// __Semantics__:
/// Constructs a complex number from the given coefficients. The third 
/// parameter Suffix is optional, and should be either “i” or “j”. 
/// Upper case “I” or “J” are not accepted for the suffix parameter.
///
/// __See also__: [crate::of::complex_()], 
#[inline]
pub fn complex<A: Number, B: Number>(real: A, imaginary: B) -> FnNumber2<A, B> {
    FnNumber2("COMPLEX", real, imaginary)
}

/// Creates a complex number from a given real coefficient and imaginary 
/// coefficient.
///
/// [documentfoundation->COMPLEX](https://wiki.documentfoundation.org/Documentation/Calc_Functions/COMPLEX)
///
/// __Syntax__: 
/// ```ods
///     COMPLEX( Real: Number; Imaginary: Number; Suffix: Text )
/// ```
///
/// __Constraints__:
/// None
///
/// __Semantics__:
/// Constructs a complex number from the given coefficients. The third 
/// parameter Suffix is optional, and should be either “i” or “j”. 
/// Upper case “I” or “J” are not accepted for the suffix parameter.
///
/// __See also__: [crate::of::complex()], 
#[inline]
pub fn complex_<A: Number, B: Number, C: Text>(real: A, imaginary: B, suffix: C) -> FnNumber3<A, B, C> {
    FnNumber3("COMPLEX", real, imaginary, suffix)
}

/// Returns the absolute value of a complex number.
///
/// [documentfoundation->IMABS](https://wiki.documentfoundation.org/Documentation/Calc_Functions/IMABS)
///
/// __Syntax__: 
/// ```ods
///     IMABS( X: Complex )
/// ```
///
/// __Constraints__:
/// None
///
/// __Semantics__:
/// If X = a + bi or X = a + bj, the absolute value =
/// ; if X = r(cosφ + isinφ), the absolute value = r.
///
/// __See also__: [crate::of::imargument()], 
#[inline]
pub fn imabs<A: Number>(x: A) -> FnNumber1<A> {
    FnNumber1("IMABS", x)
}

/// Returns the imaginary coefficient of a complex number.
///
/// [documentfoundation->IMAGINARY](https://wiki.documentfoundation.org/Documentation/Calc_Functions/IMAGINARY)
///
/// __Syntax__: 
/// ```ods
///     IMAGINARY( X: Complex )
/// ```
///
/// __Constraints__:
/// None
///
/// __Semantics__:
/// If X = a + bi or X = a + bj, then the imaginary coefficient is b.
///
/// __See also__: [crate::of::imreal()], 
#[inline]
pub fn imaginary<A: Number>(x: A) -> FnNumber1<A> {
    FnNumber1("IMAGINARY", x)
}

/// Returns the complex argument of a complex number.
///
/// [documentfoundation->IMARGUMENT](https://wiki.documentfoundation.org/Documentation/Calc_Functions/IMARGUMENT)
///
/// __Syntax__: 
/// ```ods
///     IMARGUMENT( X: Complex )
/// ```
///
/// __Constraints__:
/// None
///
/// __Semantics__:
/// If X = a + bi = r(cosφ + isinφ), a or b is not 0 and -π < φ ≤ π, 
/// then the complex argument is φ. φ is expressed by radians. If X = 0, then 
/// IMARGUMENT(X) is implementation-defined and either 0 or an error.
///
/// __See also__: [crate::of::imabs()], 
#[inline]
pub fn imargument<A: Number>(x: A) -> FnNumber1<A> {
    FnNumber1("IMARGUMENT", x)
}

/// Returns the complex conjugate of a complex number.
///
/// [documentfoundation->IMCONJUGATE](https://wiki.documentfoundation.org/Documentation/Calc_Functions/IMCONJUGATE)
///
/// __Syntax__: 
/// ```ods
///     IMCONJUGATE( X: Complex )
/// ```
///
/// __Constraints__:
/// None
///
/// __Semantics__:
/// If X = a + bi, then the complex conjugate is a - bi.
///
/// __See also__: 
#[inline]
pub fn imconjugate<A: Number>(x: A) -> FnNumber1<A> {
    FnNumber1("IMCONJUGATE", x)
}

/// Returns the cosine of a complex number.
///
/// [documentfoundation->IMCOS](https://wiki.documentfoundation.org/Documentation/Calc_Functions/IMCOS)
///
/// __Syntax__: 
/// ```ods
///     IMCOS( X: Complex )
/// ```
///
/// __Constraints__:
/// None
///
/// __Semantics__:
/// If X = a + bi, then cos(X) = cos(a)cosh(b) - sin(a)sinh(b)i.
///
/// __See also__: [crate::of::imsin()], 
#[inline]
pub fn imcos<A: Number>(x: A) -> FnNumber1<A> {
    FnNumber1("IMCOS", x)
}

/// Returns the hyperbolic cosine of a complex number.
///
/// [documentfoundation->IMCOSH](https://wiki.documentfoundation.org/Documentation/Calc_Functions/IMCOSH)
///
/// __Syntax__: 
/// ```ods
///     IMCOSH( N: Complex )
/// ```
///
/// __Constraints__:
/// None
///
/// __Semantics__:
/// If N = a + bi, then cosh(N) = cosh(a)cos(b) + sinh(a)sin(b)i.
///
/// __See also__: 
#[inline]
pub fn imcosh<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("IMCOSH", n)
}

/// Returns the cotangent of a complex number.
///
/// [documentfoundation->IMCOT](https://wiki.documentfoundation.org/Documentation/Calc_Functions/IMCOT)
///
/// __Syntax__: 
/// ```ods
///     IMCOT( N: Complex )
/// ```
///
/// __Constraints__:
/// None
///
/// __Semantics__:
/// Equivalent to the following (except N is computed only once):
/// 
/// IMDIV(IMCOS(N);IMSIN(N))
///
/// __See also__: [crate::of::imcos()], [crate::of::imdiv()], [crate::of::imsin()], [crate::of::imtan()], 
#[inline]
pub fn imcot<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("IMCOT", n)
}

/// Returns the cosecant of a complex number.
///
/// [documentfoundation->IMCSC](https://wiki.documentfoundation.org/Documentation/Calc_Functions/IMCSC)
///
/// __Syntax__: 
/// ```ods
///     IMCSC( N: Complex )
/// ```
///
/// __Constraints__:
/// None
///
/// __Semantics__:
/// Equivalent to the following:
/// 
/// IMDIV(1;IMSIN(N))
///
/// __See also__: [crate::of::imdiv()], [crate::of::imsin()], 
#[inline]
pub fn imcsc<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("IMCSC", n)
}

/// Returns the hyperbolic cosecant of a complex number.
///
/// [documentfoundation->IMCSCH](https://wiki.documentfoundation.org/Documentation/Calc_Functions/IMCSCH)
///
/// __Syntax__: 
/// ```ods
///     IMCSCH( N: Complex )
/// ```
///
/// __Constraints__:
/// None
///
/// __Semantics__:
/// Computes the hyperbolic cosecant. This is equivalent to:
/// 
/// IMDIV(1;IMSINH(N))
///
/// __See also__: [crate::of::imsinh()], [crate::of::csch()], 
#[inline]
pub fn imcsch<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("IMCSCH", n)
}

/// Divides the first number by the second.
///
/// [documentfoundation->IMDIV](https://wiki.documentfoundation.org/Documentation/Calc_Functions/IMDIV)
///
/// __Syntax__: 
/// ```ods
///     IMDIV( X: Complex; Y: Complex )
/// ```
///
/// __Constraints__:
/// None
///
/// __Semantics__:
/// Given X = a + bi and Y = c + di, return the quotient
/// 
/// Division by zero returns an Error.
///
/// __See also__: 
#[inline]
pub fn imdiv<A: Number, B: Number>(x: A, y: B) -> FnNumber2<A, B> {
    FnNumber2("IMDIV", x, y)
}

/// Returns the exponent of e and a complex number.
///
/// [documentfoundation->IMEXP](https://wiki.documentfoundation.org/Documentation/Calc_Functions/IMEXP)
///
/// __Syntax__: 
/// ```ods
///     IMEXP( X: Complex )
/// ```
///
/// __Constraints__:
/// None
///
/// __Semantics__:
/// If X = a + bi, the result is
/// .
///
/// __See also__: [crate::of::imln()], 
#[inline]
pub fn imexp<A: Number>(x: A) -> FnNumber1<A> {
    FnNumber1("IMEXP", x)
}

/// Returns the natural logarithm of a complex number.
///
/// [documentfoundation->IMLN](https://wiki.documentfoundation.org/Documentation/Calc_Functions/IMLN)
///
/// __Syntax__: 
/// ```ods
///     IMLN( X: Complex )
/// ```
///
/// __Constraints__:
/// X ≠ 0
///
/// __Semantics__:
/// COMPLEX(LN(IMABS(X)); IMARGUMENT(X)) .
///
/// __See also__: [crate::of::complex()], [crate::of::imabs()], [crate::of::imargument()], [crate::of::imexp()], [crate::of::imlog10()], [crate::of::ln()], 
#[inline]
pub fn imln<A: Number>(x: A) -> FnNumber1<A> {
    FnNumber1("IMLN", x)
}

/// Returns the common logarithm of a complex number.
///
/// [documentfoundation->IMLOG10](https://wiki.documentfoundation.org/Documentation/Calc_Functions/IMLOG10)
///
/// __Syntax__: 
/// ```ods
///     IMLOG10( X: Complex )
/// ```
///
/// __Constraints__:
/// X ≠ 0
///
/// __Semantics__:
/// IMLOG10(X) is IMDIV(IMLN(X);COMPLEX(LN(10);0)) .
///
/// __See also__: [crate::of::complex()], [crate::of::imdiv()], [crate::of::imln()], [crate::of::impower()], [crate::of::ln()], 
#[inline]
pub fn imlog10<A: Number>(x: A) -> FnNumber1<A> {
    FnNumber1("IMLOG10", x)
}

/// Returns the binary logarithm of a complex number.
///
/// [documentfoundation->IMLOG2](https://wiki.documentfoundation.org/Documentation/Calc_Functions/IMLOG2)
///
/// __Syntax__: 
/// ```ods
///     IMLOG2( X: Complex )
/// ```
///
/// __Constraints__:
/// X ≠ 0
///
/// __Semantics__:
/// IMLOG2(X) is IMDIV(IMLN(X);COMPLEX(LN(2);0)) .
///
/// __See also__: [crate::of::complex()], [crate::of::imdiv()], [crate::of::imln()], [crate::of::impower()], [crate::of::ln()], 
#[inline]
pub fn imlog2<A: Number>(x: A) -> FnNumber1<A> {
    FnNumber1("IMLOG2", x)
}

/// Returns the complex number X raised to the Yth power.
///
/// [documentfoundation->IMPOWER](https://wiki.documentfoundation.org/Documentation/Calc_Functions/IMPOWER)
///
/// __Syntax__: 
/// ```ods
///     IMPOWER( X: Complex; Y: Complex )
/// ```
///
/// __Constraints__:
/// X ≠ 0
///
/// __Semantics__:
/// IMPOWER(X;Y) is IMEXP(IMPRODUCT(Y; IMLN(X)))
/// 
/// An evaluator implementing this function shall permit any Number Y but may 
/// also allow any Complex Y.
///
/// __See also__: [crate::of::imexp()], [crate::of::imln()], [crate::of::improduct()], 
#[inline]
pub fn impower<A: Number, B: Number>(x: A, y: B) -> FnNumber2<A, B> {
    FnNumber2("IMPOWER", x, y)
}

/// Returns the product of complex numbers.
///
/// [documentfoundation->IMPRODUCT](https://wiki.documentfoundation.org/Documentation/Calc_Functions/IMPRODUCT)
///
/// __Syntax__: 
/// ```ods
///     IMPRODUCT({ N: ComplexSequence}+ )
/// ```
///
/// __Constraints__:
/// None
///
/// __Semantics__:
/// Multiply the complex numbers together. Given two complex numbers X = a + bi 
/// and
/// Y = c + di, the product X * Y = (ac - bd) + (ad + bc)i
///
/// __See also__: [crate::of::imdiv()], 
#[inline]
pub fn improduct<A: Sequence>(n: A) -> FnNumber1<A> {
    FnNumber1("IMPRODUCT", n)
}

/// Returns the real coefficient of a complex number.
///
/// [documentfoundation->IMREAL](https://wiki.documentfoundation.org/Documentation/Calc_Functions/IMREAL)
///
/// __Syntax__: 
/// ```ods
///     IMREAL( N: Complex )
/// ```
///
/// __Constraints__:
/// None
///
/// __Semantics__:
/// If N = a + bi or N = a + bj, then the real coefficient is a.
///
/// __See also__: [crate::of::imaginary()], 
#[inline]
pub fn imreal<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("IMREAL", n)
}

/// Returns the sine of a complex number.
///
/// [documentfoundation->IMSIN](https://wiki.documentfoundation.org/Documentation/Calc_Functions/IMSIN)
///
/// __Syntax__: 
/// ```ods
///     IMSIN( N: Complex )
/// ```
///
/// __Constraints__:
/// None
///
/// __Semantics__:
/// If N = a + bi, then sin(N) = sin(a)cosh(b) + cos(a)sinh(b)i.
///
/// __See also__: [crate::of::imcos()], 
#[inline]
pub fn imsin<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("IMSIN", n)
}

/// Returns the hyperbolic sine of a complex number.
///
/// [documentfoundation->IMSINH](https://wiki.documentfoundation.org/Documentation/Calc_Functions/IMSINH)
///
/// __Syntax__: 
/// ```ods
///     IMSINH( N: Complex )
/// ```
///
/// __Constraints__:
/// None
///
/// __Semantics__:
/// If N = a + bi, then sinh(N) = sinh(a)cos(b) + cosh(a)sin(b)i.
///
/// __See also__: 
#[inline]
pub fn imsinh<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("IMSINH", n)
}

/// Returns the secant of a complex number.
///
/// [documentfoundation->IMSEC](https://wiki.documentfoundation.org/Documentation/Calc_Functions/IMSEC)
///
/// __Syntax__: 
/// ```ods
///     IMSEC( N: Complex )
/// ```
///
/// __Constraints__:
/// None
///
/// __Semantics__:
/// Equivalent to the following:
/// 
/// IMDIV(1;IMCOS(N))
///
/// __See also__: [crate::of::imcos()], [crate::of::imdiv()], 
#[inline]
pub fn imsec<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("IMSEC", n)
}

/// Returns the hyperbolic secant of a complex number.
///
/// [documentfoundation->IMSECH](https://wiki.documentfoundation.org/Documentation/Calc_Functions/IMSECH)
///
/// __Syntax__: 
/// ```ods
///     IMSECH( N: Complex )
/// ```
///
/// __Constraints__:
/// None
///
/// __Semantics__:
/// Computes the hyperbolic secant. This is equivalent to:
/// 
/// IMDIV(1;IMCOSH(N))
///
/// __See also__: [crate::of::imcosh()], [crate::of::imdiv()], [crate::of::sech()], 
#[inline]
pub fn imsech<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("IMSECH", n)
}

/// Returns the square root of a complex number.
///
/// [documentfoundation->IMSQRT](https://wiki.documentfoundation.org/Documentation/Calc_Functions/IMSQRT)
///
/// __Syntax__: 
/// ```ods
///     IMSQRT( N: Complex )
/// ```
///
/// __Constraints__:
/// None
///
/// __Semantics__:
/// If N = 0 + 0i, then IMSQRT(N) = 0. Otherwise IMSQRT(N) is
/// SQRT(IMABS(N)) * sin(IMARGUMENT(N) / 2) + SQRT(IMABS(N)) * 
/// cos(IMARGUMENT(N) / 2)i.
///
/// __See also__: [crate::of::imabs()], [crate::of::imargument()], [crate::of::impower()], [crate::of::sqrt()], 
#[inline]
pub fn imsqrt<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("IMSQRT", n)
}

/// Subtracts the second complex number from the first.
///
/// [documentfoundation->IMSUB](https://wiki.documentfoundation.org/Documentation/Calc_Functions/IMSUB)
///
/// __Syntax__: 
/// ```ods
///     IMSUB( X: Complex; Y: Complex )
/// ```
///
/// __Constraints__:
/// None
///
/// __Semantics__:
/// Subtract complex number Y from X.
///
/// __See also__: [crate::of::imsum()], 
#[inline]
pub fn imsub<A: Number, B: Number>(x: A, y: B) -> FnNumber2<A, B> {
    FnNumber2("IMSUB", x, y)
}

/// Sums (add) a set of complex numbers, including all numbers in ranges.
///
/// [documentfoundation->IMSUM](https://wiki.documentfoundation.org/Documentation/Calc_Functions/IMSUM)
///
/// __Syntax__: 
/// ```ods
///     IMSUM({ N: ComplexSequence}+ )
/// ```
///
/// __Constraints__:
/// None
///
/// __Semantics__:
/// Adds complex numbers together. Text that cannot be converted to a complex 
/// number is ignored.
/// 
/// It is implementation-defined what happens if this function is given zero 
/// parameters; an evaluator may either produce an Error or the Number 0 if it 
/// is given zero parameters.
///
/// __See also__: [crate::of::imsub()], 
#[inline]
pub fn imsum<A: Sequence>(n: A) -> FnNumber1<A> {
    FnNumber1("IMSUM", n)
}

/// Returns the tangent of a complex number
///
/// [documentfoundation->IMTAN](https://wiki.documentfoundation.org/Documentation/Calc_Functions/IMTAN)
///
/// __Syntax__: 
/// ```ods
///     IMTAN( N: Complex )
/// ```
///
/// __Constraints__:
/// None
///
/// __Semantics__:
/// Equivalent to the following (except N is computed only once):
/// 
/// IMDIV(IMSIN(N);IMCOS(N))
///
/// __See also__: [crate::of::imdiv()], [crate::of::imsin()], [crate::of::imcos()], [crate::of::imcot()], 
#[inline]
pub fn imtan<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("IMTAN", n)
}
