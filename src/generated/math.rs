//! 
//! This section describes functions for various mathematical functions, 
//! including trigonometric functions like SIN 6.16.55). Note that the 
//! constraint text presumes that a value of type Number is a real number (no 
//! imaginary component). Unless noted otherwise, all angle measurements are in 
//! radians.

use crate::*;
#[allow(unused_imports)]
use crate::math::*;

/// Return the absolute (nonnegative) value.
///
/// __Syntax__: 
/// ```ods
///     ABS( N: Number )
/// ```
///
/// __Constraints__:
/// None
///
/// __Semantics__:
/// If N < 0, returns -N, otherwise returns N.
///
/// __See also__: "Prefix Operator “-”", 
#[inline]
pub fn abs<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("ABS", n)
}

/// Returns the principal value of the arc cosine of a number. The angle is 
/// returned in radians.
///
/// __Syntax__: 
/// ```ods
///     ACOS( N: Number )
/// ```
///
/// __Constraints__:
/// -1.0 ≤ N ≤ 1.0.
///
/// __Semantics__:
/// Computes the arc cosine of a number, in radians.
/// 
/// Returns a principal value 0 ≤ result ≤ π.
///
/// __See also__: "COS", "RADIANS", "DEGREES", 
#[inline]
pub fn acos<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("ACOS", n)
}

/// Return the principal value of the inverse hyperbolic cosine.
///
/// __Syntax__: 
/// ```ods
///     ACOSH( N: Number )
/// ```
///
/// __Constraints__:
/// N ≥ 1
///
/// __Semantics__:
/// Computes the principal value of the inverse hyperbolic cosine.
///
/// __See also__: "COSH", "ASINH", 
#[inline]
pub fn acosh<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("ACOSH", n)
}

/// Return the principal value of the arc cotangent of a number. The angle is 
/// returned in radians.
///
/// __Syntax__: 
/// ```ods
///     ACOT( N: Number )
/// ```
///
/// __Semantics__:
/// Computes the arc cotangent of a number, in radians.
/// 
/// Returns a principal value 0 < result < π.
///
/// __See also__: "COT", "ATAN", "TAN", "RADIANS", "DEGREES", 
#[inline]
pub fn acot<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("ACOT", n)
}

/// Return the hyperbolic arc cotangent
///
/// __Syntax__: 
/// ```ods
///     ACOTH( N: Number )
/// ```
///
/// __Constraints__:
/// ABS(N) > 1
///
/// __Semantics__:
/// Computes the hyperbolic arc cotangent. The hyperbolic arc cotangent is an 
/// analog of the ordinary (circular) arc cotangent.
///
/// __See also__: "COSH", "ASINH", 
#[inline]
pub fn acoth<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("ACOTH", n)
}

/// Return the principal value of the arc sine of a number. The angle is 
/// returned in radians.
///
/// __Syntax__: 
/// ```ods
///     ASIN( N: Number )
/// ```
///
/// __Constraints__:
/// -1 ≤ N ≤ 1.
///
/// __Semantics__:
/// Computes the arc sine of a number, in radians.
/// 
/// Returns a principal value -π/2 ≤ result ≤ π/2.
///
/// __See also__: "SIN", "RADIANS", "DEGREES", 
#[inline]
pub fn asin<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("ASIN", n)
}

/// Return the principal value of the inverse hyperbolic sine
///
/// __Syntax__: 
/// ```ods
///     ASINH( N: Number )
/// ```
///
/// __Constraints__:
/// None
///
/// __Semantics__:
/// Computes the principal value of the inverse hyperbolic sine.
///
/// __See also__: "SINH", "ACOSH", 
#[inline]
pub fn asinh<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("ASINH", n)
}

/// Return the principal value of the arc tangent of a number. The angle is 
/// returned in radians.
///
/// __Syntax__: 
/// ```ods
///     ATAN( N: Number )
/// ```
///
/// __Semantics__:
/// Computes the arc tangent of a number, in radians.
/// 
/// Returns a principal value -π/2 < result < π/2.
///
/// __See also__: "ATAN2", "TAN", "RADIANS", "DEGREES", 
#[inline]
pub fn atan<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("ATAN", n)
}

/// Returns the principal value of the arc tangent given a coordinate of two 
/// numbers.
/// 
/// The angle is returned in radians.
///
/// __Syntax__: 
/// ```ods
///     ATAN2( x: Number; y: Number )
/// ```
///
/// __Constraints__:
/// x ≠ 0 or y ≠ 0
///
/// __Semantics__:
/// Computes the arc tangent of two numbers (the x and y coordinates of a 
/// point), in radians. This is similar to ATAN(y/x), but the signs of the two 
/// numbers are taken into account so that the result covers the full range 
/// from -π to +π. ATAN2(0;0) is implementation-defined, evaluators may 
/// return 0 or an Error.
/// 
/// Returns a principal value -π < result ≤ π.
///
/// __See also__: "ATAN", "TAN", "RADIANS", "DEGREES", 
#[inline]
pub fn atan2<A: Number, B: Number>(x: A, y: B) -> FnNumber2<A, B> {
    FnNumber2("ATAN2", x, y)
}

/// Return the principal value of the inverse hyperbolic tangent
///
/// __Syntax__: 
/// ```ods
///     ATANH( N: Number )
/// ```
///
/// __Constraints__:
/// -1 < N < 1
///
/// __Semantics__:
/// Computes the principal value of the inverse hyperbolic tangent.
///
/// __See also__: "COSH", "SINH", "ASINH", "ACOSH", "ATAN", "ATAN2", "FISHER", 
#[inline]
pub fn atanh<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("ATANH", n)
}

/// Returns the modified Bessel function of integer order In(X).
///
/// __Syntax__: 
/// ```ods
///     BESSELI( X: Number; N: Number )
/// ```
///
/// __Constraints__:
/// N ≥ 0, INT(N) = N; Evaluators may evaluate expressions where N ≥ 0 
/// returns a non-error value.
///
/// __Semantics__:
/// Computes the modified Bessel function of integer order In(X). N is also 
/// known as the order.
///
/// __See also__: "BESSELJ", "BESSELK", "BESSELY", "INT", 
#[inline]
pub fn besseli<A: Number, B: Number>(x: A, n: B) -> FnNumber2<A, B> {
    FnNumber2("BESSELI", x, n)
}

