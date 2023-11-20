use crate::*;
#[allow(unused_imports)]
use crate::math::*;

#[inline]
pub fn abs<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("ABS", n)
}

#[inline]
pub fn acos<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("ACOS", n)
}

#[inline]
pub fn acosh<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("ACOSH", n)
}

#[inline]
pub fn acot<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("ACOT", n)
}

#[inline]
pub fn acoth<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("ACOTH", n)
}

#[inline]
pub fn asin<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("ASIN", n)
}

#[inline]
pub fn asinh<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("ASINH", n)
}

#[inline]
pub fn atan<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("ATAN", n)
}

#[inline]
pub fn atan2<A: Number, B: Number>(x: A, y: B) -> FnNumber2<A, B> {
    FnNumber2("ATAN2", x, y)
}

#[inline]
pub fn atanh<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("ATANH", n)
}

#[inline]
pub fn besseli<A: Number, B: Number>(x: A, n: B) -> FnNumber2<A, B> {
    FnNumber2("BESSELI", x, n)
}

#[inline]
pub fn besselj<A: Number, B: Number>(x: A, n: B) -> FnNumber2<A, B> {
    FnNumber2("BESSELJ", x, n)
}

#[inline]
pub fn besselk<A: Number, B: Number>(x: A, n: B) -> FnNumber2<A, B> {
    FnNumber2("BESSELK", x, n)
}

#[inline]
pub fn bessely<A: Number, B: Number>(x: A, n: B) -> FnNumber2<A, B> {
    FnNumber2("BESSELY", x, n)
}

#[inline]
pub fn combin<A: Number, B: Number>(n: A, r: B) -> FnNumber2<A, B> {
    FnNumber2("COMBIN", n, r)
}

#[inline]
pub fn combina<A: Number, B: Number>(n: A, m: B) -> FnNumber2<A, B> {
    FnNumber2("COMBINA", n, m)
}

#[inline]
pub fn convert<A: Number, B: Text, C: Text>(n: A, from: B, into: C) -> FnNumber3<A, B, C> {
    FnNumber3("CONVERT", n, from, into)
}

#[inline]
pub fn cos<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("COS", n)
}

#[inline]
pub fn cosh<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("COSH", n)
}

#[inline]
pub fn cot<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("COT", n)
}

#[inline]
pub fn coth<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("COTH", n)
}

#[inline]
pub fn csc<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("CSC", n)
}

#[inline]
pub fn csch<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("CSCH", n)
}

#[inline]
pub fn degrees<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("DEGREES", n)
}

#[inline]
pub fn delta<A: Number>(x: A) -> FnNumber1<A> {
    FnNumber1("DELTA", x)
}

#[inline]
pub fn delta_<A: Number, B: Number>(x: A, y: B) -> FnNumber2<A, B> {
    FnNumber2("DELTA", x, y)
}

#[inline]
pub fn erf<A: Number>(z0: A) -> FnNumber1<A> {
    FnNumber1("ERF", z0)
}

#[inline]
pub fn erf_<A: Number, B: Number>(z0: A, z1: B) -> FnNumber2<A, B> {
    FnNumber2("ERF", z0, z1)
}

#[inline]
pub fn erfc<A: Number>(z: A) -> FnNumber1<A> {
    FnNumber1("ERFC", z)
}

#[inline]
pub fn euroconvert<A: Number, B: Text, C: Text>(n: A, from: B, to: C) -> FnNumber3<A, B, C> {
    FnNumber3("EUROCONVERT", n, from, to)
}

#[inline]
pub fn euroconvert_<A: Number, B: Text, C: Text, D: Logical>(n: A, from: B, to: C, full_precision: D) -> FnNumber4<A, B, C, D> {
    FnNumber4("EUROCONVERT", n, from, to, full_precision)
}

#[inline]
pub fn euroconvert__<A: Number, B: Text, C: Text, D: Logical, E: Number>(n: A, from: B, to: C, full_precision: D, triangulation_precision: E) -> FnNumber5<A, B, C, D, E> {
    FnNumber5("EUROCONVERT", n, from, to, full_precision, triangulation_precision)
}

#[inline]
pub fn even<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("EVEN", n)
}

#[inline]
pub fn exp<A: Number>(x: A) -> FnNumber1<A> {
    FnNumber1("EXP", x)
}

#[inline]
pub fn fact<A: Number>(f: A) -> FnNumber1<A> {
    FnNumber1("FACT", f)
}

#[inline]
pub fn factdouble<A: Number>(f: A) -> FnNumber1<A> {
    FnNumber1("FACTDOUBLE", f)
}

#[inline]
pub fn gamma<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("GAMMA", n)
}

#[inline]
pub fn gammaln<A: Number>(x: A) -> FnNumber1<A> {
    FnNumber1("GAMMALN", x)
}

