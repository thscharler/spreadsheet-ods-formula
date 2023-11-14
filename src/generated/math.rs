use crate::*;
#[allow(unused_imports)]
use crate::math::*;

///  Return the absolute (nonnegative) value.
#[inline]
pub fn abs<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("ABS", n)
}

///  Returns the principal value of the arc cosine of a number. The angle is returned in radians.
#[inline]
pub fn acos<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("ACOS", n)
}

///  Return the principal value of the inverse hyperbolic cosine.
#[inline]
pub fn acosh<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("ACOSH", n)
}

///  Return the principal value of the arc cotangent of a number. The angle is returned in radians.
#[inline]
pub fn acot<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("ACOT", n)
}

///  Return the hyperbolic arc cotangent
#[inline]
pub fn acoth<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("ACOTH", n)
}

///  Return the principal value of the arc sine of a number. The angle is returned in radians.
#[inline]
pub fn asin<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("ASIN", n)
}

///  Return the principal value of the inverse hyperbolic sine
#[inline]
pub fn asinh<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("ASINH", n)
}

///  Return the principal value of the arc tangent of a number. The angle is returned in radians.
#[inline]
pub fn atan<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("ATAN", n)
}

///  Returns the principal value of the arc tangent given a coordinate of two numbers. The angle is returned in radians.
#[inline]
pub fn atan2<A: Number, B: Number>(x: A, y: B) -> FnNumber2<A, B> {
    FnNumber2("ATAN2", x, y)
}

///  Return the principal value of the inverse hyperbolic tangent
#[inline]
pub fn atanh<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("ATANH", n)
}

///  Returns the modified Bessel function of integer order In(X).
#[inline]
pub fn besseli<A: Number, B: Number>(x: A, n: B) -> FnNumber2<A, B> {
    FnNumber2("BESSELI", x, n)
}

///  Returns the Bessel function of integer order Jn(X) (cylinder function)
#[inline]
pub fn besselj<A: Number, B: Number>(x: A, n: B) -> FnNumber2<A, B> {
    FnNumber2("BESSELJ", x, n)
}

///  Returns the modified Bessel function of integer order Kn(x).
#[inline]
pub fn besselk<A: Number, B: Number>(x: A, n: B) -> FnNumber2<A, B> {
    FnNumber2("BESSELK", x, n)
}

///  Returns the Bessel function of integer order Yn(X), also known as the Neumann function.
#[inline]
pub fn bessely<A: Number, B: Number>(x: A, n: B) -> FnNumber2<A, B> {
    FnNumber2("BESSELY", x, n)
}

///  Returns the number of different R-length sets that can be selected from N items.
#[inline]
pub fn combin<A: Number, B: Number>(n: A, r: B) -> FnNumber2<A, B> {
    FnNumber2("COMBIN", n, r)
}

///  Returns the number of combinations with repetitions.
#[inline]
pub fn combina<A: Number, B: Number>(n: A, m: B) -> FnNumber2<A, B> {
    FnNumber2("COMBINA", n, m)
}

///  Returns a number converted from one unit system into another
#[inline]
pub fn convert<A: Number, B: Text, C: Text>(n: A, from: B, into: C) -> FnNumber3<A, B, C> {
    FnNumber3("CONVERT", n, from, into)
}

///  Return the cosine of an angle specified in radians.
#[inline]
pub fn cos<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("COS", n)
}

///  Return the hyperbolic cosine of the given hyperbolic angle.
#[inline]
pub fn cosh<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("COSH", n)
}

///  Return the cotangent of an angle specified in radians.
#[inline]
pub fn cot<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("COT", n)
}

///  Return the hyperbolic cotangent of the given hyperbolic angle.
#[inline]
pub fn coth<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("COTH", n)
}

///  Return the cosecant of an angle specified in radians.
#[inline]
pub fn csc<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("CSC", n)
}

///  Return the hyperbolic cosecant of the given angle specified in radians.
#[inline]
pub fn csch<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("CSCH", n)
}

