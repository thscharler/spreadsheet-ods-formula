//! 
//! Functions for complex numbers.

use crate::*;
#[allow(unused_imports)]
use crate::complex::*;

/// Creates a complex number from a given real coefficient and imaginary 
/// coefficient.
/// Syntax: COMPLEX( Real Number;; Imaginary Number; )
///
/// Constraints:
/// None
///
/// Semantics:
/// Constructs a complex number from the given coefficients. The third 
/// parameter Suffix is optional, and should be either “i” or “j”. 
/// Upper case “I” or “J” are not accepted for the suffix parameter.
#[inline]
pub fn complex<A: Number, B: Number>(real: A, imaginary: B) -> FnNumber2<A, B> {
    FnNumber2("COMPLEX", real, imaginary)
}

/// Creates a complex number from a given real coefficient and imaginary 
/// coefficient.
/// Syntax: COMPLEX( Real Number;; Imaginary Number;[; Suffix Text] )
///
/// Constraints:
/// None
///
/// Semantics:
/// Constructs a complex number from the given coefficients. The third 
/// parameter Suffix is optional, and should be either “i” or “j”. 
/// Upper case “I” or “J” are not accepted for the suffix parameter.
#[inline]
pub fn complex_<A: Number, B: Number, C: Text>(real: A, imaginary: B, suffix: C) -> FnNumber3<A, B, C> {
    FnNumber3("COMPLEX", real, imaginary, suffix)
}

/// Returns the absolute value of a complex number.
/// Syntax: IMABS( X Complex; )
///
/// Constraints:
/// None
///
/// Semantics:
/// If X = a + bi or X = a + bj, the absolute value =
/// ; if X = r(cosφ + isinφ), the absolute value = r.
///
/// See also: "IMARGUMENT", 
#[inline]
pub fn imabs<A: Number>(x: A) -> FnNumber1<A> {
    FnNumber1("IMABS", x)
}

/// Returns the imaginary coefficient of a complex number.
/// Syntax: IMAGINARY( X Complex; )
///
/// Constraints:
/// None
///
/// Semantics:
/// If X = a + bi or X = a + bj, then the imaginary coefficient is b.
///
/// See also: "IMREAL", 
#[inline]
pub fn imaginary<A: Number>(x: A) -> FnNumber1<A> {
    FnNumber1("IMAGINARY", x)
}

/// Returns the complex argument of a complex number.
/// Syntax: IMARGUMENT( X Complex; )
///
/// Constraints:
/// None
///
/// Semantics:
/// If X = a + bi = r(cosφ + isinφ), a or b is not 0 and -π < φ ≤ π, 
/// then the complex argument is φ. φ is expressed by radians. If X = 0, then 
/// IMARGUMENT(X) is implementation-defined and either 0 or an error.
///
/// See also: "IMABS", 
#[inline]
pub fn imargument<A: Number>(x: A) -> FnNumber1<A> {
    FnNumber1("IMARGUMENT", x)
}

/// Returns the complex conjugate of a complex number.
/// Syntax: IMCONJUGATE( X Complex; )
///
/// Constraints:
/// None
///
/// Semantics:
/// If X = a + bi, then the complex conjugate is a - bi.
#[inline]
pub fn imconjugate<A: Number>(x: A) -> FnNumber1<A> {
    FnNumber1("IMCONJUGATE", x)
}

/// Returns the cosine of a complex number.
/// Syntax: IMCOS( X Complex; )
///
/// Constraints:
/// None
///
/// Semantics:
/// If X = a + bi, then cos(X) = cos(a)cosh(b) - sin(a)sinh(b)i.
///
/// See also: "IMSIN", 
#[inline]
pub fn imcos<A: Number>(x: A) -> FnNumber1<A> {
    FnNumber1("IMCOS", x)
}

/// Returns the hyperbolic cosine of a complex number.
/// Syntax: IMCOSH( N Complex; )
///
/// Constraints:
/// None
///
/// Semantics:
/// If N = a + bi, then cosh(N) = cosh(a)cos(b) + sinh(a)sin(b)i.
#[inline]
pub fn imcosh<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("IMCOSH", n)
}