/// Returns the Bessel function of integer order Jn(X) (cylinder function)
///
/// __Syntax__: 
/// ```ods
///     BESSELJ( X: Number; N: Number )
/// ```
///
/// __Constraints__:
/// N ≥ 0, INT(N) = N; Evaluators may evaluate expressions where N ≥ 0 
/// returns a non-error value.
///
/// __Semantics__:
/// Computes the Bessel function of integer order Jn(X). N is also known as the 
/// order.
///
/// __See also__: "BESSELI", "BESSELK", "BESSELY", "INT", 
#[inline]
pub fn besselj<A: Number, B: Number>(x: A, n: B) -> FnNumber2<A, B> {
    FnNumber2("BESSELJ", x, n)
}

/// Returns the modified Bessel function of integer order Kn(x).
///
/// __Syntax__: 
/// ```ods
///     BESSELK( X: Number; N: Number )
/// ```
///
/// __Constraints__:
/// X ≠ 0, N ≥ 0, INT(N) = N; Evaluators may evaluate expressions where N 
/// ≥ 0 returns a non-error value.
///
/// __Semantics__:
/// Computes the Bessel function of integer order Kn(x). N is also known as the 
/// order.
///
/// __See also__: "BESSELI", "BESSELJ", "BESSELY", "INT", 
#[inline]
pub fn besselk<A: Number, B: Number>(x: A, n: B) -> FnNumber2<A, B> {
    FnNumber2("BESSELK", x, n)
}

/// Returns the Bessel function of integer order Yn(X), also known as the 
/// Neumann function.
///
/// __Syntax__: 
/// ```ods
///     BESSELY( X: Number; N: Number )
/// ```
///
/// __Constraints__:
/// X ≠ 0, N ≥ 0, INT(N) = N; Evaluators may evaluate expressions where N 
/// ≥ 0 returns a non-error value.
///
/// __Semantics__:
/// Computes Bessel function of integer order Yn(X), also known as the Neumann 
/// function. N is also known as the order.
///
/// __See also__: "BESSELI", "BESSELJ", "BESSELK", "INT", 
#[inline]
pub fn bessely<A: Number, B: Number>(x: A, n: B) -> FnNumber2<A, B> {
    FnNumber2("BESSELY", x, n)
}

/// Returns the number of different R-length sets that can be selected from N 
/// items.
///
/// __Syntax__: 
/// ```ods
///     COMBIN( N: Integer; R: Integer )
/// ```
///
/// __Constraints__:
/// N ≥ 0, R ≥ 0, R ≤ N
///
/// __Semantics__:
/// COMBIN returns the binomial coefficient, which is the number of different 
/// R-length sets that can be selected from N items. Since they are sets, order 
/// in the sets is not relevant. The parameters are truncated (using INT) 
/// before use. For example, if a jar contains five marbles, each one a 
/// distinct color, the number of different three-marble groups COMBIN(5;3) = 
/// 10. The result is
/// 
/// Note that if order is important, use PERMUT instead.
///
/// __See also__: "INT", "PERMUT", 
#[inline]
pub fn combin<A: Number, B: Number>(n: A, r: B) -> FnNumber2<A, B> {
    FnNumber2("COMBIN", n, r)
}

/// Returns the number of combinations with repetitions.
///
/// __Syntax__: 
/// ```ods
///     COMBINA( N: Integer; M: Integer )
/// ```
///
/// __Constraints__:
/// N ≥ 0, M ≥ 0, N ≥ M; Evaluators may evaluate expressions where N ≥ 
/// 0, M ≥ 0 returns a non-error value.
///
/// __Semantics__:
/// Returns the number of possible combinations of M objects out of N possible 
/// ones, with repetitions allowed. Actual arguments that are not integers are 
/// truncated (using INT) before use. The result is
///
/// __See also__: "COMBIN", 
#[inline]
pub fn combina<A: Number, B: Number>(n: A, m: B) -> FnNumber2<A, B> {
    FnNumber2("COMBINA", n, m)
}

/// Returns a number converted from one unit system into another.
///
/// __Syntax__: 
/// ```ods
///     CONVERT( N: Number; From: Text; Into: Text )
/// ```
///
/// __Constraints__:
/// From and Into shall be legal units, and shall be in the same unit group.
///
/// __Semantics__:
/// Returns the number converted from the unit identified by From into the unit 
/// identified by Into. A unit is a unit symbol , optionally preceded by a unit 
/// prefix (either a decimal prefix or a binary prefix, as specified in Table 
/// 25 - Decimal Prefixes for use in CONVERT and Table 26 - Binary prefixes for 
/// use in CONVERT respectively). Units (including both the unit symbol and the 
/// optional unit prefix) are case-sensitive.
/// 
/// Evaluators claiming to implement this function shall support at least the 
/// following unit symbols (with conversions between them and other units in 
/// the same group):
/// 
/// If a conversion factor (as listed above) is not exact, an implementation 
/// may use a more accurate conversion factor instead.
/// 
/// Implementation-defined unit names should contain a 'FULL STOP' (U+002E) 
/// character.
/// 
/// Evaluators shall support decimal prefixes for unit symbols marked with * 
/// and binary prefixes for unit symbols marked with †. Evaluators should not 
/// support prefixes for other unit symbols.
/// 
/// The unit symbols in parentheses are deprecated unit symbols; evaluators 
/// shall support these unit symbols.
/// 
/// Evaluators should use internationally-standardized unit name abbreviations 
/// for such additions where possible. Evaluators may support the obsolete 
/// symbols “p” and “P” as unit names for Pascals.
/// 
/// For purposes of this function, a year is exactly 365.25 days long.
/// 
/// Evaluators claiming to support this function shall permit the unit decimal 
/// prefixes specified in Table 25 - Decimal Prefixes for use in CONVERT to be 
/// prepended to any unit symbol marked with * in Table 24 - Unit names. Adding 
/// a unit prefix indicates multiplication of the (scalar) unit by the given 
/// prefix value; for example km indicates kilometres, and km2 or km^2 indicate 
/// square kilometres.
///
/// __Note__:
/// The prefix “e” for 10 1 is nonstandard and included for backward 
/// compatibility with legacy applications and documents.
/// 
/// The unit names marked with † in Table 24 - Unit names (see the 
/// Information Unit group) shall also support the following binary prefixes 
/// per IEC 60027-2:
/// 
/// In the case where there is a naming conflict (a unit name with a prefix is 
/// the same as an unprefixed name), the unprefixed name shall take precedence.
/// 
/// Evaluators may implement this conversion by first converting to some SI 
/// unit (e.g., meter and kilogram), and then convert again to the final unit.
///
/// __See also__: "EUROCONVERT", 
#[inline]
pub fn convert<A: Number>(n: A, from: ConvertUnit, into: ConvertUnit) -> FnNumber3<A, ConvertUnit, ConvertUnit> {
    FnNumber3("CONVERT", n, from, into)
}