///  Convert radians to degrees.
#[inline]
pub fn degrees<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("DEGREES", n)
}

///  Report if two numbers are equal, returns 1 if they are equal
#[inline]
pub fn delta<A: Number, B: Number>(x: A, y: Option<B>) -> FnNumber2<A, Option<B>> {
    FnNumber2("DELTA", x, y)
}

///  Calculates the error function.
#[inline]
pub fn erf<A: Number, B: Number>(z0: A, z1: B) -> FnNumber2<A, B> {
    FnNumber2("ERF", z0, z1)
}

///  Calculates the complementary error function.
#[inline]
pub fn erfc<A: Number>(z: A) -> FnNumber1<A> {
    FnNumber1("ERFC", z)
}

///  Rounds a number up to the nearest even integer. Rounding is away from zero.
#[inline]
pub fn even<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("EVEN", n)
}

///  Returns e raised by the given number
#[inline]
pub fn exp<A: Number>(x: A) -> FnNumber1<A> {
    FnNumber1("EXP", x)
}

///  Return factorial (!).
#[inline]
pub fn fact<A: Number>(f: A) -> FnNumber1<A> {
    FnNumber1("FACT", f)
}

///  Returns double factorial (!!).
#[inline]
pub fn factdouble<A: Number>(f: A) -> FnNumber1<A> {
    FnNumber1("FACTDOUBLE", f)
}

///  Return gamma function value.
#[inline]
pub fn gamma<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("GAMMA", n)
}

///  Returns the natural logarithm of the GAMMA function.
#[inline]
pub fn gammaln<A: Number>(x: A) -> FnNumber1<A> {
    FnNumber1("GAMMALN", x)
}

///  Returns the greatest common divisor (GCD)
#[inline]
pub fn gcd<A: Sequence>(x: A) -> FnNumber1<A> {
    FnNumber1("GCD", x)
}

///  Returns 1 if a number is greater than or equal to another number, else returns 0.
#[inline]
pub fn gestep<A: Number, B: Number>(x: A, step: Option<B>) -> FnNumber2<A, Option<B>> {
    FnNumber2("GESTEP", x, step)
}

///  Returns the least common multiplier
#[inline]
pub fn lcm<A: Sequence>(x: A) -> FnNumber1<A> {
    FnNumber1("LCM", x)
}

///  Return the natural logarithm of a number.
#[inline]
pub fn ln<A: Number>(x: A) -> FnNumber1<A> {
    FnNumber1("LN", x)
}

///  Return the logarithm of a number in a specified base.
#[inline]
pub fn log<A: Number, B: Number>(n: A, base: Option<B>) -> FnNumber2<A, Option<B>> {
    FnNumber2("LOG", n, base)
}

///  Return the base 10 logarithm of a number.
#[inline]
pub fn log10<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("LOG10", n)
}

///  Return the remainder when one number is divided by another number.
#[inline]
pub fn mod_<A: Number, B: Number>(a: A, b: B) -> FnNumber2<A, B> {
    FnNumber2("MOD", a, b)
}

///  Returns the multinomial for the given values.
#[inline]
pub fn multinominal<A: Sequence>(a: A) -> FnNumber1<A> {
    FnNumber1("MULTINOMINAL", a)
}

///  Rounds a number up to the nearest odd integer, where "up" means "away
#[inline]
pub fn odd<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("ODD", n)
}

///  Return the approximate value of π.
#[inline]
pub fn pi() -> FnNumber0 {
    FnNumber0("PI", )
}

///  Return the value of one number raised to the power of another number.
#[inline]
pub fn power<A: Number, B: Number>(a: A, b: B) -> FnNumber2<A, B> {
    FnNumber2("POWER", a, b)
}

///  Multiply the set of numbers, including all numbers inside ranges
#[inline]
pub fn product<A: Sequence>(a: A) -> FnNumber1<A> {
    FnNumber1("PRODUCT", a)
}