#[inline]
pub fn gcd<A: Sequence>(x: A) -> FnNumber1<A> {
    FnNumber1("GCD", x)
}

#[inline]
pub fn gestep<A: Number>(x: A) -> FnNumber1<A> {
    FnNumber1("GESTEP", x)
}

#[inline]
pub fn gestep_<A: Number, B: Number>(x: A, step: B) -> FnNumber2<A, B> {
    FnNumber2("GESTEP", x, step)
}

#[inline]
pub fn lcm<A: Sequence>(x: A) -> FnNumber1<A> {
    FnNumber1("LCM", x)
}

#[inline]
pub fn ln<A: Number>(x: A) -> FnNumber1<A> {
    FnNumber1("LN", x)
}

#[inline]
pub fn log<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("LOG", n)
}

#[inline]
pub fn log_<A: Number, B: Number>(n: A, base: B) -> FnNumber2<A, B> {
    FnNumber2("LOG", n, base)
}

#[inline]
pub fn log10<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("LOG10", n)
}

#[inline]
pub fn mod_<A: Number, B: Number>(a: A, b: B) -> FnNumber2<A, B> {
    FnNumber2("MOD", a, b)
}

#[inline]
pub fn multinomial<A: Sequence>(a: A) -> FnNumber1<A> {
    FnNumber1("MULTINOMIAL", a)
}

#[inline]
pub fn odd<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("ODD", n)
}

#[inline]
pub fn pi() -> FnNumber0 {
    FnNumber0("PI", )
}

#[inline]
pub fn power<A: Number, B: Number>(a: A, b: B) -> FnNumber2<A, B> {
    FnNumber2("POWER", a, b)
}

#[inline]
pub fn product<A: Sequence>(n: A) -> FnNumber1<A> {
    FnNumber1("PRODUCT", n)
}

#[inline]
pub fn quotient<A: Number, B: Number>(a: A, b: B) -> FnNumber2<A, B> {
    FnNumber2("QUOTIENT", a, b)
}

#[inline]
pub fn radians<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("RADIANS", n)
}

#[inline]
pub fn rand() -> FnNumber0 {
    FnNumber0("RAND", )
}

#[inline]
pub fn randbetween<A: Number, B: Number>(a: A, b: B) -> FnNumber2<A, B> {
    FnNumber2("RANDBETWEEN", a, b)
}

#[inline]
pub fn sec<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("SEC", n)
}

#[inline]
pub fn seriessum<A: Number, B: Number, C: Number, D: Array>(x: A, n: B, m: C, coefficients: D) -> FnNumber4<A, B, C, D> {
    FnNumber4("SERIESSUM", x, n, m, coefficients)
}

#[inline]
pub fn sign<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("SIGN", n)
}

#[inline]
pub fn sin<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("SIN", n)
}

#[inline]
pub fn sinh<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("SINH", n)
}

#[inline]
pub fn sech<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("SECH", n)
}

#[inline]
pub fn sqrt<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("SQRT", n)
}

#[inline]
pub fn sqrtpi<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("SQRTPI", n)
}

#[inline]
pub fn subtotal<A: Sequence>(function: SubtotalFunction, sequence: A) -> FnNumber2<SubtotalFunction, A> {
    FnNumber2("SUBTOTAL", function, sequence)
}

#[inline]
pub fn sum<A: Sequence>(n: A) -> FnNumber1<A> {
    FnNumber1("SUM", n)
}

#[inline]
pub fn sumif<A: Reference, B: Criterion>(r: A, c: B) -> FnNumber2<A, B> {
    FnNumber2("SUMIF", r, c)
}

#[inline]
pub fn sumif_<A: Reference, B: Criterion, C: Reference>(r: A, c: B, s: C) -> FnNumber3<A, B, C> {
    FnNumber3("SUMIF", r, c, s)
}

#[inline]
pub fn sumproduct<A: Sequence>(a: A) -> FnNumber1<A> {
    FnNumber1("SUMPRODUCT", a)
}

#[inline]
pub fn sumsq<A: Sequence>(n: A) -> FnNumber1<A> {
    FnNumber1("SUMSQ", n)
}

#[inline]
pub fn sumx2my2<A: Array, B: Array>(a: A, b: B) -> FnNumber2<A, B> {
    FnNumber2("SUMX2MY2", a, b)
}

#[inline]
pub fn sumx2py2<A: Array, B: Array>(a: A, b: B) -> FnNumber2<A, B> {
    FnNumber2("SUMX2PY2", a, b)
}

#[inline]
pub fn sumxmy2<A: Array, B: Array>(a: A, b: B) -> FnNumber2<A, B> {
    FnNumber2("SUMXMY2", a, b)
}

#[inline]
pub fn tan<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("TAN", n)
}

#[inline]
pub fn tanh<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("TANH", n)
}