/// Return the cosine of an angle specified in radians.
///
/// __Syntax__: 
/// ```ods
///     COS( N: Number )
/// ```
///
/// __Constraints__:
/// None
///
/// __Semantics__:
/// Computes the cosine of an angle specified in radians.
///
/// __See also__: "ACOS", "RADIANS", "DEGREES", 
#[inline]
pub fn cos<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("COS", n)
}

/// Return the hyperbolic cosine of the given hyperbolic angle.
///
/// __Syntax__: 
/// ```ods
///     COSH( N: Number )
/// ```
///
/// __Constraints__:
/// None
///
/// __Semantics__:
/// Computes the hyperbolic cosine of a hyperbolic angle. The hyperbolic cosine 
/// is an analog of the ordinary (circular) cosine. The points (cosh t, sinh t) 
/// define the right half of the equilateral hyperbola, just as the points (cos 
/// t, sin t) define the points of a circle.
///
/// __See also__: "ACOSH", "SINH", "TANH", 
#[inline]
pub fn cosh<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("COSH", n)
}

/// Return the cotangent of an angle specified in radians.
///
/// __Syntax__: 
/// ```ods
///     COT( N: Number )
/// ```
///
/// __Constraints__:
/// None
///
/// __Semantics__:
/// Computes the cotangent of an angle specified in radians.
/// 
/// COT(x) = 1 / TAN(x)
///
/// __See also__: "ACOT", "TAN", "RADIANS", "DEGREES", "SIN", "COS", 
#[inline]
pub fn cot<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("COT", n)
}

/// Return the hyperbolic cotangent of the given hyperbolic angle.
///
/// __Syntax__: 
/// ```ods
///     COTH( N: Number )
/// ```
///
/// __Constraints__:
/// N ≠ 0
///
/// __Semantics__:
/// Computes the hyperbolic cotangent of a hyperbolic angle. The hyperbolic 
/// cotangent is an analog of the ordinary (circular) cotangent.
///
/// __See also__: "ACOSH", "COSH", "SINH", "TANH", 
#[inline]
pub fn coth<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("COTH", n)
}

/// Return the cosecant of an angle specified in radians.
///
/// __Syntax__: 
/// ```ods
///     CSC( N: Number )
/// ```
///
/// __Constraints__:
/// None
///
/// __Semantics__:
/// Computes the cosecant cosine of an angle specified in radians. Equivalent 
/// to:
/// 
/// 1 / SIN(N)
///
/// __See also__: "SIN", 
#[inline]
pub fn csc<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("CSC", n)
}

/// Return the hyperbolic cosecant of the given angle specified in radians.
///
/// __Syntax__: 
/// ```ods
///     CSCH( N: Number )
/// ```
///
/// __Constraints__:
/// None
///
/// __Semantics__:
/// Computes the hyperbolic cosecant of a hyperbolic angle. This is equivalent 
/// to:
/// 
/// 1 / SINH(N)
///
/// __See also__: "SINH", 
#[inline]
pub fn csch<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("CSCH", n)
}

/// Convert radians to degrees.
///
/// __Syntax__: 
/// ```ods
///     DEGREES( N: Number )
/// ```
///
/// __Constraints__:
/// None
///
/// __Semantics__:
/// Converts a number in radians into a number in degrees. DEGREES(N) is equal 
/// to N * 180 / π.
///
/// __See also__: "RADIANS", "PI", 
#[inline]
pub fn degrees<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("DEGREES", n)
}

/// Report if two numbers are equal, returns 1 if they are equal.
///
/// __Syntax__: 
/// ```ods
///     DELTA( X: Number )
/// ```
///
/// __Constraints__:
/// None
///
/// __Semantics__:
/// If X and Y are equal, return 1, else 0. Y is set to 0 if omitted.
///
/// __See also__: "Infix operator “=”", 
#[inline]
pub fn delta<A: Number>(x: A) -> FnNumber1<A> {
    FnNumber1("DELTA", x)
}

/// Report if two numbers are equal, returns 1 if they are equal.
///
/// __Syntax__: 
/// ```ods
///     DELTA( X: Number; Y: Number )
/// ```
///
/// __Constraints__:
/// None
///
/// __Semantics__:
/// If X and Y are equal, return 1, else 0. Y is set to 0 if omitted.
///
/// __See also__: "Infix operator “=”", 
#[inline]
pub fn delta_<A: Number, B: Number>(x: A, y: B) -> FnNumber2<A, B> {
    FnNumber2("DELTA", x, y)
}

/// Calculates the error function.
///
/// __Syntax__: 
/// ```ods
///     ERF( Z0: Number )
/// ```
///
/// __Constraints__:
/// None
///
/// __Semantics__:
/// With a single argument, returns the error function of Z0:
/// 
/// With two arguments, returns
///
/// __See also__: "ERFC", 
#[inline]
pub fn erf<A: Number>(z0: A) -> FnNumber1<A> {
    FnNumber1("ERF", z0)
}

/// Calculates the error function.
///
/// __Syntax__: 
/// ```ods
///     ERF( Z0: Number; Z1: Number )
/// ```
///
/// __Constraints__:
/// None
///
/// __Semantics__:
/// With a single argument, returns the error function of Z0:
/// 
/// With two arguments, returns
///
/// __See also__: "ERFC", 
#[inline]
pub fn erf_<A: Number, B: Number>(z0: A, z1: B) -> FnNumber2<A, B> {
    FnNumber2("ERF", z0, z1)
}

/// Calculates the complementary error function.
///
/// __Syntax__: 
/// ```ods
///     ERFC( Z: Number )
/// ```
///
/// __Constraints__:
/// None
///
/// __Semantics__:
/// returns the complementary error function of Z: ERFC(Z) = 1 – ERF(Z)
///
/// __See also__: "ERF", 
#[inline]
pub fn erfc<A: Number>(z: A) -> FnNumber1<A> {
    FnNumber1("ERFC", z)
}