///  Return the integer portion of a division.
#[inline]
pub fn quotient<A: Number, B: Number>(a: A, b: B) -> FnNumber2<A, B> {
    FnNumber2("QUOTIENT", a, b)
}

///  Convert degrees to radians.
#[inline]
pub fn radians<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("RADIANS", n)
}

///  Return a random number between 0 (inclusive) and 1 (exclusive).
#[inline]
pub fn rand() -> FnNumber0 {
    FnNumber0("RAND", )
}

///  Return a random integer number between A and B.
#[inline]
pub fn randbetween<A: Number, B: Number>(a: A, b: B) -> FnNumber2<A, B> {
    FnNumber2("RANDBETWEEN", a, b)
}

///  Return the secant of an angle specified in radians.
#[inline]
pub fn sec<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("SEC", n)
}

///  Return the hyperbolic secant of the given angle specified in radians
#[inline]
pub fn sech<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("SECH", n)
}

///  Returns the sum of a power series.
#[inline]
pub fn seriessum<A: Number, B: Number, C: Number, D: Array>(x: A, n: B, m: C, coefficients: D) -> FnNumber4<A, B, C, D> {
    FnNumber4("SERIESSUM", x, n, m, coefficients)
}

///  Return the sign of a number.
#[inline]
pub fn sign<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("SIGN", n)
}

///  Return the sine of an angle specified in radians.
#[inline]
pub fn sin<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("SIN", n)
}

///  Return the hyperbolic sine of the given hyperbolic angle.
#[inline]
pub fn sinh<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("SINH", n)
}

///  Return the square root of a number.
#[inline]
pub fn sqrt<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("SQRT", n)
}

///  Return the square root of a number multiplied by π (pi)
#[inline]
pub fn sqrtpi<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("SQRTPI", n)
}

///  Evaluates a function on a range.
#[inline]
pub fn subtotal<A: Sequence>(f: SubtotalFunction, seq: A) -> FnNumber2<SubtotalFunction, A> {
    FnNumber2("SUBTOTAL", f, seq)
}

///  Sum (add) the set of numbers, including all numbers in ranges.
#[inline]
pub fn sum<A: Sequence>(n: A) -> FnNumber1<A> {
    FnNumber1("SUM", n)
}

///  Sum the values of cells in a range that meet a criteria.
#[inline]
pub fn sumif<A: Sequence, B: Criterion, C: Reference>(n: A, crit: B, refs: Option<C>) -> FnNumber3<A, B, Option<C>> {
    FnNumber3("SUMIF", n, crit, refs)
}

///  Returns the sum of the products of the matrix elements
#[inline]
pub fn sumproduct<A: Sequence>(a: A) -> FnNumber1<A> {
    FnNumber1("SUMPRODUCT", a)
}

///  Sum (add) the set of squares of numbers, including all numbers in ranges
#[inline]
pub fn sumsq<A: Sequence>(n: A) -> FnNumber1<A> {
    FnNumber1("SUMSQ", n)
}

///  Returns the sum of the difference between the squares of the matrices A and B.
#[inline]
pub fn sumx2my2<A: Array, B: Array>(a: A, b: B) -> FnNumber2<A, B> {
    FnNumber2("SUMX2MY2", a, b)
}

///  Returns the total sum of the squares of the matrices A and B
#[inline]
pub fn sumx2py2<A: Array, B: Array>(a: A, b: B) -> FnNumber2<A, B> {
    FnNumber2("SUMX2PY2", a, b)
}

///  Returns the sum of the squares of the differences between matrix A and B.
#[inline]
pub fn sumxmy2<A: Array, B: Array>(a: A, b: B) -> FnNumber2<A, B> {
    FnNumber2("SUMXMY2", a, b)
}

///  Return the tangent of an angle specified in radians
#[inline]
pub fn tan<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("TAN", n)
}

///  Return the hyperbolic tangent of the given hyperbolic angle
#[inline]
pub fn tanh<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("TANH", n)
}