/// Returns the cotangent of a complex number.
/// Syntax: IMCOT( N Complex; )
///
/// Constraints:
/// None
///
/// Semantics:
/// Equivalent to the following (except N is computed only once):
/// 
/// IMDIV(IMCOS(N);IMSIN(N))
///
/// See also: "IMCOS", "IMDIV", "IMSIN", "IMTAN", 
#[inline]
pub fn imcot<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("IMCOT", n)
}

/// Returns the cosecant of a complex number.
/// Syntax: IMCSC( N Complex; )
///
/// Constraints:
/// None
///
/// Semantics:
/// Equivalent to the following:
/// 
/// IMDIV(1;IMSIN(N))
///
/// See also: "IMDIV", "IMSIN", 
#[inline]
pub fn imcsc<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("IMCSC", n)
}

/// Returns the hyperbolic cosecant of a complex number.
/// Syntax: IMCSCH( N Complex; )
///
/// Constraints:
/// None
///
/// Semantics:
/// Computes the hyperbolic cosecant. This is equivalent to:
/// 
/// IMDIV(1;IMSINH(N))
///
/// See also: "IMSINH", "CSCH", 
#[inline]
pub fn imcsch<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("IMCSCH", n)
}

/// Divides the first number by the second.
/// Syntax: IMDIV( X Complex;; Y Complex; )
///
/// Constraints:
/// None
///
/// Semantics:
/// Given X = a + bi and Y = c + di, return the quotient
/// 
/// Division by zero returns an Error.
///
/// See also: "IMDIV", 
#[inline]
pub fn imdiv<A: Number, B: Number>(x: A, y: B) -> FnNumber2<A, B> {
    FnNumber2("IMDIV", x, y)
}

/// Returns the exponent of e and a complex number.
/// Syntax: IMEXP( X Complex; )
///
/// Constraints:
/// None
///
/// Semantics:
/// If X = a + bi, the result is
/// .
///
/// See also: "IMLN", 
#[inline]
pub fn imexp<A: Number>(x: A) -> FnNumber1<A> {
    FnNumber1("IMEXP", x)
}

/// Returns the natural logarithm of a complex number.
/// Syntax: IMLN( X Complex; )
///
/// Constraints:
/// X ≠ 0
///
/// Semantics:
/// COMPLEX(LN(IMABS(X)); IMARGUMENT(X)) .
///
/// See also: "COMPLEX", "IMABS", "IMARGUMENT", "IMEXP", "IMLOG10", "LN", 
#[inline]
pub fn imln<A: Number>(x: A) -> FnNumber1<A> {
    FnNumber1("IMLN", x)
}

/// Returns the common logarithm of a complex number.
/// Syntax: IMLOG10( X Complex; )
///
/// Constraints:
/// X ≠ 0
///
/// Semantics:
/// IMLOG10(X) is IMDIV(IMLN(X);COMPLEX(LN(10);0)) .
///
/// See also: "COMPLEX", "IMDIV", "IMLN", "IMPOWER", "LN", 
#[inline]
pub fn imlog10<A: Number>(x: A) -> FnNumber1<A> {
    FnNumber1("IMLOG10", x)
}

/// Returns the binary logarithm of a complex number.
/// Syntax: IMLOG2( X Complex; )
///
/// Constraints:
/// X ≠ 0
///
/// Semantics:
/// IMLOG2(X) is IMDIV(IMLN(X);COMPLEX(LN(2);0)) .
///
/// See also: "COMPLEX", "IMDIV", "IMLN", "IMPOWER", "LN", 
#[inline]
pub fn imlog2<A: Number>(x: A) -> FnNumber1<A> {
    FnNumber1("IMLOG2", x)
}

/// Returns the complex number X raised to the Yth power.
/// Syntax: IMPOWER( X Complex;; Y Complex; )
///
/// Constraints:
/// X ≠ 0
///
/// Semantics:
/// IMPOWER(X;Y) is IMEXP(IMPRODUCT(Y; IMLN(X)))
/// 
/// An evaluator implementing this function shall permit any Number Y but may 
/// also allow any Complex Y.
///
/// See also: "IMEXP", "IMLN", "IMPOWER", "IMPRODUCT", 
#[inline]
pub fn impower<A: Number, B: Number>(x: A, y: B) -> FnNumber2<A, B> {
    FnNumber2("IMPOWER", x, y)
}