/// Converts a Number, representing a value in one European currency, to an 
/// equivalent value in another European currency, according to the fixed 
/// conversion rates defined by the Council of the European Union.
///
/// __Syntax__: 
/// ```ods
///     EUROCONVERT( N: Number; From: Text; To: Text )
/// ```
///
/// __Constraints__:
/// From and To shall be known to the evaluator. TriangulationPrecision shall 
/// be ≥ 3, if not omitted.
/// 
/// If an evaluator does not support the parameters FullPrecision and 
/// TriangulationPrecision, FullPrecision should be assumed to be false.
///
/// __Semantics__:
/// Returns the given money value of a conversion from From currency into To 
/// currency. Both From and To shall be the official [ISO4217] abbreviation for 
/// the given currency; note that these are in upper case, but the function 
/// accepts lower case or mixed case as well. If From and To are equal 
/// currencies, the value N is returned, no precision or triangulation is 
/// applied.
/// 
/// As new member countries adopt the Euro, new conversion rates will become 
/// active and evaluators may add them using the respective [ISO4217] codes and 
/// fixed rates as defined by the European Council, on the basis of a European 
/// Commission proposal.
///
/// __Note__:
/// 
/// The European Commission's Euro entry page is http://ec.europa.eu/euro/
/// The conversion rates and triangulation rules are available at 
/// http://ec.europa.eu/economy_finance/euro/adoption/conversion/index_en.htm 
/// with links to the European Council Regulation legal documents at the 
/// http://eur-lex.europa.eu/ European Union law database server.
/// 
/// If FullPrecision is omitted or FALSE, the result is rounded according to 
/// the decimals of the To currency. If FullPrecision is TRUE the result is not 
/// rounded.
/// 
/// If TriangulationPrecision is given and ≥ 3, the intermediate result of a 
/// triangular conversion (currency1,EUR,currency2) is rounded to that 
/// precision. If TriangulationPrecision is omitted, the intermediate result is 
/// not rounded. Also if To currency is “EUR”, TriangulationPrecision 
/// precision is used as if triangulation was needed and conversion from EUR to 
/// EUR was applied.
///
/// __See also__: "CONVERT", 
#[inline]
pub fn euroconvert<A: Number, B: Text, C: Text>(n: A, from: B, to: C) -> FnNumber3<A, B, C> {
    FnNumber3("EUROCONVERT", n, from, to)
}

/// Converts a Number, representing a value in one European currency, to an 
/// equivalent value in another European currency, according to the fixed 
/// conversion rates defined by the Council of the European Union.
///
/// __Syntax__: 
/// ```ods
///     EUROCONVERT( N: Number; From: Text; To: Text; FullPrecision: Logical )
/// ```
///
/// __Constraints__:
/// From and To shall be known to the evaluator. TriangulationPrecision shall 
/// be ≥ 3, if not omitted.
/// 
/// If an evaluator does not support the parameters FullPrecision and 
/// TriangulationPrecision, FullPrecision should be assumed to be false.
///
/// __Semantics__:
/// Returns the given money value of a conversion from From currency into To 
/// currency. Both From and To shall be the official [ISO4217] abbreviation for 
/// the given currency; note that these are in upper case, but the function 
/// accepts lower case or mixed case as well. If From and To are equal 
/// currencies, the value N is returned, no precision or triangulation is 
/// applied.
/// 
/// As new member countries adopt the Euro, new conversion rates will become 
/// active and evaluators may add them using the respective [ISO4217] codes and 
/// fixed rates as defined by the European Council, on the basis of a European 
/// Commission proposal.
///
/// __Note__:
/// 
/// The European Commission's Euro entry page is http://ec.europa.eu/euro/
/// The conversion rates and triangulation rules are available at 
/// http://ec.europa.eu/economy_finance/euro/adoption/conversion/index_en.htm 
/// with links to the European Council Regulation legal documents at the 
/// http://eur-lex.europa.eu/ European Union law database server.
/// 
/// If FullPrecision is omitted or FALSE, the result is rounded according to 
/// the decimals of the To currency. If FullPrecision is TRUE the result is not 
/// rounded.
/// 
/// If TriangulationPrecision is given and ≥ 3, the intermediate result of a 
/// triangular conversion (currency1,EUR,currency2) is rounded to that 
/// precision. If TriangulationPrecision is omitted, the intermediate result is 
/// not rounded. Also if To currency is “EUR”, TriangulationPrecision 
/// precision is used as if triangulation was needed and conversion from EUR to 
/// EUR was applied.
///
/// __See also__: "CONVERT", 
#[inline]
pub fn euroconvert_<A: Number, B: Text, C: Text, D: Logical>(n: A, from: B, to: C, full_precision: D) -> FnNumber4<A, B, C, D> {
    FnNumber4("EUROCONVERT", n, from, to, full_precision)
}

/// Converts a Number, representing a value in one European currency, to an 
/// equivalent value in another European currency, according to the fixed 
/// conversion rates defined by the Council of the European Union.
///
/// __Syntax__: 
/// ```ods
///     EUROCONVERT( N: Number; From: Text; To: Text; FullPrecision: Logical; TriangulationPrecision: Integer )
/// ```
///
/// __Constraints__:
/// From and To shall be known to the evaluator. TriangulationPrecision shall 
/// be ≥ 3, if not omitted.
/// 
/// If an evaluator does not support the parameters FullPrecision and 
/// TriangulationPrecision, FullPrecision should be assumed to be false.
///
/// __Semantics__:
/// Returns the given money value of a conversion from From currency into To 
/// currency. Both From and To shall be the official [ISO4217] abbreviation for 
/// the given currency; note that these are in upper case, but the function 
/// accepts lower case or mixed case as well. If From and To are equal 
/// currencies, the value N is returned, no precision or triangulation is 
/// applied.
/// 
/// As new member countries adopt the Euro, new conversion rates will become 
/// active and evaluators may add them using the respective [ISO4217] codes and 
/// fixed rates as defined by the European Council, on the basis of a European 
/// Commission proposal.
///
/// __Note__:
/// 
/// The European Commission's Euro entry page is http://ec.europa.eu/euro/
/// The conversion rates and triangulation rules are available at 
/// http://ec.europa.eu/economy_finance/euro/adoption/conversion/index_en.htm 
/// with links to the European Council Regulation legal documents at the 
/// http://eur-lex.europa.eu/ European Union law database server.
/// 
/// If FullPrecision is omitted or FALSE, the result is rounded according to 
/// the decimals of the To currency. If FullPrecision is TRUE the result is not 
/// rounded.
/// 
/// If TriangulationPrecision is given and ≥ 3, the intermediate result of a 
/// triangular conversion (currency1,EUR,currency2) is rounded to that 
/// precision. If TriangulationPrecision is omitted, the intermediate result is 
/// not rounded. Also if To currency is “EUR”, TriangulationPrecision 
/// precision is used as if triangulation was needed and conversion from EUR to 
/// EUR was applied.
///
/// __See also__: "CONVERT", 
#[inline]
pub fn euroconvert__<A: Number, B: Text, C: Text, D: Logical, E: Number>(n: A, from: B, to: C, full_precision: D, triangulation_precision: E) -> FnNumber5<A, B, C, D, E> {
    FnNumber5("EUROCONVERT", n, from, to, full_precision, triangulation_precision)
}

/// Rounds a number up to the nearest even integer. Rounding is away from zero.
///
/// __Syntax__: 
/// ```ods
///     EVEN( N: Number )
/// ```
///
/// __Constraints__:
/// None
///
/// __Semantics__:
/// Returns the even integer whose sign is the same as N's and whose absolute 
/// value is greater than or equal to the absolute value of N.
///
/// __See also__: "ODD", 
#[inline]
pub fn even<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("EVEN", n)
}

/// Returns e raised by the given number.
///
/// __Syntax__: 
/// ```ods
///     EXP( X: Number )
/// ```
///
/// __Constraints__:
/// None
///
/// __Semantics__:
/// Computes
///
/// __See also__: "LOG", "LN", 
#[inline]
pub fn exp<A: Number>(x: A) -> FnNumber1<A> {
    FnNumber1("EXP", x)
}

/// Return factorial (!).
///
/// __Syntax__: 
/// ```ods
///     FACT( F: Integer )
/// ```
///
/// __Constraints__:
/// F ≥ 0
///
/// __Semantics__:
/// Return the factorial
/// 
/// F(0) = F(1) = 1.
///
/// __See also__: "Infix Operator \"*\"", "GAMMA", 
#[inline]
pub fn fact<A: Number>(f: A) -> FnNumber1<A> {
    FnNumber1("FACT", f)
}

/// Returns double factorial (!!).
///
/// __Syntax__: 
/// ```ods
///     FACTDOUBLE( F: Integer )
/// ```
///
/// __Constraints__:
/// F ≥ 0
///
/// __Semantics__:
/// Return
/// 
/// Double factorial is computed by multiplying every other number in the 1..N 
/// range, with N always being included.
///
/// __See also__: "Infix Operator \"*\"", "GAMMA", "FACT", 
#[inline]
pub fn factdouble<A: Number>(f: A) -> FnNumber1<A> {
    FnNumber1("FACTDOUBLE", f)
}

/// Return gamma function value.
///
/// __Syntax__: 
/// ```ods
///     GAMMA( N: Number )
/// ```
///
/// __Constraints__:
/// N ≠ 0 and N not a negative integer.
///
/// __Semantics__:
/// Return
/// 
/// with Γ(N + 1) = N * Γ(N). Note that for non-negative integers N, Γ(N + 
/// 1) = N! = FACT(N). Note that GAMMA can accept non-integers.
///
/// __See also__: "FACT", 
#[inline]
pub fn gamma<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("GAMMA", n)
}

/// Returns the natural logarithm of the GAMMA function.
///
/// __Syntax__: 
/// ```ods
///     GAMMALN( X: Number )
/// ```
///
/// __Constraints__:
/// For each X, X > 0
///
/// __Semantics__:
/// Returns the same value as LN(GAMMA(X))
///
/// __See also__: "GAMMA", "FACT", 
#[inline]
pub fn gammaln<A: Number>(x: A) -> FnNumber1<A> {
    FnNumber1("GAMMALN", x)
}

/// Returns the greatest common divisor (GCD)
///
/// __Syntax__: 
/// ```ods
///     GCD({ X: NumberSequenceList}+ )
/// ```
///
/// __Constraints__:
/// For all a in X: INT(a) ≥ 0 and for at least one a in X: INT(a) > 0
///
/// __Semantics__:
/// Return the largest integer N such that for every a in X: INT(a) is a 
/// multiple of N.
///
/// __Note__:
/// If for all a in X: INT(a) = 0 the return value is implementation-defined 
/// but is either an Error or 0.
///
/// __See also__: "LCM", "INT", 
#[inline]
pub fn gcd<A: Sequence>(x: A) -> FnNumber1<A> {
    FnNumber1("GCD", x)
}

/// Returns 1 if a number is greater than or equal to another number, else 
/// returns 0.
///
/// __Syntax__: 
/// ```ods
///     GESTEP( X: Number )
/// ```
///
/// __Semantics__:
/// Number X is tested against number Step. If greater or equal 1 is returned, 
/// else 0. The second parameter is assumed 0 if omitted. If one of the 
/// parameters is not a Number, the function results in an Error.
#[inline]
pub fn gestep<A: Number>(x: A) -> FnNumber1<A> {
    FnNumber1("GESTEP", x)
}

/// Returns 1 if a number is greater than or equal to another number, else 
/// returns 0.
///
/// __Syntax__: 
/// ```ods
///     GESTEP( X: Number; Step: Number )
/// ```
///
/// __Semantics__:
/// Number X is tested against number Step. If greater or equal 1 is returned, 
/// else 0. The second parameter is assumed 0 if omitted. If one of the 
/// parameters is not a Number, the function results in an Error.
#[inline]
pub fn gestep_<A: Number, B: Number>(x: A, step: B) -> FnNumber2<A, B> {
    FnNumber2("GESTEP", x, step)
}

/// Returns the least common multiplier
///
/// __Syntax__: 
/// ```ods
///     LCM({ X: NumberSequenceList}+ )
/// ```
///
/// __Constraints__:
/// For all in X: INT(X) = X, X ≥ 0
///
/// __Semantics__:
/// Return the smallest integer that is the multiple of the given values. Each 
/// value has INT applied to it first. Note that if given two numbers, ABS(a * 
/// b) = LCM(a;b) * GCD(a;b).
///
/// __See also__: "GCD", "INT", 
#[inline]
pub fn lcm<A: Sequence>(x: A) -> FnNumber1<A> {
    FnNumber1("LCM", x)
}

/// Return the natural logarithm of a number.
///
/// __Syntax__: 
/// ```ods
///     LN( X: Number )
/// ```
///
/// __Constraints__:
/// X > 0
///
/// __Semantics__:
/// Computes the natural logarithm (base e) of the given number.
///
/// __See also__: "LOG", "LOG10", "POWER", "EXP", 
#[inline]
pub fn ln<A: Number>(x: A) -> FnNumber1<A> {
    FnNumber1("LN", x)
}

/// Return the logarithm of a number in a specified base.
///
/// __Syntax__: 
/// ```ods
///     LOG( N: Number )
/// ```
///
/// __Constraints__:
/// N > 0
///
/// __Semantics__:
/// Computes the logarithm of a number in the specified base. Note that if the 
/// base is not specified, the logarithm base 10 is returned.
///
/// __See also__: "LOG10", "LN", "POWER", "EXP", 
#[inline]
pub fn log<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("LOG", n)
}