/// Returns the product of complex numbers.
/// Syntax: IMPRODUCT({ N ComplexSequence}+ )
///
/// Constraints:
/// None
///
/// Semantics:
/// Multiply the complex numbers together. Given two complex numbers X = a + bi 
/// and
/// Y = c + di, the product X * Y = (ac - bd) + (ad + bc)i
///
/// See also: "IMDIV", 
#[inline]
pub fn improduct<A: Sequence>(n: A) -> FnNumber1<A> {
    FnNumber1("IMPRODUCT", n)
}

/// Returns the real coefficient of a complex number.
/// Syntax: IMREAL( N Complex; )
///
/// Constraints:
/// None
///
/// Semantics:
/// If N = a + bi or N = a + bj, then the real coefficient is a.
///
/// See also: "IMAGINARY", 
#[inline]
pub fn imreal<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("IMREAL", n)
}

/// Returns the sine of a complex number.
/// Syntax: IMSIN( N Complex; )
///
/// Constraints:
/// None
///
/// Semantics:
/// If N = a + bi, then sin(N) = sin(a)cosh(b) + cos(a)sinh(b)i.
///
/// See also: "IMCOS", 
#[inline]
pub fn imsin<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("IMSIN", n)
}

/// Returns the hyperbolic sine of a complex number.
/// Syntax: IMSINH( N Complex; )
///
/// Constraints:
/// None
///
/// Semantics:
/// If N = a + bi, then sinh(N) = sinh(a)cos(b) + cosh(a)sin(b)i.
#[inline]
pub fn imsinh<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("IMSINH", n)
}

/// Returns the secant of a complex number.
/// Syntax: IMSEC( N Complex; )
///
/// Constraints:
/// None
///
/// Semantics:
/// Equivalent to the following:
/// 
/// IMDIV(1;IMCOS(N))
///
/// See also: "IMCOS", "IMDIV", 
#[inline]
pub fn imsec<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("IMSEC", n)
}

/// Returns the hyperbolic secant of a complex number.
/// Syntax: IMSECH( N Complex; )
///
/// Constraints:
/// None
///
/// Semantics:
/// Computes the hyperbolic secant. This is equivalent to:
/// 
/// IMDIV(1;IMCOSH(N))
///
/// See also: "IMCOSH", "IMDIV", "SECH", 
#[inline]
pub fn imsech<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("IMSECH", n)
}

/// Returns the square root of a complex number.
/// Syntax: IMSQRT( N Complex; )
///
/// Constraints:
/// None
///
/// Semantics:
/// If N = 0 + 0i, then IMSQRT(N) = 0. Otherwise IMSQRT(N) is
/// SQRT(IMABS(N)) * sin(IMARGUMENT(N) / 2) + SQRT(IMABS(N)) * 
/// cos(IMARGUMENT(N) / 2)i.
///
/// See also: "IMABS", "IMARGUMENT", "IMPOWER", "SQRT", 
#[inline]
pub fn imsqrt<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("IMSQRT", n)
}

/// Subtracts the second complex number from the first.
/// Syntax: IMSUB( X Complex;; Y Complex; )
///
/// Constraints:
/// None
///
/// Semantics:
/// Subtract complex number Y from X.
///
/// See also: "IMSUM", 
#[inline]
pub fn imsub<A: Number, B: Number>(x: A, y: B) -> FnNumber2<A, B> {
    FnNumber2("IMSUB", x, y)
}

/// Sums (add) a set of complex numbers, including all numbers in ranges.
/// Syntax: IMSUM({ N ComplexSequence}+ )
///
/// Constraints:
/// None
///
/// Semantics:
/// Adds complex numbers together. Text that cannot be converted to a complex 
/// number is ignored.
/// 
/// It is implementation-defined what happens if this function is given zero 
/// parameters; an evaluator may either produce an Error or the Number 0 if it 
/// is given zero parameters.
///
/// See also: "IMSUB", 
#[inline]
pub fn imsum<A: Sequence>(n: A) -> FnNumber1<A> {
    FnNumber1("IMSUM", n)
}

/// Returns the tangent of a complex number
/// Syntax: IMTAN( N Complex; )
///
/// Constraints:
/// None
///
/// Semantics:
/// Equivalent to the following (except N is computed only once):
/// 
/// IMDIV(IMSIN(N);IMCOS(N))
///
/// See also: "IMDIV", "IMSIN", "IMCOS", "IMCOT", 
#[inline]
pub fn imtan<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("IMTAN", n)
}