/// Return the logarithm of a number in a specified base.
///
/// __Syntax__: 
/// ```ods
///     LOG( N: Number; Base: Number )
/// ```
///
/// __Constraints__:
/// N > 0
///
/// __Semantics__:
/// Computes the logarithm of a number in the specified base. Note that if the 
/// base is not specified, the logarithm base 10 is returned.
///
/// __See also__: "LOG10", "LN", "POWER", "EXP", 
#[inline]
pub fn log_<A: Number, B: Number>(n: A, base: B) -> FnNumber2<A, B> {
    FnNumber2("LOG", n, base)
}

/// Return the base 10 logarithm of a number.
///
/// __Syntax__: 
/// ```ods
///     LOG10( N: Number )
/// ```
///
/// __Constraints__:
/// N > 0
///
/// __Semantics__:
/// Computes the base 10 logarithm of a number.
///
/// __See also__: "LOG", "LN", "POWER", "EXP", 
#[inline]
pub fn log10<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("LOG10", n)
}

/// Return the remainder when one number is divided by another number.
///
/// __Syntax__: 
/// ```ods
///     MOD( A: Number; B: Number )
/// ```
///
/// __Constraints__:
/// B != 0
///
/// __Semantics__:
/// Computes the remainder of A / B. The remainder has the same sign as B.
///
/// __See also__: "Infix Operator \"/\"", "QUOTIENT", 
#[inline]
pub fn mod_<A: Number, B: Number>(a: A, b: B) -> FnNumber2<A, B> {
    FnNumber2("MOD", a, b)
}

/// Returns the multinomial for the given values.
///
/// __Syntax__: 
/// ```ods
///     MULTINOMIAL({ A: NumberSequence}+ )
/// ```
///
/// __Constraints__:
/// None
///
/// __Semantics__:
/// Returns the multinomial of the sequence A = (a1, a2, ..., an). Multinomial 
/// is defined as FACT(a1 + a2 +...+ an) / (FACT(a1) * FACT(a2) *...* FACT(an))
///
/// __See also__: "FACT", 
#[inline]
pub fn multinomial<A: Sequence>(a: A) -> FnNumber1<A> {
    FnNumber1("MULTINOMIAL", a)
}

/// Rounds a number up to the nearest odd integer, where "up" means "away from 
/// 0".
///
/// __Syntax__: 
/// ```ods
///     ODD( N: Number )
/// ```
///
/// __Constraints__:
/// None
///
/// __Semantics__:
/// Returns the odd integer whose sign is the same as N's and whose absolute 
/// value is greater than or equal to the absolute value of N. In other words, 
/// any "rounding" is away from zero. By definition, ODD(0) is 1.
///
/// __See also__: "EVEN", 
#[inline]
pub fn odd<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("ODD", n)
}

/// Return the approximate value of π.
///
/// __Syntax__: 
/// ```ods
///     PI( )
/// ```
///
/// __Constraints__:
/// None.
///
/// __Semantics__:
/// This function takes no arguments and returns the (approximate) value of π 
/// (pi). Evaluators should use the closest possible numerical representation 
/// that is possible in their representation of numbers.
///
/// __See also__: "SIN", "COS", 
#[inline]
pub fn pi() -> FnNumber0 {
    FnNumber0("PI", )
}

/// Return the value of one number raised to the power of another number.
///
/// __Syntax__: 
/// ```ods
///     POWER( A: Number; B: Number )
/// ```
///
/// __Constraints__:
/// None
///
/// __Semantics__:
/// Computes A raised to the power B.
/// 
/// •POWER(0,0) is implementation-defined, but shall be one of 0,1, or an 
/// Error.
/// 
/// •POWER(0,B), where B < 0, shall return an Error.
/// 
/// •POWER(A,B), where A ≤ 0 and INT(B) != B, is implementation-defined.
///
/// __See also__: "LOG", "LOG10", "LN", "EXP", 
#[inline]
pub fn power<A: Number, B: Number>(a: A, b: B) -> FnNumber2<A, B> {
    FnNumber2("POWER", a, b)
}

/// Multiply the set of numbers, including all numbers inside ranges.
///
/// __Syntax__: 
/// ```ods
///     PRODUCT({ N: NumberSequenceList}+ )
/// ```
///
/// __Constraints__:
/// None
///
/// __Semantics__:
/// Returns the product of the Numbers (and only the Numbers, i.e., not Text 
/// inside ranges).
///
/// __See also__: "SUM", 
#[inline]
pub fn product<A: Sequence>(n: A) -> FnNumber1<A> {
    FnNumber1("PRODUCT", n)
}

/// Return the integer portion of a division.
///
/// __Syntax__: 
/// ```ods
///     QUOTIENT( A: Number; B: Number )
/// ```
///
/// __Constraints__:
/// B ≠ 0
///
/// __Semantics__:
/// Return the integer portion of a division.
///
/// __See also__: "MOD", 
#[inline]
pub fn quotient<A: Number, B: Number>(a: A, b: B) -> FnNumber2<A, B> {
    FnNumber2("QUOTIENT", a, b)
}

/// Convert degrees to radians.
///
/// __Syntax__: 
/// ```ods
///     RADIANS( N: Number )
/// ```
///
/// __Constraints__:
/// None
///
/// __Semantics__:
/// Converts a number in degrees into a number in radians. RADIANS(N) is equal 
/// to N * PI() / 180.
///
/// __See also__: "DEGREES", "PI", 
#[inline]
pub fn radians<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("RADIANS", n)
}

/// Return a random number between 0 (inclusive) and 1 (exclusive).
///
/// __Syntax__: 
/// ```ods
///     RAND( )
/// ```
///
/// __Semantics__:
/// This function takes no arguments and returns a random number between 0 
/// (inclusive) and 1 (exclusive). Note that unlike most functions, this 
/// function will typically return different values when called each time with 
/// the same (empty set of) parameters.
///
/// __See also__: "RANDBETWEEN", 
#[inline]
pub fn rand() -> FnNumber0 {
    FnNumber0("RAND", )
}

/// Return a random integer number between A and B.
///
/// __Syntax__: 
/// ```ods
///     RANDBETWEEN( A: Integer; B: Integer )
/// ```
///
/// __Constraints__:
/// A ≤ B
///
/// __Semantics__:
/// The function returns a random integer number between A and B inclusive. 
/// Note that unlike most functions, this function will often return different 
/// values when called each time with the same parameters.
///
/// __See also__: "RAND", 
#[inline]
pub fn randbetween<A: Number, B: Number>(a: A, b: B) -> FnNumber2<A, B> {
    FnNumber2("RANDBETWEEN", a, b)
}

/// Return the secant of an angle specified in radians.
///
/// __Syntax__: 
/// ```ods
///     SEC( N: Number )
/// ```
///
/// __Constraints__:
/// None
///
/// __Semantics__:
/// Computes the secant cosine of an angle specified in radians. Equivalent to:
/// 
/// 1 / COS(N)
///
/// __See also__: "SIN", 
#[inline]
pub fn sec<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("SEC", n)
}

/// Returns the sum of a power series.
///
/// __Syntax__: 
/// ```ods
///     SERIESSUM( X: Number; N: Number; M: Number; Coefficients: Array )
/// ```
///
/// __Arguments__:
/// 
/// •X: the independent variable of the power series.
/// 
/// •N: the initial power to which X is to be raised.
/// 
/// •M: the increment by which to increase N for each term in the series.
/// 
/// •Coefficients: a set of coefficients by which each successive power of 
/// the variable X is multiplied.
///
/// __Constraints__:
/// 
/// All elements of Coefficients are of type Number.
/// 
/// X ≠ 0 if any of the exponents, which are generated from N and M, are 
/// negative.
///
/// __Semantics__:
/// Returns a sum of powers of the number X.
/// 
/// With C being the number of coefficients the function is computed as:
/// 
/// If X = 0 and all of the exponents are non-negative then
/// shall be set to 1 and
/// shall be set to 0.
#[inline]
pub fn seriessum<A: Number, B: Number, C: Number, D: Array>(x: A, n: B, m: C, coefficients: D) -> FnNumber4<A, B, C, D> {
    FnNumber4("SERIESSUM", x, n, m, coefficients)
}

/// Return the sign of a number.
///
/// __Syntax__: 
/// ```ods
///     SIGN( N: Number )
/// ```
///
/// __Constraints__:
/// None
///
/// __Semantics__:
/// If N < 0, returns -1; if N > 0, returns +1; if N = 0, returns 0.
///
/// __See also__: "ABS", 
#[inline]
pub fn sign<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("SIGN", n)
}

/// Return the sine of an angle specified in radians.
///
/// __Syntax__: 
/// ```ods
///     SIN( N: Number )
/// ```
///
/// __Constraints__:
/// None
///
/// __Semantics__:
/// Computes the sine of an angle specified in radians.
///
/// __See also__: "ASIN", "RADIANS", "DEGREES", 
#[inline]
pub fn sin<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("SIN", n)
}

/// Return the hyperbolic sine of the given hyperbolic angle.
///
/// __Syntax__: 
/// ```ods
///     SINH( N: Number )
/// ```
///
/// __Constraints__:
/// None
///
/// __Semantics__:
/// Computes the hyperbolic sine of a hyperbolic angle. The hyperbolic sine is 
/// an analog of the ordinary (circular) sine. The points (cosh t, sinh t) 
/// define the right half of the equilateral hyperbola, just as the points (cos 
/// t, sin t) define the points of a circle.
///
/// __See also__: "ASINH", "COSH", "TANH", 
#[inline]
pub fn sinh<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("SINH", n)
}

/// Return the hyperbolic secant of the given angle specified in radians.
///
/// __Syntax__: 
/// ```ods
///     SECH( N: Number )
/// ```
///
/// __Constraints__:
/// None
///
/// __Semantics__:
/// Computes the hyperbolic secant of a hyperbolic angle. This is equivalent 
/// to:
/// 
/// 1 / COSH(N)
///
/// __See also__: "SINH", "COSH", "CSCH", 
#[inline]
pub fn sech<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("SECH", n)
}

/// Return the square root of a number.
///
/// __Syntax__: 
/// ```ods
///     SQRT( N: Number )
/// ```
///
/// __Constraints__:
/// N ≥ 0
///
/// __Semantics__:
/// Returns the square root of a non-negative number. This function shall 
/// produce an Error if given a negative number; for producing complex numbers, 
/// see IMSQRT.
///
/// __See also__: "POWER", "IMSQRT", "SQRTPI", 
#[inline]
pub fn sqrt<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("SQRT", n)
}

/// Return the square root of a number multiplied by π (pi).
///
/// __Syntax__: 
/// ```ods
///     SQRTPI( N: Number )
/// ```
///
/// __Constraints__:
/// N ≥ 0
///
/// __Semantics__:
/// Returns the square root of a non-negative number after it was first 
/// multiplied by π, that is, SQRT(N * PI()). This function shall produce an 
/// Error if given a negative number; for producing complex numbers, see 
/// IMSQRT.
///
/// __See also__: "POWER", "SQRT", "PI", "IMSQRT", 
#[inline]
pub fn sqrtpi<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("SQRTPI", n)
}

/// Evaluates a function on a range.
///
/// __Syntax__: 
/// ```ods
///     SUBTOTAL( Function: Integer; Sequence: NumberSequence )
/// ```
///
/// __Constraints__:
/// None
///
/// __Semantics__:
/// Computes a given function on a number sequence. The function is denoted by 
/// the first parameter: The difference from standard functions is that all 
/// members of the sequence are excluded which:
/// 
/// •include a call to SUBTOTAL in their formula
/// 
/// •are in a row that is hidden by a table:visibility=”filter” attribute 
/// of the <table:table-row> element (OpenDocument, Part 3, 19.754).
/// 
/// •are in a row that is hidden by a table:visibility=”collapse” 
/// attribute of the <table:table-row> element if the function ID is one of 
/// 101...111.
///
/// __See also__: "SUM", "AVERAGE", 
#[inline]
pub fn subtotal<A: Sequence>(function: SubtotalFunction, sequence: A) -> FnNumber2<SubtotalFunction, A> {
    FnNumber2("SUBTOTAL", function, sequence)
}

/// Sum (add) the set of numbers, including all numbers in ranges.
///
/// __Syntax__: 
/// ```ods
///     SUM({ N: NumberSequenceList}+ )
/// ```
///
/// __Constraints__:
/// N != {}; Evaluators may evaluate expressions that do not meet this 
/// constraint.
///
/// __Semantics__:
/// Adds Numbers (and only Numbers) together (see the text on conversions).
///
/// __See also__: "AVERAGE", 
#[inline]
pub fn sum<A: Sequence>(n: A) -> FnNumber1<A> {
    FnNumber1("SUM", n)
}

/// Sum the values of cells in a range that meet a criteria.
///
/// __Syntax__: 
/// ```ods
///     SUMIF( R: ReferenceList|Reference; C: Criterion )
/// ```
///
/// __Constraints__:
/// Does not accept constant values as the range parameter.
///
/// __Semantics__:
/// Sums the values of type Number in the range R or S that meet the Criterion 
/// C (4.11.8).
/// 
/// If S is not given, R may be a reference list. If S is given, R shall not be 
/// a reference list with more than 1 references and an Error be generated if 
/// it was.
/// 
/// If the optional range S is included, then the values of S starting from the 
/// top left cell and matching the geometry of R (same number of rows and 
/// columns) are summed if the corresponding value in R meets the Criterion. 
/// The actual range S is not considered. If the resulting range exceeds the 
/// sheet bounds, column numbers larger than the maximum column and row numbers 
/// larger than the maximum row are silently ignored, no Error is generated for 
/// this case.
/// 
/// The values returned may vary depending upon the 
/// HOST-USE-REGULAR-EXPRESSIONS or HOST-USE-WILDCARDS or 
/// HOST-SEARCH-CRITERIA-MUST-APPLY-TO-WHOLE-CELL properties. 3.4
///
/// __See also__: "COUNTIF", "SUM", "Infix Operator \"=\"", "Infix Operator \"<>\"", "Infix Operator Ordered Comparison (\"<\", \"<=\", \">\", \">=\")", 
#[inline]
pub fn sumif<A: Reference, B: Criterion>(r: A, c: B) -> FnNumber2<A, B> {
    FnNumber2("SUMIF", r, c)
}

/// Sum the values of cells in a range that meet a criteria.
///
/// __Syntax__: 
/// ```ods
///     SUMIF( R: ReferenceList|Reference; C: Criterion; S: Reference )
/// ```
///
/// __Constraints__:
/// Does not accept constant values as the range parameter.
///
/// __Semantics__:
/// Sums the values of type Number in the range R or S that meet the Criterion 
/// C (4.11.8).
/// 
/// If S is not given, R may be a reference list. If S is given, R shall not be 
/// a reference list with more than 1 references and an Error be generated if 
/// it was.
/// 
/// If the optional range S is included, then the values of S starting from the 
/// top left cell and matching the geometry of R (same number of rows and 
/// columns) are summed if the corresponding value in R meets the Criterion. 
/// The actual range S is not considered. If the resulting range exceeds the 
/// sheet bounds, column numbers larger than the maximum column and row numbers 
/// larger than the maximum row are silently ignored, no Error is generated for 
/// this case.
/// 
/// The values returned may vary depending upon the 
/// HOST-USE-REGULAR-EXPRESSIONS or HOST-USE-WILDCARDS or 
/// HOST-SEARCH-CRITERIA-MUST-APPLY-TO-WHOLE-CELL properties. 3.4
///
/// __See also__: "COUNTIF", "SUM", "Infix Operator \"=\"", "Infix Operator \"<>\"", "Infix Operator Ordered Comparison (\"<\", \"<=\", \">\", \">=\")", 
#[inline]
pub fn sumif_<A: Reference, B: Criterion, C: Reference>(r: A, c: B, s: C) -> FnNumber3<A, B, C> {
    FnNumber3("SUMIF", r, c, s)
}

/// Returns the sum of the products of the matrix elements.
///
/// __Syntax__: 
/// ```ods
///     SUMPRODUCT({ A: Array}+ )
/// ```
///
/// __Constraints__:
/// All matrices shall have the same dimensions.
///
/// __Semantics__:
/// Multiplies the corresponding elements of all matrices and returns the sum 
/// of them.
/// 
/// where
/// denotes an element of the matrix
/// .
#[inline]
pub fn sumproduct<A: Sequence>(a: A) -> FnNumber1<A> {
    FnNumber1("SUMPRODUCT", a)
}

/// Sum (add) the set of squares of numbers, including all numbers in ranges
///
/// __Syntax__: 
/// ```ods
///     SUMSQ({ N: NumberSequence}+ )
/// ```
///
/// __Constraints__:
/// N != {}; Evaluators may evaluate expressions that do not meet this 
/// constraint.
///
/// __Semantics__:
/// Adds squares of Numbers (and only Numbers) together. See the text on 
/// conversions.
#[inline]
pub fn sumsq<A: Sequence>(n: A) -> FnNumber1<A> {
    FnNumber1("SUMSQ", n)
}

/// Returns the sum of the difference between the squares of the matrices A and 
/// B.
///
/// __Syntax__: 
/// ```ods
///     SUMX2MY2( A: Array; B: Array )
/// ```
///
/// __Constraints__:
/// Both matrices shall have the same dimensions.
///
/// __Semantics__:
/// Sums up the differences of the corresponding elements squares for two 
/// matrices.
#[inline]
pub fn sumx2my2<A: Array, B: Array>(a: A, b: B) -> FnNumber2<A, B> {
    FnNumber2("SUMX2MY2", a, b)
}

/// Returns the total sum of the squares of the matrices A and B.
///
/// __Syntax__: 
/// ```ods
///     SUMX2PY2( A: Array; B: Array )
/// ```
///
/// __Constraints__:
/// Both matrices shall have the same dimensions.
///
/// __Semantics__:
/// Sums up the squares of each element of the two matrices.
#[inline]
pub fn sumx2py2<A: Array, B: Array>(a: A, b: B) -> FnNumber2<A, B> {
    FnNumber2("SUMX2PY2", a, b)
}

/// Returns the sum of the squares of the differences between matrix A and B.
///
/// __Syntax__: 
/// ```ods
///     SUMXMY2( A: Array; B: Array )
/// ```
///
/// __Constraints__:
/// Both matrices shall have the same dimensions.
///
/// __Semantics__:
/// Sums up the squares of the differences of the corresponding elements for 
/// two matrices.
#[inline]
pub fn sumxmy2<A: Array, B: Array>(a: A, b: B) -> FnNumber2<A, B> {
    FnNumber2("SUMXMY2", a, b)
}

/// Return the tangent of an angle specified in radians
///
/// __Syntax__: 
/// ```ods
///     TAN( N: Number )
/// ```
///
/// __Constraints__:
/// None
///
/// __Semantics__:
/// Computes the tangent of an angle specified in radians.
/// 
/// TAN(x) = SIN(x) / COS(x)
///
/// __See also__: "ATAN", "ATAN2", "RADIANS", "DEGREES", "SIN", "COS", "COT", 
#[inline]
pub fn tan<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("TAN", n)
}

/// Return the hyperbolic tangent of the given hyperbolic angle
///
/// __Syntax__: 
/// ```ods
///     TANH( N: Number )
/// ```
///
/// __Constraints__:
/// None
///
/// __Semantics__:
/// Computes the hyperbolic tangent of a hyperbolic angle. The hyperbolic 
/// tangent is an analog of the ordinary (circular) tangent. The points (cosh 
/// t, sinh t) define the right half of the equilateral hyperbola, just as the 
/// points (cos t, sin t) define the points of a circle.
///
/// __See also__: "ATANH", "SINH", "COSH", "FISHERINV", 
#[inline]
pub fn tanh<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("TANH", n)
}
