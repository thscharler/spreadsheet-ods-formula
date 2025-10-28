//! 
//! The following are statistical functions (functions that report information 
//! on a set of numbers). Some functions that could also be considered 
//! statistical functions, such as SUM, are listed elsewhere.

use crate::*;
#[allow(unused_imports)]
use crate::stat::*;

/// Calculates the average of the absolute deviations of the values in list.
///
/// [documentfoundation->AVEDEV](https://wiki.documentfoundation.org/Documentation/Calc_Functions/AVEDEV)
///
/// __Syntax__: 
/// ```ods
///     AVEDEV({ N: NumberSequenceList}+ )
/// ```
///
/// __Constraints__:
/// None.
///
/// __Semantics__:
/// For a list N containing n numbers x1 to xn, with average x, AVEDEV(N) is 
/// equal to:
///
/// __See also__: [crate::of::sum()], [crate::of::average()], 
#[inline]
pub fn avedev<A: Sequence>(n: A) -> FnNumber1<A> {
    FnNumber1("AVEDEV", n)
}

/// Average the set of numbers
///
/// [documentfoundation->AVERAGE](https://wiki.documentfoundation.org/Documentation/Calc_Functions/AVERAGE)
///
/// __Syntax__: 
/// ```ods
///     AVERAGE({ N: NumberSequence}+ )
/// ```
///
/// __Constraints__:
/// At least one Number included. Returns an Error if no Numbers provided.
///
/// __Semantics__:
/// Computes SUM(N) / COUNT(N).
///
/// __See also__: [crate::of::sum()], [crate::of::count()], 
#[inline]
pub fn average<A: Sequence>(n: A) -> FnNumber1<A> {
    FnNumber1("AVERAGE", n)
}

/// Average values, including values of type Text and Logical.
///
/// [documentfoundation->AVERAGEA](https://wiki.documentfoundation.org/Documentation/Calc_Functions/AVERAGEA)
///
/// __Syntax__: 
/// ```ods
///     AVERAGEA({ N: Any}+ )
/// ```
///
/// __Constraints__:
/// At least one value included. Returns an Error if no value provided.
///
/// __Semantics__:
/// A variant of the AVERAGE function that includes values of type Text and 
/// Logical. Text values are treated as number 0. Logical TRUE is treated as 1 
/// and FALSE is treated as 0. Empty cells are not included. Any N may be of 
/// type ReferenceList.
///
/// __See also__: [crate::of::average()], 
#[inline]
pub fn averagea<A: Sequence>(n: A) -> FnNumber1<A> {
    FnNumber1("AVERAGEA", n)
}

/// Average the values of cells in a range that meet a criteria.
///
/// [documentfoundation->AVERAGEIF](https://wiki.documentfoundation.org/Documentation/Calc_Functions/AVERAGEIF)
///
/// __Syntax__: 
/// ```ods
///     AVERAGEIF( R: Reference; C: Criterion )
/// ```
///
/// __Constraints__:
/// Does not accept constant values as reference parameters.
///
/// __Semantics__:
/// If reference A is omitted, averages the values of cells in the reference 
/// range R that meet the Criterion C (4.11.8). If reference A is given, 
/// averages the values of cells of a range that is constructed using the top 
/// left cell of reference A and applying the dimensions, shape and size, of 
/// reference R. If no cell in range R matches the Criterion C, an Error is 
/// returned. If no Numbers are in the range to be averaged, an Error is 
/// returned.
/// 
/// The values returned may vary depending upon the 
/// HOST-USE-REGULAR-EXPRESSIONS or HOST-USE-WILDCARDS or 
/// HOST-SEARCH-CRITERIA-MUST-APPLY-TO-WHOLE-CELL properties. 3.4
///
/// __See also__: [crate::of::averageifs()], [crate::of::countif()], [crate::of::sumif()], [crate::of::infix operator "="()], [crate::of::infix operator "<>"()], [crate::of::infix operator ordered comparison ("<", "<=", ">", ">=")()], [crate::of::averageif_()], 
#[inline]
pub fn averageif<A: Reference, B: Criterion>(r: A, c: B) -> FnNumber2<A, B> {
    FnNumber2("AVERAGEIF", r, c)
}

/// Average the values of cells in a range that meet a criteria.
///
/// [documentfoundation->AVERAGEIF](https://wiki.documentfoundation.org/Documentation/Calc_Functions/AVERAGEIF)
///
/// __Syntax__: 
/// ```ods
///     AVERAGEIF( R: Reference; C: Criterion; A: Reference )
/// ```
///
/// __Constraints__:
/// Does not accept constant values as reference parameters.
///
/// __Semantics__:
/// If reference A is omitted, averages the values of cells in the reference 
/// range R that meet the Criterion C (4.11.8). If reference A is given, 
/// averages the values of cells of a range that is constructed using the top 
/// left cell of reference A and applying the dimensions, shape and size, of 
/// reference R. If no cell in range R matches the Criterion C, an Error is 
/// returned. If no Numbers are in the range to be averaged, an Error is 
/// returned.
/// 
/// The values returned may vary depending upon the 
/// HOST-USE-REGULAR-EXPRESSIONS or HOST-USE-WILDCARDS or 
/// HOST-SEARCH-CRITERIA-MUST-APPLY-TO-WHOLE-CELL properties. 3.4
///
/// __See also__: [crate::of::averageifs()], [crate::of::countif()], [crate::of::sumif()], [crate::of::infix operator "="()], [crate::of::infix operator "<>"()], [crate::of::infix operator ordered comparison ("<", "<=", ">", ">=")()], [crate::of::averageif()], 
#[inline]
pub fn averageif_<A: Reference, B: Criterion, C: Reference>(r: A, c: B, a: C) -> FnNumber3<A, B, C> {
    FnNumber3("AVERAGEIF", r, c, a)
}

/// returns the value of the probability density function or the cumulative 
/// distribution function for the beta distribution.
///
/// [documentfoundation->BETADIST](https://wiki.documentfoundation.org/Documentation/Calc_Functions/BETADIST)
///
/// __Syntax__: 
/// ```ods
///     BETADIST( x: Number; α: Number; β: Number )
/// ```
///
/// __Constraints__:
/// α > 0, β > 0, a < b,
/// If α < 1, then the density function has a pole at x = a.
/// If β < 1, then the density function has a pole at x = b.
/// In both cases, if x = a respectively x = b and Cumulative = FALSE, an Error 
/// is returned.
///
/// __Semantics__:
/// If Cumulative is FALSE, BETADIST returns 0 if x < a or x > b and the value
/// 
/// otherwise.
/// 
/// If Cumulative is TRUE, BETADIST returns 0 if x < a, 1 if x > b, and the 
/// value
/// 
/// otherwise.
///
/// __Note__:
/// With substitution
/// ≝
/// 
/// the term can be written as
///
/// __See also__: [crate::of::betainv()], [crate::of::betadist_()], [crate::of::betadist__()], [crate::of::betadist___()], 
#[inline]
pub fn betadist<A: Number, B: Number, C: Number>(x: A, alpha: B, beta: C) -> FnNumber3<A, B, C> {
    FnNumber3("BETADIST", x, alpha, beta)
}

/// returns the value of the probability density function or the cumulative 
/// distribution function for the beta distribution.
///
/// [documentfoundation->BETADIST](https://wiki.documentfoundation.org/Documentation/Calc_Functions/BETADIST)
///
/// __Syntax__: 
/// ```ods
///     BETADIST( x: Number; α: Number; β: Number; a: Number )
/// ```
///
/// __Constraints__:
/// α > 0, β > 0, a < b,
/// If α < 1, then the density function has a pole at x = a.
/// If β < 1, then the density function has a pole at x = b.
/// In both cases, if x = a respectively x = b and Cumulative = FALSE, an Error 
/// is returned.
///
/// __Semantics__:
/// If Cumulative is FALSE, BETADIST returns 0 if x < a or x > b and the value
/// 
/// otherwise.
/// 
/// If Cumulative is TRUE, BETADIST returns 0 if x < a, 1 if x > b, and the 
/// value
/// 
/// otherwise.
///
/// __Note__:
/// With substitution
/// ≝
/// 
/// the term can be written as
///
/// __See also__: [crate::of::betainv()], [crate::of::betadist()], [crate::of::betadist__()], [crate::of::betadist___()], 
#[inline]
pub fn betadist_<A: Number, B: Number, C: Number, D: Number>(x: A, alpha: B, beta: C, a: D) -> FnNumber4<A, B, C, D> {
    FnNumber4("BETADIST", x, alpha, beta, a)
}

/// returns the value of the probability density function or the cumulative 
/// distribution function for the beta distribution.
///
/// [documentfoundation->BETADIST](https://wiki.documentfoundation.org/Documentation/Calc_Functions/BETADIST)
///
/// __Syntax__: 
/// ```ods
///     BETADIST( x: Number; α: Number; β: Number; a: Number; b: Number )
/// ```
///
/// __Constraints__:
/// α > 0, β > 0, a < b,
/// If α < 1, then the density function has a pole at x = a.
/// If β < 1, then the density function has a pole at x = b.
/// In both cases, if x = a respectively x = b and Cumulative = FALSE, an Error 
/// is returned.
///
/// __Semantics__:
/// If Cumulative is FALSE, BETADIST returns 0 if x < a or x > b and the value
/// 
/// otherwise.
/// 
/// If Cumulative is TRUE, BETADIST returns 0 if x < a, 1 if x > b, and the 
/// value
/// 
/// otherwise.
///
/// __Note__:
/// With substitution
/// ≝
/// 
/// the term can be written as
///
/// __See also__: [crate::of::betainv()], [crate::of::betadist()], [crate::of::betadist_()], [crate::of::betadist___()], 
#[inline]
pub fn betadist__<A: Number, B: Number, C: Number, D: Number, E: Number>(x: A, alpha: B, beta: C, a: D, b: E) -> FnNumber5<A, B, C, D, E> {
    FnNumber5("BETADIST", x, alpha, beta, a, b)
}

/// returns the value of the probability density function or the cumulative 
/// distribution function for the beta distribution.
///
/// [documentfoundation->BETADIST](https://wiki.documentfoundation.org/Documentation/Calc_Functions/BETADIST)
///
/// __Syntax__: 
/// ```ods
///     BETADIST( x: Number; α: Number; β: Number; a: Number; b: Number; Cumulative: Logical )
/// ```
///
/// __Constraints__:
/// α > 0, β > 0, a < b,
/// If α < 1, then the density function has a pole at x = a.
/// If β < 1, then the density function has a pole at x = b.
/// In both cases, if x = a respectively x = b and Cumulative = FALSE, an Error 
/// is returned.
///
/// __Semantics__:
/// If Cumulative is FALSE, BETADIST returns 0 if x < a or x > b and the value
/// 
/// otherwise.
/// 
/// If Cumulative is TRUE, BETADIST returns 0 if x < a, 1 if x > b, and the 
/// value
/// 
/// otherwise.
///
/// __Note__:
/// With substitution
/// ≝
/// 
/// the term can be written as
///
/// __See also__: [crate::of::betainv()], [crate::of::betadist()], [crate::of::betadist_()], [crate::of::betadist__()], 
#[inline]
pub fn betadist___<A: Number, B: Number, C: Number, D: Number, E: Number, F: Logical>(x: A, alpha: B, beta: C, a: D, b: E, cumulative: F) -> FnNumber6<A, B, C, D, E, F> {
    FnNumber6("BETADIST", x, alpha, beta, a, b, cumulative)
}

/// returns the inverse of BETADIST(x;α;β;A;B;TRUE()).
///
/// [documentfoundation->BETAINV](https://wiki.documentfoundation.org/Documentation/Calc_Functions/BETAINV)
///
/// __Syntax__: 
/// ```ods
///     BETAINV( P: Number; α: Number; β: Number )
/// ```
///
/// __Constraints__:
/// 0 ≤ P ≤ 1, α > 0, β > 0, A < B
///
/// __Semantics__:
/// BETAINV returns the unique number x in the closed interval from A to B such 
/// that BETADIST(x;α;β;A;B) = P.
///
/// __See also__: [crate::of::betadist()], [crate::of::betainv_()], [crate::of::betainv__()], 
#[inline]
pub fn betainv<A: Number, B: Number, C: Number>(p: A, alpha: B, beta: C) -> FnNumber3<A, B, C> {
    FnNumber3("BETAINV", p, alpha, beta)
}

/// returns the inverse of BETADIST(x;α;β;A;B;TRUE()).
///
/// [documentfoundation->BETAINV](https://wiki.documentfoundation.org/Documentation/Calc_Functions/BETAINV)
///
/// __Syntax__: 
/// ```ods
///     BETAINV( P: Number; α: Number; β: Number; A: Number )
/// ```
///
/// __Constraints__:
/// 0 ≤ P ≤ 1, α > 0, β > 0, A < B
///
/// __Semantics__:
/// BETAINV returns the unique number x in the closed interval from A to B such 
/// that BETADIST(x;α;β;A;B) = P.
///
/// __See also__: [crate::of::betadist()], [crate::of::betainv()], [crate::of::betainv__()], 
#[inline]
pub fn betainv_<A: Number, B: Number, C: Number, D: Number>(p: A, alpha: B, beta: C, a: D) -> FnNumber4<A, B, C, D> {
    FnNumber4("BETAINV", p, alpha, beta, a)
}

/// returns the inverse of BETADIST(x;α;β;A;B;TRUE()).
///
/// [documentfoundation->BETAINV](https://wiki.documentfoundation.org/Documentation/Calc_Functions/BETAINV)
///
/// __Syntax__: 
/// ```ods
///     BETAINV( P: Number; α: Number; β: Number; A: Number; B: Number )
/// ```
///
/// __Constraints__:
/// 0 ≤ P ≤ 1, α > 0, β > 0, A < B
///
/// __Semantics__:
/// BETAINV returns the unique number x in the closed interval from A to B such 
/// that BETADIST(x;α;β;A;B) = P.
///
/// __See also__: [crate::of::betadist()], [crate::of::betainv()], [crate::of::betainv_()], 
#[inline]
pub fn betainv__<A: Number, B: Number, C: Number, D: Number, E: Number>(p: A, alpha: B, beta: C, a: D, b: E) -> FnNumber5<A, B, C, D, E> {
    FnNumber5("BETAINV", p, alpha, beta, a, b)
}

/// Returns the probability of a trial result using binomial distribution.
///
/// [documentfoundation->BINOM.DIST.RANGE](https://wiki.documentfoundation.org/Documentation/Calc_Functions/BINOM.DIST.RANGE)
///
/// __Syntax__: 
/// ```ods
///     BINOM.DIST.RANGE( N: Integer; P: Number; S: Integer )
/// ```
///
/// __Constraints__:
/// 0 ≤ P ≤ 1, 0 ≤ S ≤ S2 ≤ N
///
/// __Semantics__:
/// Let N be a total number of independent trials, and P be a probability of 
/// success for each trial. This function returns the probability that the 
/// number of successful trials shall be exactly S. If the optional parameter 
/// S2 is provided, this function returns the probability that the number of 
/// successful trials shall lie between S and S2 inclusive.
/// 
/// This function is computed as follows:
/// 
/// If S2 is not given, let S2 = S. Then the function returns the value of
///
/// __See also__: [crate::of::binomdist()], [crate::of::binom_dist_range_()], 
#[inline]
pub fn binom_dist_range<A: Number, B: Number, C: Number>(n: A, p: B, s: C) -> FnNumber3<A, B, C> {
    FnNumber3("BINOM.DIST.RANGE", n, p, s)
}

/// Returns the probability of a trial result using binomial distribution.
///
/// [documentfoundation->BINOM.DIST.RANGE](https://wiki.documentfoundation.org/Documentation/Calc_Functions/BINOM.DIST.RANGE)
///
/// __Syntax__: 
/// ```ods
///     BINOM.DIST.RANGE( N: Integer; P: Number; S: Integer; S2: Integer )
/// ```
///
/// __Constraints__:
/// 0 ≤ P ≤ 1, 0 ≤ S ≤ S2 ≤ N
///
/// __Semantics__:
/// Let N be a total number of independent trials, and P be a probability of 
/// success for each trial. This function returns the probability that the 
/// number of successful trials shall be exactly S. If the optional parameter 
/// S2 is provided, this function returns the probability that the number of 
/// successful trials shall lie between S and S2 inclusive.
/// 
/// This function is computed as follows:
/// 
/// If S2 is not given, let S2 = S. Then the function returns the value of
///
/// __See also__: [crate::of::binomdist()], [crate::of::binom_dist_range()], 
#[inline]
pub fn binom_dist_range_<A: Number, B: Number, C: Number, D: Number>(n: A, p: B, s: C, s2: D) -> FnNumber4<A, B, C, D> {
    FnNumber4("BINOM.DIST.RANGE", n, p, s, s2)
}

/// Returns the binomial distribution.
///
/// [documentfoundation->BINOMDIST](https://wiki.documentfoundation.org/Documentation/Calc_Functions/BINOMDIST)
///
/// __Syntax__: 
/// ```ods
///     BINOMDIST( S: Integer; N: Integer; P: Number; Cumulative: Logical )
/// ```
///
/// __Constraints__:
/// 0 ≤ P ≤ 1; 0 ≤ S ≤ N
///
/// __Semantics__:
/// If Cumulative is FALSE, this function returns the same result as 
/// BINOM.DIST.RANGE(N;P;S). If Cumulative is TRUE, it is equivalent to calling 
/// BINOM.DIST.RANGE(N;P;0;S).
///
/// __See also__: [crate::of::binom_dist_range()], 
#[inline]
pub fn binomdist<A: Number, B: Number, C: Number, D: Logical>(s: A, n: B, p: C, cumulative: D) -> FnNumber4<A, B, C, D> {
    FnNumber4("BINOMDIST", s, n, p, cumulative)
}

/// returns the right-tail probability for the χ2-distribution.
///
/// [documentfoundation->LEGACY.CHIDIST](https://wiki.documentfoundation.org/Documentation/Calc_Functions/LEGACY.CHIDIST)
///
/// __Syntax__: 
/// ```ods
///     LEGACY.CHIDIST( X: Number; DegreesOfFreedom: Number )
/// ```
///
/// __Constraints__:
/// DegreesOfFreedom is a positive integer.
///
/// __Semantics__:
/// In the following n is DegreesOfFreedom. LEGACY.CHIDIST returns 1 for X ≤ 
/// 0 and the value
/// 
/// for X > 0.
///
/// __See also__: [crate::of::chisqdist()], [crate::of::legacy_chitest()], 
#[inline]
pub fn legacy_chidist<A: Number, B: Number>(x: A, degrees_of_freedom: B) -> FnNumber2<A, B> {
    FnNumber2("LEGACY.CHIDIST", x, degrees_of_freedom)
}

/// returns the value of the probability density function or the cumulative 
/// distribution function for the χ2-distribution.
///
/// [documentfoundation->CHISQDIST](https://wiki.documentfoundation.org/Documentation/Calc_Functions/CHISQDIST)
///
/// __Syntax__: 
/// ```ods
///     CHISQDIST( X: Number; DegreesOfFreedom: Number )
/// ```
///
/// __Constraints__:
/// DegreesOfFreedom is a positive integer.
///
/// __Semantics__:
/// In the following n is DegreesOfFreedom.
/// 
/// If Cumulative is FALSE, CHISQDIST returns 0 for X ≤ 0 and the value
/// 
/// for X > 0.
/// 
/// If Cumulative is TRUE, CHISQDIST returns 0 for X ≤ 0 and the value
/// 
/// for X > 0.
///
/// __See also__: [crate::of::legacy_chidist()], [crate::of::chisqdist_()], 
#[inline]
pub fn chisqdist<A: Number, B: Number>(x: A, degrees_of_freedom: B) -> FnNumber2<A, B> {
    FnNumber2("CHISQDIST", x, degrees_of_freedom)
}

/// returns the value of the probability density function or the cumulative 
/// distribution function for the χ2-distribution.
///
/// [documentfoundation->CHISQDIST](https://wiki.documentfoundation.org/Documentation/Calc_Functions/CHISQDIST)
///
/// __Syntax__: 
/// ```ods
///     CHISQDIST( X: Number; DegreesOfFreedom: Number; Cumulative: Logical )
/// ```
///
/// __Constraints__:
/// DegreesOfFreedom is a positive integer.
///
/// __Semantics__:
/// In the following n is DegreesOfFreedom.
/// 
/// If Cumulative is FALSE, CHISQDIST returns 0 for X ≤ 0 and the value
/// 
/// for X > 0.
/// 
/// If Cumulative is TRUE, CHISQDIST returns 0 for X ≤ 0 and the value
/// 
/// for X > 0.
///
/// __See also__: [crate::of::legacy_chidist()], [crate::of::chisqdist()], 
#[inline]
pub fn chisqdist_<A: Number, B: Number, C: Logical>(x: A, degrees_of_freedom: B, cumulative: C) -> FnNumber3<A, B, C> {
    FnNumber3("CHISQDIST", x, degrees_of_freedom, cumulative)
}

/// returns the inverse of LEGACY.CHIDIST(x; DegreesOfFreedom).
///
/// [documentfoundation->LEGACY.CHIINV](https://wiki.documentfoundation.org/Documentation/Calc_Functions/LEGACY.CHIINV)
///
/// __Syntax__: 
/// ```ods
///     LEGACY.CHIINV( P: Number; DegreesOfFreedom: Number )
/// ```
///
/// __Constraints__:
/// DegreesOfFreedom is a positive integer and 0 < P ≤ 1.
///
/// __Semantics__:
/// LEGACY.CHIINV returns the unique number x such that LEGACY.CHIDIST(x; 
/// DegreesOfFreedom) = P.
///
/// __See also__: [crate::of::legacy_chidist()], 
#[inline]
pub fn legacy_chiinv<A: Number, B: Number>(p: A, degrees_of_freedom: B) -> FnNumber2<A, B> {
    FnNumber2("LEGACY.CHIINV", p, degrees_of_freedom)
}

/// returns the inverse of CHISQDIST(x; DegreesOfFreedom; TRUE()).
///
/// [documentfoundation->CHISQINV](https://wiki.documentfoundation.org/Documentation/Calc_Functions/CHISQINV)
///
/// __Syntax__: 
/// ```ods
///     CHISQINV( P: Number; DegreesOfFreedom: Number )
/// ```
///
/// __Constraints__:
/// DegreesOfFreedom is a positive integer and 0 < P ≤ 1 .
///
/// __Semantics__:
/// CHISQINV returns the unique number x ≥ 0 such that CHISQDIST(x; 
/// DegreesOfFreedom;TRUE()) = P.
///
/// __See also__: [crate::of::chisqdist()], 
#[inline]
pub fn chisqinv<A: Number, B: Number>(p: A, degrees_of_freedom: B) -> FnNumber2<A, B> {
    FnNumber2("CHISQINV", p, degrees_of_freedom)
}

/// Returns some Chi square goodness-for-fit test.
///
/// [documentfoundation->LEGACY.CHITEST](https://wiki.documentfoundation.org/Documentation/Calc_Functions/LEGACY.CHITEST)
///
/// __Syntax__: 
/// ```ods
///     LEGACY.CHITEST( A: Array; E: Array )
/// ```
///
/// __Constraints__:
/// 
/// ROWS(A) = ROWS(E)
/// COLUMNS(A) = COLUMNS(E)
/// COLUMNS(A) * ROWS(A) > 1
///
/// __Semantics__:
/// 
/// For an empty element or an element of type Text or Boolean in A the element 
/// at the corresponding position of E is ignored, and vice versa.
/// 
/// •A: actual observation data.
/// 
/// •E: expected values.
/// 
/// First a Chi square statistic is calculated:
/// 
/// Then LEGACY.CHIDIST is called with the Chi-square value and a degree of 
/// freedom (df):
///
/// __See also__: [crate::of::columns()], [crate::of::rows()], [crate::of::legacy_chidist()], 
#[inline]
pub fn legacy_chitest<A: Array, B: Array>(a: A, e: B) -> FnNumber2<A, B> {
    FnNumber2("LEGACY.CHITEST", a, e)
}

/// Returns the confidence interval for a population mean.
///
/// [documentfoundation->CONFIDENCE](https://wiki.documentfoundation.org/Documentation/Calc_Functions/CONFIDENCE)
///
/// __Syntax__: 
/// ```ods
///     CONFIDENCE( Alpha: Number; Stddev: Number; Size: Number )
/// ```
///
/// __Constraints__:
/// 0 < Alpha < 1; Stddev > 0, Size ≥ 1
///
/// __Semantics__:
/// Calling this function is equivalent to calling
/// NORMINV(1 - Alpha / 2; 0; 1) * Stddev / SQRT (Size)
///
/// __See also__: [crate::of::norminv()], [crate::of::sqrt()], 
#[inline]
pub fn confidence<A: Number, B: Number, C: Number>(alpha: A, stddev: B, size: C) -> FnNumber3<A, B, C> {
    FnNumber3("CONFIDENCE", alpha, stddev, size)
}

/// Calculates the correlation coefficient of values in N1 and N2.
///
/// [documentfoundation->CORREL](https://wiki.documentfoundation.org/Documentation/Calc_Functions/CORREL)
///
/// __Syntax__: 
/// ```ods
///     CORREL( N1: Array; N2: Array )
/// ```
///
/// __Constraints__:
/// COLUMNS(N1) = COLUMNS(N2), ROWS(N1) = ROWS(N2), both sequences shall 
/// contain at least one number at corresponding positions each.
///
/// __Semantics__:
/// Has the same value as COVAR(N1;N2) / STDEVP(N1) * (STDEVP(N2)). The CORREL 
/// function actually is identical to the PEARSON function.
/// 
/// For an empty element or an element of type Text or Boolean in N1 the 
/// element at the corresponding position of N2 is ignored, and vice versa.
///
/// __See also__: [crate::of::columns()], [crate::of::rows()], [crate::of::covar()], [crate::of::stdevp()], [crate::of::pearson()], 
#[inline]
pub fn correl<A: Array, B: Array>(n1: A, n2: B) -> FnNumber2<A, B> {
    FnNumber2("CORREL", n1, n2)
}

/// Calculates covariance of two cell ranges.
///
/// [documentfoundation->COVAR](https://wiki.documentfoundation.org/Documentation/Calc_Functions/COVAR)
///
/// __Syntax__: 
/// ```ods
///     COVAR( N1: Array; N2: Array )
/// ```
///
/// __Constraints__:
/// COLUMNS(N1) = COLUMNS(N2), ROWS(N1) = ROWS(N2), both sequences shall 
/// contain at least one number at corresponding positions each.
///
/// __Semantics__:
/// returns
/// 
/// where
/// is the result of calling AVERAGE(N1), and
/// is the result of calling AVERAGE(N2), and N is the number of terms in the 
/// sum.
/// 
/// For an empty element or an element of type Text or Boolean in N1 the 
/// element at the corresponding position of N2 is ignored, and vice versa.
///
/// __See also__: [crate::of::columns()], [crate::of::rows()], [crate::of::average()], 
#[inline]
pub fn covar<A: Array, B: Array>(n1: A, n2: B) -> FnNumber2<A, B> {
    FnNumber2("COVAR", n1, n2)
}

/// Returns the smallest value for which the cumulative binomial distribution 
/// is greater than or equal to a criterion value.
///
/// [documentfoundation->CRITBINOM](https://wiki.documentfoundation.org/Documentation/Calc_Functions/CRITBINOM)
///
/// __Syntax__: 
/// ```ods
///     CRITBINOM( Trials: Number; SP: Number; Alpha: Number )
/// ```
///
/// __Constraints__:
/// Trials ≥ 0, 0 ≤ SP ≤ 1, 0 ≤ Alpha ≤ 1
///
/// __Semantics__:
/// 
/// •Trials: the total number of trials.
/// 
/// •SP: the probability of success for one trial.
/// 
/// •Alpha: the threshold probability to be reached or exceeded.
///
/// __See also__: 
#[inline]
pub fn critbinom<A: Number, B: Number, C: Number>(trials: A, s_p: B, alpha: C) -> FnNumber3<A, B, C> {
    FnNumber3("CRITBINOM", trials, s_p, alpha)
}

/// Calculates sum of squares of deviations.
///
/// [documentfoundation->DEVSQ](https://wiki.documentfoundation.org/Documentation/Calc_Functions/DEVSQ)
///
/// __Syntax__: 
/// ```ods
///     DEVSQ({ N: NumberSequence}+ )
/// ```
///
/// __Semantics__:
/// returns
/// 
/// where a is the result of calling AVERAGE(N).
///
/// __See also__: 
#[inline]
pub fn devsq<A: Sequence>(n: A) -> FnNumber1<A> {
    FnNumber1("DEVSQ", n)
}

/// returns the value of the probability density function or the cumulative 
/// distribution function for the exponential distribution.
///
/// [documentfoundation->EXPONDIST](https://wiki.documentfoundation.org/Documentation/Calc_Functions/EXPONDIST)
///
/// __Syntax__: 
/// ```ods
///     EXPONDIST( X: Number; lambda: Number )
/// ```
///
/// __Constraints__:
/// lambda > 0
///
/// __Semantics__:
/// If Cumulative is FALSE, EXPONDIST returns 0 if X < 0 and the value
/// 
/// otherwise.
/// 
/// If Cumulative is TRUE, EXPONDIST returns 0 if X < 0 and the value
/// 
/// otherwise.
///
/// __See also__: [crate::of::expondist_()], 
#[inline]
pub fn expondist<A: Number, B: Number>(x: A, lambda: B) -> FnNumber2<A, B> {
    FnNumber2("EXPONDIST", x, lambda)
}

/// returns the value of the probability density function or the cumulative 
/// distribution function for the exponential distribution.
///
/// [documentfoundation->EXPONDIST](https://wiki.documentfoundation.org/Documentation/Calc_Functions/EXPONDIST)
///
/// __Syntax__: 
/// ```ods
///     EXPONDIST( X: Number; lambda: Number; Cumulative: Logical )
/// ```
///
/// __Constraints__:
/// lambda > 0
///
/// __Semantics__:
/// If Cumulative is FALSE, EXPONDIST returns 0 if X < 0 and the value
/// 
/// otherwise.
/// 
/// If Cumulative is TRUE, EXPONDIST returns 0 if X < 0 and the value
/// 
/// otherwise.
///
/// __See also__: [crate::of::expondist()], 
#[inline]
pub fn expondist_<A: Number, B: Number, C: Logical>(x: A, lambda: B, cumulative: C) -> FnNumber3<A, B, C> {
    FnNumber3("EXPONDIST", x, lambda, cumulative)
}

/// returns the value of the probability density function or the cumulative 
/// distribution function for the F-distribution.
///
/// [documentfoundation->FDIST](https://wiki.documentfoundation.org/Documentation/Calc_Functions/FDIST)
///
/// __Syntax__: 
/// ```ods
///     FDIST( X: Number; R1: Number; R2: Number )
/// ```
///
/// __Constraints__:
/// R1 and R2 are positive integers
///
/// __Semantics__:
/// 
/// •R1: the degrees of freedom in the numerator of the F distribution.
/// 
/// •R2: the degrees of freedom in the denominator of the F distribution.
/// 
/// If Cumulative is FALSE, FDIST returns 0 if X < 0, an Error if the numerator 
/// degrees of freedom R1 = 1 and X = 0, and the value
/// 
/// otherwise.
/// If the numerator degrees of freedom R1 = 1, then the density function has a 
/// pole at X = 0, the subterm
/// is not defined.
/// 
/// If Cumulative is TRUE, FDIST returns 0 if X < 0 and the value
/// 
/// otherwise.
///
/// __See also__: [crate::of::legacy_fdist()], [crate::of::fdist_()], 
#[inline]
pub fn fdist<A: Number, B: Number, C: Number>(x: A, r1: B, r2: C) -> FnNumber3<A, B, C> {
    FnNumber3("FDIST", x, r1, r2)
}

/// returns the value of the probability density function or the cumulative 
/// distribution function for the F-distribution.
///
/// [documentfoundation->FDIST](https://wiki.documentfoundation.org/Documentation/Calc_Functions/FDIST)
///
/// __Syntax__: 
/// ```ods
///     FDIST( X: Number; R1: Number; R2: Number; Cumulative: Logical )
/// ```
///
/// __Constraints__:
/// R1 and R2 are positive integers
///
/// __Semantics__:
/// 
/// •R1: the degrees of freedom in the numerator of the F distribution.
/// 
/// •R2: the degrees of freedom in the denominator of the F distribution.
/// 
/// If Cumulative is FALSE, FDIST returns 0 if X < 0, an Error if the numerator 
/// degrees of freedom R1 = 1 and X = 0, and the value
/// 
/// otherwise.
/// If the numerator degrees of freedom R1 = 1, then the density function has a 
/// pole at X = 0, the subterm
/// is not defined.
/// 
/// If Cumulative is TRUE, FDIST returns 0 if X < 0 and the value
/// 
/// otherwise.
///
/// __See also__: [crate::of::legacy_fdist()], [crate::of::fdist()], 
#[inline]
pub fn fdist_<A: Number, B: Number, C: Number, D: Logical>(x: A, r1: B, r2: C, cumulative: D) -> FnNumber4<A, B, C, D> {
    FnNumber4("FDIST", x, r1, r2, cumulative)
}

/// returns the area of the right tail of the probability density function for 
/// the F-distribution.
///
/// [documentfoundation->LEGACY.FDIST](https://wiki.documentfoundation.org/Documentation/Calc_Functions/LEGACY.FDIST)
///
/// __Syntax__: 
/// ```ods
///     LEGACY.FDIST( X: Number; R1: Number; R2: Number )
/// ```
///
/// __Constraints__:
/// R1 and R2 are positive integers
///
/// __Semantics__:
/// 
/// LEGACY.FDIST returns Error if x < 0 and the value
/// 
/// otherwise.
/// 
/// Note that the latter is (1-FDIST(x; r1; r2;TRUE())).
///
/// __See also__: [crate::of::fdist()], 
#[inline]
pub fn legacy_fdist<A: Number, B: Number, C: Number>(x: A, r1: B, r2: C) -> FnNumber3<A, B, C> {
    FnNumber3("LEGACY.FDIST", x, r1, r2)
}

/// returns the inverse of FDIST(x;R1;R2;TRUE()).
///
/// [documentfoundation->FINV](https://wiki.documentfoundation.org/Documentation/Calc_Functions/FINV)
///
/// __Syntax__: 
/// ```ods
///     FINV( P: Number; R1: Number; R2: Number )
/// ```
///
/// __Constraints__:
/// 0 ≤ P < 1, R1 and R2 are positive integers
///
/// __Semantics__:
/// FINV returns the unique non-negative number x such that FDIST(x;R1;R2) = P.
///
/// __See also__: [crate::of::fdist()], [crate::of::legacy_fdist()], [crate::of::legacy_finv()], 
#[inline]
pub fn finv<A: Number, B: Number, C: Number>(p: A, r1: B, r2: C) -> FnNumber3<A, B, C> {
    FnNumber3("FINV", p, r1, r2)
}

/// returns the inverse of LEGACY.FDIST(x;R1;R2).
///
/// [documentfoundation->LEGACY.FINV](https://wiki.documentfoundation.org/Documentation/Calc_Functions/LEGACY.FINV)
///
/// __Syntax__: 
/// ```ods
///     LEGACY.FINV( P: Number; R1: Number; R2: Number )
/// ```
///
/// __Constraints__:
/// 0 < P ≤ 1, R1 and R2 are positive integers
///
/// __Semantics__:
/// LEGACY.FINV returns the unique non-negative number x such that 
/// LEGACY.FDIST(x;R1;R2) = P.
///
/// __See also__: [crate::of::fdist()], [crate::of::legacy_fdist()], [crate::of::finv()], 
#[inline]
pub fn legacy_finv<A: Number, B: Number, C: Number>(p: A, r1: B, r2: C) -> FnNumber3<A, B, C> {
    FnNumber3("LEGACY.FINV", p, r1, r2)
}

/// returns the Fisher transformation.
///
/// [documentfoundation->FISHER](https://wiki.documentfoundation.org/Documentation/Calc_Functions/FISHER)
///
/// __Syntax__: 
/// ```ods
///     FISHER( R: Number )
/// ```
///
/// __Constraints__:
/// -1 < R < 1
///
/// __Semantics__:
/// Returns the Fisher transformation with a sample correlation R. This 
/// function computes
/// 
/// where ln is the natural logarithm function.
/// 
/// FISHER is a synonym for ATANH.
///
/// __See also__: [crate::of::atanh()], 
#[inline]
pub fn fisher<A: Number>(r: A) -> FnNumber1<A> {
    FnNumber1("FISHER", r)
}

/// returns the inverse Fisher transformation.
///
/// [documentfoundation->FISHERINV](https://wiki.documentfoundation.org/Documentation/Calc_Functions/FISHERINV)
///
/// __Syntax__: 
/// ```ods
///     FISHERINV( R: Number )
/// ```
///
/// __Constraints__:
/// none
///
/// __Semantics__:
/// Returns the inverse Fisher transformation. This function computes
/// 
/// FISHERINV is a synonym for TANH.
///
/// __See also__: [crate::of::tanh()], 
#[inline]
pub fn fisherinv<A: Number>(r: A) -> FnNumber1<A> {
    FnNumber1("FISHERINV", r)
}

/// Extrapolates future values based on existing x and y values.
///
/// [documentfoundation->FORECAST](https://wiki.documentfoundation.org/Documentation/Calc_Functions/FORECAST)
///
/// __Syntax__: 
/// ```ods
///     FORECAST( Value: Number; Data_Y: Array; Data_X: Array )
/// ```
///
/// __Constraints__:
/// COLUMNS(Data_Y) = COLUMNS(Data_X), ROWS(Data_Y) = ROWS(Data_X)
///
/// __Semantics__:
/// 
/// •Value: the x-value, for which the y-value on the linear regression is to 
/// be returned.
/// 
/// •Data_Y: the array or range of known y-values.
/// 
/// •Data_X: the array or range of known x-values.
/// 
/// For an empty element or an element of type Text or Boolean in Data_Y the 
/// element at the corresponding position of Data_X is ignored, and vice versa.
///
/// __See also__: [crate::of::columns()], [crate::of::rows()], 
#[inline]
pub fn forecast<A: Number, B: Array, C: Array>(value: A, data_y: B, data_x: C) -> FnNumber3<A, B, C> {
    FnNumber3("FORECAST", value, data_y, data_x)
}

/// Categorizes values into intervals and counts the number of values in each 
/// interval.
///
/// [documentfoundation->FREQUENCY](https://wiki.documentfoundation.org/Documentation/Calc_Functions/FREQUENCY)
///
/// __Syntax__: 
/// ```ods
///     FREQUENCY( Data: NumberSequenceList; Bins: NumberSequenceList )
/// ```
///
/// __Constraints__:
/// Values in Bins shall be sorted in ascending order and Bins shall be a 
/// column vector. Evaluators may accept unsorted values in bins.
///
/// __Semantics__:
/// Counts the number of values for each interval given by the border values in 
/// Bins .
/// The values in Bins determine the upper boundaries of the intervals. The 
/// intervals include the upper boundarie. The returned array is a column 
/// vector and has one more element than Bins ; the last element represents the 
/// number of all elements greater than the last value in Bins . If Bins is 
/// empty, all values in Data are counted. The values in the result array are 
/// ordered matching the original order of Bins . If the values in Bins are not 
/// sorted in ascending order, they are sorted internally to form category 
/// intervals and the counts of Data values are "unsorted" to the original 
/// order of Bins. If Data is empty, the value of all elements in the returned 
/// array is 0.
/// 
/// Data: The data, that should be categorized and counted according to the 
/// given intervals.
/// Bins: The upper boundaries determining the intervals the values in data 
/// should be grouped by.
///
/// __See also__: 
#[inline]
pub fn frequency<A: Sequence, B: Sequence>(data: A, bins: B) -> FnArray2<A, B> {
    FnArray2("FREQUENCY", data, bins)
}

/// Calculates the probability of an F-test.
///
/// [documentfoundation->FTEST](https://wiki.documentfoundation.org/Documentation/Calc_Functions/FTEST)
///
/// __Syntax__: 
/// ```ods
///     FTEST( Data_1: NumberSequence; Data_2: NumberSequence )
/// ```
///
/// __Constraints__:
/// Data_1 and Data_2 shall both contain at least 2 numbers and shall both have 
/// nonzero variances
///
/// __Semantics__:
/// 
/// Calculates a two-sided P-value to decide, whether the difference in the 
/// variances of the two data sets are significant enough to reject the 
/// hypothesise, that both sets come from normally distributed populations with 
/// the same variances.
/// Suppose the data set Data_1 is a sample of size
/// from a normal distribution and has the sample variance
/// , and the data set Data_2 is a sample of size
/// from a normal distribution and has the sample variance
/// .
/// 
/// Get the value
/// as the area of the right tail beyond
/// of the F‑distribution with numerator degrees of freedom
/// and denominator degrees of freedom
/// .
/// 
/// FTEST returns twice the minimum of the values
/// and
/// .See also TTEST 6.18.81
///
/// __See also__: 
#[inline]
pub fn ftest<A: Sequence, B: Sequence>(data_1: A, data_2: B) -> FnNumber2<A, B> {
    FnNumber2("FTEST", data_1, data_2)
}

/// returns the value of the probability density function or the cumulative 
/// distribution function for the Gamma distribution.
///
/// [documentfoundation->GAMMADIST](https://wiki.documentfoundation.org/Documentation/Calc_Functions/GAMMADIST)
///
/// __Syntax__: 
/// ```ods
///     GAMMADIST( X: Number; α: Number; β: Number )
/// ```
///
/// __Constraints__:
/// α > 0, β > 0
///
/// __Semantics__:
/// If Cumulative is FALSE, GAMMADIST returns 0 if X < 0 and the value
/// 
/// otherwise.
/// 
/// If Cumulative is TRUE(), GAMMADIST returns 0 if X < 0 and the value
/// 
/// otherwise.
///
/// __See also__: [crate::of::gamma()], [crate::of::gammainv()], [crate::of::gammadist_()], 
#[inline]
pub fn gammadist<A: Number, B: Number, C: Number>(x: A, alpha: B, beta: C) -> FnNumber3<A, B, C> {
    FnNumber3("GAMMADIST", x, alpha, beta)
}

/// returns the value of the probability density function or the cumulative 
/// distribution function for the Gamma distribution.
///
/// [documentfoundation->GAMMADIST](https://wiki.documentfoundation.org/Documentation/Calc_Functions/GAMMADIST)
///
/// __Syntax__: 
/// ```ods
///     GAMMADIST( X: Number; α: Number; β: Number; Cumulative: Logical )
/// ```
///
/// __Constraints__:
/// α > 0, β > 0
///
/// __Semantics__:
/// If Cumulative is FALSE, GAMMADIST returns 0 if X < 0 and the value
/// 
/// otherwise.
/// 
/// If Cumulative is TRUE(), GAMMADIST returns 0 if X < 0 and the value
/// 
/// otherwise.
///
/// __See also__: [crate::of::gamma()], [crate::of::gammainv()], [crate::of::gammadist()], 
#[inline]
pub fn gammadist_<A: Number, B: Number, C: Number, D: Logical>(x: A, alpha: B, beta: C, cumulative: D) -> FnNumber4<A, B, C, D> {
    FnNumber4("GAMMADIST", x, alpha, beta, cumulative)
}

/// returns the inverse of GAMMADIST(X;α;β;TRUE).
///
/// [documentfoundation->GAMMAINV](https://wiki.documentfoundation.org/Documentation/Calc_Functions/GAMMAINV)
///
/// __Syntax__: 
/// ```ods
///     GAMMAINV( P: Number; α: Number; β: Number )
/// ```
///
/// __Constraints__:
/// 0 ≤ P < 1, α > 0, β > 0
///
/// __Semantics__:
/// GAMMAINV returns the unique number X ≥ 0 such that GAMMAINV(X;α;β) = P.
///
/// __See also__: [crate::of::gammadist()], 
#[inline]
pub fn gammainv<A: Number, B: Number, C: Number>(p: A, alpha: B, beta: C) -> FnNumber3<A, B, C> {
    FnNumber3("GAMMAINV", p, alpha, beta)
}

/// Returns 0.5 less than the standard normal cumulative distribution
///
/// [documentfoundation->GAUSS](https://wiki.documentfoundation.org/Documentation/Calc_Functions/GAUSS)
///
/// __Syntax__: 
/// ```ods
///     GAUSS( X: Number )
/// ```
///
/// __Semantics__:
/// Returns NORMDIST(X;0;1;TRUE())-0.5
///
/// __See also__: [crate::of::normdist()], 
#[inline]
pub fn gauss<A: Number>(x: A) -> FnNumber1<A> {
    FnNumber1("GAUSS", x)
}

/// returns the geometric mean of a sequence
///
/// [documentfoundation->GEOMEAN](https://wiki.documentfoundation.org/Documentation/Calc_Functions/GEOMEAN)
///
/// __Syntax__: 
/// ```ods
///     GEOMEAN({ N: NumberSequenceList}+ )
/// ```
///
/// __Semantics__:
/// Returns the geometric mean of a given sequence. That means
/// 
/// where n is a result of calling COUNT(N).
///
/// __See also__: [crate::of::count()], 
#[inline]
pub fn geomean<A: Sequence>(n: A) -> FnNumber1<A> {
    FnNumber1("GEOMEAN", n)
}

/// Returns predicted values based on an exponential regression.
///
/// [documentfoundation->GROWTH](https://wiki.documentfoundation.org/Documentation/Calc_Functions/GROWTH)
///
/// __Syntax__: 
/// ```ods
///     GROWTH( KnownY: Array )
/// ```
///
/// __Constraints__:
/// (COLUMNS(KnownY) = COLUMNS(KnownX) and ROWS(KnownY) = ROWS(KnownX)) or 
/// (COLUMNS(KnownY) = 1 and ROWS(KnownY) = ROWS(KnownX) and COLUMNS(KnownX) = 
/// COLUMNS(NewX)) or (COLUMNS(KnownY) = COLUMNS(KnownX) and ROWS(KnownY) = 1 
/// and ROWS(KnownX) = ROWS(NewX))
///
/// __Semantics__:
/// 
/// •KnownY: The set of known y-values to be used to determine the regression 
/// equation
/// 
/// •KnownX: The set of known x-values to be used to determine the regression 
/// equation. If omitted or an empty parameter, it is set to the sequence 
/// 1,2,3,…,k , where
/// 
/// k = ROWS(KnownY) ∙ COLUMNS(KnownY)
/// 
/// •NewX: The set of x-values for which predicted y-values are to be 
/// calculated. If omitted or an empty parameter, it is set to KnownX.
/// 
/// Const: If set to FALSE, the model constant a is equal to 0.
/// LOGEST(KnownY ; KnownX; Const; FALSE) either returns an error or an array 
/// with 1 row and n+1 columns. If it returns an error then so does GROWTH. If 
/// it returns an array, we call the entries in that array
/// .
/// Let
/// denote the entry in the ith row and jth column of NewX.
/// If COLUMNS(KnownY ) = COLUMNS(KnownX) and ROWS(KnownY ) = ROWS(KnownX), 
/// then GROWTH returns an array with ROWS(NewX) rows and COLUMNS(NewX) column, 
/// such that the entry in its ith row and jth column is
/// .
/// Otherwise, if COLUMNS(KnownY ) = 1 and ROWS(KnownY ) = ROWS(KnownX) and 
/// COLUMNS(KnownX) = COLUMNS(NewX), then GROWTH returns an array with 
/// ROWS(NewX) rows and 1 column, such that the entry in the ith row is
/// .
/// Otherwise, if COLUMNS(KnownY ) = COLUMNS(KnownX) and ROWS(KnownY ) = 1 and 
/// ROWS(KnownX) = ROWS(NewX), then GROWTH returns an array with 1 row and 
/// COLUMNS(NewX) columns, such that the entry in the jth column is
/// .
///
/// __See also__: [crate::of::columns()], [crate::of::rows()], [crate::of::logest()], [crate::of::trend()], [crate::of::growth_()], [crate::of::growth__()], [crate::of::growth___()], 
#[inline]
pub fn growth<A: Array>(known_y: A) -> FnArray1<A> {
    FnArray1("GROWTH", known_y)
}

/// Returns predicted values based on an exponential regression.
///
/// [documentfoundation->GROWTH](https://wiki.documentfoundation.org/Documentation/Calc_Functions/GROWTH)
///
/// __Syntax__: 
/// ```ods
///     GROWTH( KnownY: Array; KnownX: Array )
/// ```
///
/// __Constraints__:
/// (COLUMNS(KnownY) = COLUMNS(KnownX) and ROWS(KnownY) = ROWS(KnownX)) or 
/// (COLUMNS(KnownY) = 1 and ROWS(KnownY) = ROWS(KnownX) and COLUMNS(KnownX) = 
/// COLUMNS(NewX)) or (COLUMNS(KnownY) = COLUMNS(KnownX) and ROWS(KnownY) = 1 
/// and ROWS(KnownX) = ROWS(NewX))
///
/// __Semantics__:
/// 
/// •KnownY: The set of known y-values to be used to determine the regression 
/// equation
/// 
/// •KnownX: The set of known x-values to be used to determine the regression 
/// equation. If omitted or an empty parameter, it is set to the sequence 
/// 1,2,3,…,k , where
/// 
/// k = ROWS(KnownY) ∙ COLUMNS(KnownY)
/// 
/// •NewX: The set of x-values for which predicted y-values are to be 
/// calculated. If omitted or an empty parameter, it is set to KnownX.
/// 
/// Const: If set to FALSE, the model constant a is equal to 0.
/// LOGEST(KnownY ; KnownX; Const; FALSE) either returns an error or an array 
/// with 1 row and n+1 columns. If it returns an error then so does GROWTH. If 
/// it returns an array, we call the entries in that array
/// .
/// Let
/// denote the entry in the ith row and jth column of NewX.
/// If COLUMNS(KnownY ) = COLUMNS(KnownX) and ROWS(KnownY ) = ROWS(KnownX), 
/// then GROWTH returns an array with ROWS(NewX) rows and COLUMNS(NewX) column, 
/// such that the entry in its ith row and jth column is
/// .
/// Otherwise, if COLUMNS(KnownY ) = 1 and ROWS(KnownY ) = ROWS(KnownX) and 
/// COLUMNS(KnownX) = COLUMNS(NewX), then GROWTH returns an array with 
/// ROWS(NewX) rows and 1 column, such that the entry in the ith row is
/// .
/// Otherwise, if COLUMNS(KnownY ) = COLUMNS(KnownX) and ROWS(KnownY ) = 1 and 
/// ROWS(KnownX) = ROWS(NewX), then GROWTH returns an array with 1 row and 
/// COLUMNS(NewX) columns, such that the entry in the jth column is
/// .
///
/// __See also__: [crate::of::columns()], [crate::of::rows()], [crate::of::logest()], [crate::of::trend()], [crate::of::growth()], [crate::of::growth__()], [crate::of::growth___()], 
#[inline]
pub fn growth_<A: Array, B: Array>(known_y: A, known_x: B) -> FnArray2<A, B> {
    FnArray2("GROWTH", known_y, known_x)
}

/// Returns predicted values based on an exponential regression.
///
/// [documentfoundation->GROWTH](https://wiki.documentfoundation.org/Documentation/Calc_Functions/GROWTH)
///
/// __Syntax__: 
/// ```ods
///     GROWTH( KnownY: Array; KnownX: Array; NewX: Array )
/// ```
///
/// __Constraints__:
/// (COLUMNS(KnownY) = COLUMNS(KnownX) and ROWS(KnownY) = ROWS(KnownX)) or 
/// (COLUMNS(KnownY) = 1 and ROWS(KnownY) = ROWS(KnownX) and COLUMNS(KnownX) = 
/// COLUMNS(NewX)) or (COLUMNS(KnownY) = COLUMNS(KnownX) and ROWS(KnownY) = 1 
/// and ROWS(KnownX) = ROWS(NewX))
///
/// __Semantics__:
/// 
/// •KnownY: The set of known y-values to be used to determine the regression 
/// equation
/// 
/// •KnownX: The set of known x-values to be used to determine the regression 
/// equation. If omitted or an empty parameter, it is set to the sequence 
/// 1,2,3,…,k , where
/// 
/// k = ROWS(KnownY) ∙ COLUMNS(KnownY)
/// 
/// •NewX: The set of x-values for which predicted y-values are to be 
/// calculated. If omitted or an empty parameter, it is set to KnownX.
/// 
/// Const: If set to FALSE, the model constant a is equal to 0.
/// LOGEST(KnownY ; KnownX; Const; FALSE) either returns an error or an array 
/// with 1 row and n+1 columns. If it returns an error then so does GROWTH. If 
/// it returns an array, we call the entries in that array
/// .
/// Let
/// denote the entry in the ith row and jth column of NewX.
/// If COLUMNS(KnownY ) = COLUMNS(KnownX) and ROWS(KnownY ) = ROWS(KnownX), 
/// then GROWTH returns an array with ROWS(NewX) rows and COLUMNS(NewX) column, 
/// such that the entry in its ith row and jth column is
/// .
/// Otherwise, if COLUMNS(KnownY ) = 1 and ROWS(KnownY ) = ROWS(KnownX) and 
/// COLUMNS(KnownX) = COLUMNS(NewX), then GROWTH returns an array with 
/// ROWS(NewX) rows and 1 column, such that the entry in the ith row is
/// .
/// Otherwise, if COLUMNS(KnownY ) = COLUMNS(KnownX) and ROWS(KnownY ) = 1 and 
/// ROWS(KnownX) = ROWS(NewX), then GROWTH returns an array with 1 row and 
/// COLUMNS(NewX) columns, such that the entry in the jth column is
/// .
///
/// __See also__: [crate::of::columns()], [crate::of::rows()], [crate::of::logest()], [crate::of::trend()], [crate::of::growth()], [crate::of::growth_()], [crate::of::growth___()], 
#[inline]
pub fn growth__<A: Array, B: Array, C: Array>(known_y: A, known_x: B, new_x: C) -> FnArray3<A, B, C> {
    FnArray3("GROWTH", known_y, known_x, new_x)
}

/// Returns predicted values based on an exponential regression.
///
/// [documentfoundation->GROWTH](https://wiki.documentfoundation.org/Documentation/Calc_Functions/GROWTH)
///
/// __Syntax__: 
/// ```ods
///     GROWTH( KnownY: Array; KnownX: Array; NewX: Array; Const: Logical )
/// ```
///
/// __Constraints__:
/// (COLUMNS(KnownY) = COLUMNS(KnownX) and ROWS(KnownY) = ROWS(KnownX)) or 
/// (COLUMNS(KnownY) = 1 and ROWS(KnownY) = ROWS(KnownX) and COLUMNS(KnownX) = 
/// COLUMNS(NewX)) or (COLUMNS(KnownY) = COLUMNS(KnownX) and ROWS(KnownY) = 1 
/// and ROWS(KnownX) = ROWS(NewX))
///
/// __Semantics__:
/// 
/// •KnownY: The set of known y-values to be used to determine the regression 
/// equation
/// 
/// •KnownX: The set of known x-values to be used to determine the regression 
/// equation. If omitted or an empty parameter, it is set to the sequence 
/// 1,2,3,…,k , where
/// 
/// k = ROWS(KnownY) ∙ COLUMNS(KnownY)
/// 
/// •NewX: The set of x-values for which predicted y-values are to be 
/// calculated. If omitted or an empty parameter, it is set to KnownX.
/// 
/// Const: If set to FALSE, the model constant a is equal to 0.
/// LOGEST(KnownY ; KnownX; Const; FALSE) either returns an error or an array 
/// with 1 row and n+1 columns. If it returns an error then so does GROWTH. If 
/// it returns an array, we call the entries in that array
/// .
/// Let
/// denote the entry in the ith row and jth column of NewX.
/// If COLUMNS(KnownY ) = COLUMNS(KnownX) and ROWS(KnownY ) = ROWS(KnownX), 
/// then GROWTH returns an array with ROWS(NewX) rows and COLUMNS(NewX) column, 
/// such that the entry in its ith row and jth column is
/// .
/// Otherwise, if COLUMNS(KnownY ) = 1 and ROWS(KnownY ) = ROWS(KnownX) and 
/// COLUMNS(KnownX) = COLUMNS(NewX), then GROWTH returns an array with 
/// ROWS(NewX) rows and 1 column, such that the entry in the ith row is
/// .
/// Otherwise, if COLUMNS(KnownY ) = COLUMNS(KnownX) and ROWS(KnownY ) = 1 and 
/// ROWS(KnownX) = ROWS(NewX), then GROWTH returns an array with 1 row and 
/// COLUMNS(NewX) columns, such that the entry in the jth column is
/// .
///
/// __See also__: [crate::of::columns()], [crate::of::rows()], [crate::of::logest()], [crate::of::trend()], [crate::of::growth()], [crate::of::growth_()], [crate::of::growth__()], 
#[inline]
pub fn growth___<A: Array, B: Array, C: Array, D: Logical>(known_y: A, known_x: B, new_x: C, const_: D) -> FnArray4<A, B, C, D> {
    FnArray4("GROWTH", known_y, known_x, new_x, const_)
}

/// returns the harmonic mean of a sequence
///
/// [documentfoundation->HARMEAN](https://wiki.documentfoundation.org/Documentation/Calc_Functions/HARMEAN)
///
/// __Syntax__: 
/// ```ods
///     HARMEAN({ N: NumberSequenceList}+ )
/// ```
///
/// __Semantics__:
/// Returns the harmonic mean of a given sequence. That means
/// 
/// where a1,a2,...,an are the numbers of the sequence N and n is a result of 
/// calling COUNT(N).
///
/// __See also__: [crate::of::count()], 
#[inline]
pub fn harmean<A: Sequence>(n: A) -> FnNumber1<A> {
    FnNumber1("HARMEAN", n)
}

/// The hypergeometric distribution returns the number of successes in a 
/// sequence of n draws from a finite population without replacement.
///
/// [documentfoundation->HYPGEOMDIST](https://wiki.documentfoundation.org/Documentation/Calc_Functions/HYPGEOMDIST)
///
/// __Syntax__: 
/// ```ods
///     HYPGEOMDIST( X: Integer; T: Integer; M: Integer; N: Integer )
/// ```
///
/// __Constraints__:
/// 0 ≤ X ≤ T ≤ N, 0 ≤ M ≤ N
///
/// __Semantics__:
/// 
/// •X: the number of successes in T trials
/// 
/// •T: the number of trials
/// 
/// •M: the number of successes in the population
/// 
/// •N: the total population
/// 
/// •Cumulative : a Logical parameter.
/// 
/// If Cumulative is FALSE, return the probability of exactly X successes. If 
/// Cumulative is TRUE, return the probability of at most X successes. If 
/// omitted, FALSE is assumed.
/// 
/// If Cumulative is FALSE, HYPGEOMDIST returns
/// 
/// If Cumulative is TRUE, HYPGEOMDIST returns
///
/// __See also__: [crate::of::hypgeomdist_()], 
#[inline]
pub fn hypgeomdist<A: Number, B: Number, C: Number, D: Number>(x: A, t: B, m: C, n: D) -> FnNumber4<A, B, C, D> {
    FnNumber4("HYPGEOMDIST", x, t, m, n)
}

/// The hypergeometric distribution returns the number of successes in a 
/// sequence of n draws from a finite population without replacement.
///
/// [documentfoundation->HYPGEOMDIST](https://wiki.documentfoundation.org/Documentation/Calc_Functions/HYPGEOMDIST)
///
/// __Syntax__: 
/// ```ods
///     HYPGEOMDIST( X: Integer; T: Integer; M: Integer; N: Integer; Cumulative: Logical )
/// ```
///
/// __Constraints__:
/// 0 ≤ X ≤ T ≤ N, 0 ≤ M ≤ N
///
/// __Semantics__:
/// 
/// •X: the number of successes in T trials
/// 
/// •T: the number of trials
/// 
/// •M: the number of successes in the population
/// 
/// •N: the total population
/// 
/// •Cumulative : a Logical parameter.
/// 
/// If Cumulative is FALSE, return the probability of exactly X successes. If 
/// Cumulative is TRUE, return the probability of at most X successes. If 
/// omitted, FALSE is assumed.
/// 
/// If Cumulative is FALSE, HYPGEOMDIST returns
/// 
/// If Cumulative is TRUE, HYPGEOMDIST returns
///
/// __See also__: [crate::of::hypgeomdist()], 
#[inline]
pub fn hypgeomdist_<A: Number, B: Number, C: Number, D: Number, E: Logical>(x: A, t: B, m: C, n: D, cumulative: E) -> FnNumber5<A, B, C, D, E> {
    FnNumber5("HYPGEOMDIST", x, t, m, n, cumulative)
}

/// Returns the y-intercept of the linear regression line for the given data.
///
/// [documentfoundation->INTERCEPT](https://wiki.documentfoundation.org/Documentation/Calc_Functions/INTERCEPT)
///
/// __Syntax__: 
/// ```ods
///     INTERCEPT( Data_Y: Array; Data_X: Array )
/// ```
///
/// __Constraints__:
/// COLUMNS(Data_X) = COLUMNS(Data_Y), ROWS(Data_X) = ROWS(Data_Y)
///
/// __Semantics__:
/// 
/// INTERCEPT returns the intercept (a) calculated as described in 6.18.41 for 
/// the function call LINEST(Data_Y,Data_X,FALSE()).
/// 
/// For an empty element or an element of type Text or Boolean in Data_Y the 
/// element at the corresponding position of Data_X is ignored, and vice versa.
///
/// __See also__: [crate::of::columns()], [crate::of::rows()], 
#[inline]
pub fn intercept<A: Array, B: Array>(data_y: A, data_x: B) -> FnNumber2<A, B> {
    FnNumber2("INTERCEPT", data_y, data_x)
}

/// Return the kurtosis (“peakedness”) of a data set.
///
/// [documentfoundation->KURT](https://wiki.documentfoundation.org/Documentation/Calc_Functions/KURT)
///
/// __Syntax__: 
/// ```ods
///     KURT({ X: NumberSequenceList}+ )
/// ```
///
/// __Constraints__:
/// COUNT(X) ≥ 4, STDEV(X) ≠ 0
///
/// __Semantics__:
/// 
/// Kurtosis characterizes the relative peakedness or flatness of a 
/// distribution compared with the normal distribution. Positive kurtosis 
/// indicates a relatively peaked distribution (compared to the normal 
/// distribution), while negative kurtosis indicates a relatively flat 
/// distribution.
/// 
/// where s is the sample standard deviation, and n is the number of numbers.
///
/// __See also__: [crate::of::stdev()], 
#[inline]
pub fn kurt<A: Sequence>(x: A) -> FnNumber1<A> {
    FnNumber1("KURT", x)
}

/// Finds the nth largest value in a list.
///
/// [documentfoundation->LARGE](https://wiki.documentfoundation.org/Documentation/Calc_Functions/LARGE)
///
/// __Syntax__: 
/// ```ods
///     LARGE( List: NumberSequenceList; N: Number|Array )
/// ```
///
/// __Constraints__:
/// ROUNDUP(N;0) = N. If the resulting N is <1 or larger than the size of List, 
/// Error is returned
///
/// __Semantics__:
/// If N is an array of numbers, an array of largest values is returned.
///
/// __See also__: [crate::of::small()], [crate::of::roundup()], 
#[inline]
pub fn large<A: Sequence, B: NumberOrArray>(list: A, n: B) -> FnArray2<A, B> {
    FnArray2("LARGE", list, n)
}

/// Returns the parameters of the (simple or multiple) linear regression 
/// equation for the given data and, optionally, statistics on this regression.
///
/// [documentfoundation->LINEST](https://wiki.documentfoundation.org/Documentation/Calc_Functions/LINEST)
///
/// __Syntax__: 
/// ```ods
///     LINEST( KnownY: Array )
/// ```
///
/// __Constraints__:
/// (COLUMNS(KnownY) = COLUMNS(KnownX) and ROWS(KnownY) = ROWS(KnownX)) or 
/// (COLUMNS(KnownY) = 1 and ROWS(KnownY) = ROWS(KnownX)) or (COLUMNS(KnownY) = 
/// COLUMNS(KnownX) and ROWS(KnownY) = 1)
///
/// __Semantics__:
/// 
/// •KnownY: The set of y-values for the equation
/// 
/// •KnownX: The set of x-values for the equation. If omitted or an empty 
/// parameter, it is set to the sequence 1,2,3,…,k , where k = ROWS(KnownY) 
/// ∙ COLUMNS(KnownY).
/// 
/// •Const: If set to FALSE, the model constant a is equal to 0.
/// 
/// •Stats: If FALSE, only the regression coefficient is to be calculated. If 
/// set to TRUE, the result will include other statistical data.
/// 
/// If any of the entries in KnownY and KnownX do not convert to Number, LINEST 
/// returns an error.
/// 
/// ** Some formulas **
///
/// __See also__: [crate::of::columns()], [crate::of::rows()], [crate::of::linest_()], [crate::of::linest__()], [crate::of::linest___()], 
#[inline]
pub fn linest<A: Array>(known_y: A) -> FnArray1<A> {
    FnArray1("LINEST", known_y)
}

/// Returns the parameters of the (simple or multiple) linear regression 
/// equation for the given data and, optionally, statistics on this regression.
///
/// [documentfoundation->LINEST](https://wiki.documentfoundation.org/Documentation/Calc_Functions/LINEST)
///
/// __Syntax__: 
/// ```ods
///     LINEST( KnownY: Array; KnownX: Array )
/// ```
///
/// __Constraints__:
/// (COLUMNS(KnownY) = COLUMNS(KnownX) and ROWS(KnownY) = ROWS(KnownX)) or 
/// (COLUMNS(KnownY) = 1 and ROWS(KnownY) = ROWS(KnownX)) or (COLUMNS(KnownY) = 
/// COLUMNS(KnownX) and ROWS(KnownY) = 1)
///
/// __Semantics__:
/// 
/// •KnownY: The set of y-values for the equation
/// 
/// •KnownX: The set of x-values for the equation. If omitted or an empty 
/// parameter, it is set to the sequence 1,2,3,…,k , where k = ROWS(KnownY) 
/// ∙ COLUMNS(KnownY).
/// 
/// •Const: If set to FALSE, the model constant a is equal to 0.
/// 
/// •Stats: If FALSE, only the regression coefficient is to be calculated. If 
/// set to TRUE, the result will include other statistical data.
/// 
/// If any of the entries in KnownY and KnownX do not convert to Number, LINEST 
/// returns an error.
/// 
/// ** Some formulas **
///
/// __See also__: [crate::of::columns()], [crate::of::rows()], [crate::of::linest()], [crate::of::linest__()], [crate::of::linest___()], 
#[inline]
pub fn linest_<A: Array, B: Array>(known_y: A, known_x: B) -> FnArray2<A, B> {
    FnArray2("LINEST", known_y, known_x)
}

/// Returns the parameters of the (simple or multiple) linear regression 
/// equation for the given data and, optionally, statistics on this regression.
///
/// [documentfoundation->LINEST](https://wiki.documentfoundation.org/Documentation/Calc_Functions/LINEST)
///
/// __Syntax__: 
/// ```ods
///     LINEST( KnownY: Array; KnownX: Array; Const: Logical )
/// ```
///
/// __Constraints__:
/// (COLUMNS(KnownY) = COLUMNS(KnownX) and ROWS(KnownY) = ROWS(KnownX)) or 
/// (COLUMNS(KnownY) = 1 and ROWS(KnownY) = ROWS(KnownX)) or (COLUMNS(KnownY) = 
/// COLUMNS(KnownX) and ROWS(KnownY) = 1)
///
/// __Semantics__:
/// 
/// •KnownY: The set of y-values for the equation
/// 
/// •KnownX: The set of x-values for the equation. If omitted or an empty 
/// parameter, it is set to the sequence 1,2,3,…,k , where k = ROWS(KnownY) 
/// ∙ COLUMNS(KnownY).
/// 
/// •Const: If set to FALSE, the model constant a is equal to 0.
/// 
/// •Stats: If FALSE, only the regression coefficient is to be calculated. If 
/// set to TRUE, the result will include other statistical data.
/// 
/// If any of the entries in KnownY and KnownX do not convert to Number, LINEST 
/// returns an error.
/// 
/// ** Some formulas **
///
/// __See also__: [crate::of::columns()], [crate::of::rows()], [crate::of::linest()], [crate::of::linest_()], [crate::of::linest___()], 
#[inline]
pub fn linest__<A: Array, B: Array, C: Logical>(known_y: A, known_x: B, const_: C) -> FnArray3<A, B, C> {
    FnArray3("LINEST", known_y, known_x, const_)
}

/// Returns the parameters of the (simple or multiple) linear regression 
/// equation for the given data and, optionally, statistics on this regression.
///
/// [documentfoundation->LINEST](https://wiki.documentfoundation.org/Documentation/Calc_Functions/LINEST)
///
/// __Syntax__: 
/// ```ods
///     LINEST( KnownY: Array; KnownX: Array; Const: Logical; Stats: Logical )
/// ```
///
/// __Constraints__:
/// (COLUMNS(KnownY) = COLUMNS(KnownX) and ROWS(KnownY) = ROWS(KnownX)) or 
/// (COLUMNS(KnownY) = 1 and ROWS(KnownY) = ROWS(KnownX)) or (COLUMNS(KnownY) = 
/// COLUMNS(KnownX) and ROWS(KnownY) = 1)
///
/// __Semantics__:
/// 
/// •KnownY: The set of y-values for the equation
/// 
/// •KnownX: The set of x-values for the equation. If omitted or an empty 
/// parameter, it is set to the sequence 1,2,3,…,k , where k = ROWS(KnownY) 
/// ∙ COLUMNS(KnownY).
/// 
/// •Const: If set to FALSE, the model constant a is equal to 0.
/// 
/// •Stats: If FALSE, only the regression coefficient is to be calculated. If 
/// set to TRUE, the result will include other statistical data.
/// 
/// If any of the entries in KnownY and KnownX do not convert to Number, LINEST 
/// returns an error.
/// 
/// ** Some formulas **
///
/// __See also__: [crate::of::columns()], [crate::of::rows()], [crate::of::linest()], [crate::of::linest_()], [crate::of::linest__()], 
#[inline]
pub fn linest___<A: Array, B: Array, C: Logical, D: Logical>(known_y: A, known_x: B, const_: C, stats: D) -> FnArray4<A, B, C, D> {
    FnArray4("LINEST", known_y, known_x, const_, stats)
}

/// Returns the parameters of an exponential regression equation for the given 
/// data obtained by linearizing this intrinsically linear response function 
/// and returns, optionally, statistics on this regression.
///
/// [documentfoundation->LOGEST](https://wiki.documentfoundation.org/Documentation/Calc_Functions/LOGEST)
///
/// __Syntax__: 
/// ```ods
///     LOGEST( KnownY: Array )
/// ```
///
/// __Constraints__:
/// (COLUMNS(KnownY) = COLUMNS(KnownX) and ROWS(KnownY) = ROWS(KnownX)) or 
/// (COLUMNS(KnownY) = 1 and ROWS(KnownY) = ROWS(KnownX)) or (COLUMNS(KnownY) = 
/// COLUMNS(KnownX) and ROWS(KnownY) = 1)
///
/// __Semantics__:
/// 
/// •KnownY: The set of y-values for the equation
/// 
/// •KnownX: The set of x-values for the equation. If omitted or an empty 
/// parameter, it is set to the sequence 1,2,3,…,k, where k = ROWS(KnownY) 
/// ∙ COLUMNS(KnownY).
/// 
/// •Const: If set to FALSE, the model constant a is equal to 0.
/// 
/// •Stats: If FALSE, only the regression coefficient is to be calculated. If 
/// set to TRUE, the result will include other statistical data.
/// 
/// If any of the entries in KnownY and KnownX do not convert to Number or if 
/// any of the entries in KnownY is negative, LOGEST returns an error.
/// 
/// ** Some formulas **
///
/// __See also__: [crate::of::columns()], [crate::of::rows()], [crate::of::logest_()], [crate::of::logest__()], [crate::of::logest___()], 
#[inline]
pub fn logest<A: Array>(known_y: A) -> FnArray1<A> {
    FnArray1("LOGEST", known_y)
}

/// Returns the parameters of an exponential regression equation for the given 
/// data obtained by linearizing this intrinsically linear response function 
/// and returns, optionally, statistics on this regression.
///
/// [documentfoundation->LOGEST](https://wiki.documentfoundation.org/Documentation/Calc_Functions/LOGEST)
///
/// __Syntax__: 
/// ```ods
///     LOGEST( KnownY: Array; KnownX: Array )
/// ```
///
/// __Constraints__:
/// (COLUMNS(KnownY) = COLUMNS(KnownX) and ROWS(KnownY) = ROWS(KnownX)) or 
/// (COLUMNS(KnownY) = 1 and ROWS(KnownY) = ROWS(KnownX)) or (COLUMNS(KnownY) = 
/// COLUMNS(KnownX) and ROWS(KnownY) = 1)
///
/// __Semantics__:
/// 
/// •KnownY: The set of y-values for the equation
/// 
/// •KnownX: The set of x-values for the equation. If omitted or an empty 
/// parameter, it is set to the sequence 1,2,3,…,k, where k = ROWS(KnownY) 
/// ∙ COLUMNS(KnownY).
/// 
/// •Const: If set to FALSE, the model constant a is equal to 0.
/// 
/// •Stats: If FALSE, only the regression coefficient is to be calculated. If 
/// set to TRUE, the result will include other statistical data.
/// 
/// If any of the entries in KnownY and KnownX do not convert to Number or if 
/// any of the entries in KnownY is negative, LOGEST returns an error.
/// 
/// ** Some formulas **
///
/// __See also__: [crate::of::columns()], [crate::of::rows()], [crate::of::logest()], [crate::of::logest__()], [crate::of::logest___()], 
#[inline]
pub fn logest_<A: Array, B: Array>(known_y: A, known_x: B) -> FnArray2<A, B> {
    FnArray2("LOGEST", known_y, known_x)
}

/// Returns the parameters of an exponential regression equation for the given 
/// data obtained by linearizing this intrinsically linear response function 
/// and returns, optionally, statistics on this regression.
///
/// [documentfoundation->LOGEST](https://wiki.documentfoundation.org/Documentation/Calc_Functions/LOGEST)
///
/// __Syntax__: 
/// ```ods
///     LOGEST( KnownY: Array; KnownX: Array; Const: Logical )
/// ```
///
/// __Constraints__:
/// (COLUMNS(KnownY) = COLUMNS(KnownX) and ROWS(KnownY) = ROWS(KnownX)) or 
/// (COLUMNS(KnownY) = 1 and ROWS(KnownY) = ROWS(KnownX)) or (COLUMNS(KnownY) = 
/// COLUMNS(KnownX) and ROWS(KnownY) = 1)
///
/// __Semantics__:
/// 
/// •KnownY: The set of y-values for the equation
/// 
/// •KnownX: The set of x-values for the equation. If omitted or an empty 
/// parameter, it is set to the sequence 1,2,3,…,k, where k = ROWS(KnownY) 
/// ∙ COLUMNS(KnownY).
/// 
/// •Const: If set to FALSE, the model constant a is equal to 0.
/// 
/// •Stats: If FALSE, only the regression coefficient is to be calculated. If 
/// set to TRUE, the result will include other statistical data.
/// 
/// If any of the entries in KnownY and KnownX do not convert to Number or if 
/// any of the entries in KnownY is negative, LOGEST returns an error.
/// 
/// ** Some formulas **
///
/// __See also__: [crate::of::columns()], [crate::of::rows()], [crate::of::logest()], [crate::of::logest_()], [crate::of::logest___()], 
#[inline]
pub fn logest__<A: Array, B: Array, C: Logical>(known_y: A, known_x: B, const_: C) -> FnArray3<A, B, C> {
    FnArray3("LOGEST", known_y, known_x, const_)
}

/// Returns the parameters of an exponential regression equation for the given 
/// data obtained by linearizing this intrinsically linear response function 
/// and returns, optionally, statistics on this regression.
///
/// [documentfoundation->LOGEST](https://wiki.documentfoundation.org/Documentation/Calc_Functions/LOGEST)
///
/// __Syntax__: 
/// ```ods
///     LOGEST( KnownY: Array; KnownX: Array; Const: Logical; Stats: Logical )
/// ```
///
/// __Constraints__:
/// (COLUMNS(KnownY) = COLUMNS(KnownX) and ROWS(KnownY) = ROWS(KnownX)) or 
/// (COLUMNS(KnownY) = 1 and ROWS(KnownY) = ROWS(KnownX)) or (COLUMNS(KnownY) = 
/// COLUMNS(KnownX) and ROWS(KnownY) = 1)
///
/// __Semantics__:
/// 
/// •KnownY: The set of y-values for the equation
/// 
/// •KnownX: The set of x-values for the equation. If omitted or an empty 
/// parameter, it is set to the sequence 1,2,3,…,k, where k = ROWS(KnownY) 
/// ∙ COLUMNS(KnownY).
/// 
/// •Const: If set to FALSE, the model constant a is equal to 0.
/// 
/// •Stats: If FALSE, only the regression coefficient is to be calculated. If 
/// set to TRUE, the result will include other statistical data.
/// 
/// If any of the entries in KnownY and KnownX do not convert to Number or if 
/// any of the entries in KnownY is negative, LOGEST returns an error.
/// 
/// ** Some formulas **
///
/// __See also__: [crate::of::columns()], [crate::of::rows()], [crate::of::logest()], [crate::of::logest_()], [crate::of::logest__()], 
#[inline]
pub fn logest___<A: Array, B: Array, C: Logical, D: Logical>(known_y: A, known_x: B, const_: C, stats: D) -> FnArray4<A, B, C, D> {
    FnArray4("LOGEST", known_y, known_x, const_, stats)
}

/// returns the inverse of LOGNORMDIST(x;Mean;StandardDeviation,TRUE()).
///
/// [documentfoundation->LOGINV](https://wiki.documentfoundation.org/Documentation/Calc_Functions/LOGINV)
///
/// __Syntax__: 
/// ```ods
///     LOGINV( P: Number )
/// ```
///
/// __Constraints__:
/// StandardDeviation > 0 and 0 < P < 1.
///
/// __Semantics__:
/// LOGINV returns the unique number x such that 
/// LOGNORMDIST(x;Mean;StandardDeviation;TRUE()) = P.
///
/// __See also__: [crate::of::lognormdist()], [crate::of::loginv_()], [crate::of::loginv__()], 
#[inline]
pub fn loginv<A: Number>(p: A) -> FnNumber1<A> {
    FnNumber1("LOGINV", p)
}

/// returns the inverse of LOGNORMDIST(x;Mean;StandardDeviation,TRUE()).
///
/// [documentfoundation->LOGINV](https://wiki.documentfoundation.org/Documentation/Calc_Functions/LOGINV)
///
/// __Syntax__: 
/// ```ods
///     LOGINV( P: Number; Mean: Number )
/// ```
///
/// __Constraints__:
/// StandardDeviation > 0 and 0 < P < 1.
///
/// __Semantics__:
/// LOGINV returns the unique number x such that 
/// LOGNORMDIST(x;Mean;StandardDeviation;TRUE()) = P.
///
/// __See also__: [crate::of::lognormdist()], [crate::of::loginv()], [crate::of::loginv__()], 
#[inline]
pub fn loginv_<A: Number, B: Number>(p: A, mean: B) -> FnNumber2<A, B> {
    FnNumber2("LOGINV", p, mean)
}

/// returns the inverse of LOGNORMDIST(x;Mean;StandardDeviation,TRUE()).
///
/// [documentfoundation->LOGINV](https://wiki.documentfoundation.org/Documentation/Calc_Functions/LOGINV)
///
/// __Syntax__: 
/// ```ods
///     LOGINV( P: Number; Mean: Number; StandardDeviation: Number )
/// ```
///
/// __Constraints__:
/// StandardDeviation > 0 and 0 < P < 1.
///
/// __Semantics__:
/// LOGINV returns the unique number x such that 
/// LOGNORMDIST(x;Mean;StandardDeviation;TRUE()) = P.
///
/// __See also__: [crate::of::lognormdist()], [crate::of::loginv()], [crate::of::loginv_()], 
#[inline]
pub fn loginv__<A: Number, B: Number, C: Number>(p: A, mean: B, standard_deviation: C) -> FnNumber3<A, B, C> {
    FnNumber3("LOGINV", p, mean, standard_deviation)
}

/// returns the value of the probability density function or the cumulative 
/// distribution function for the lognormal distribution with the mean and 
/// standard deviation given.
///
/// [documentfoundation->LOGNORMDIST](https://wiki.documentfoundation.org/Documentation/Calc_Functions/LOGNORMDIST)
///
/// __Syntax__: 
/// ```ods
///     LOGNORMDIST( X: Number )
/// ```
///
/// __Constraints__:
/// sigma > 0; X > 0 if Cumulative is FALSE
///
/// __Semantics__:
/// If Cumulative is FALSE, LOGNORMDIST returns the value
/// 
/// If Cumulative is TRUE, LOGNORMDIST returns the value
/// 
/// if X > 0 and 0 otherwise.
///
/// __See also__: [crate::of::lognormdist_()], [crate::of::lognormdist__()], [crate::of::lognormdist___()], 
#[inline]
pub fn lognormdist<A: Number>(x: A) -> FnNumber1<A> {
    FnNumber1("LOGNORMDIST", x)
}

/// returns the value of the probability density function or the cumulative 
/// distribution function for the lognormal distribution with the mean and 
/// standard deviation given.
///
/// [documentfoundation->LOGNORMDIST](https://wiki.documentfoundation.org/Documentation/Calc_Functions/LOGNORMDIST)
///
/// __Syntax__: 
/// ```ods
///     LOGNORMDIST( X: Number; mu: Number )
/// ```
///
/// __Constraints__:
/// sigma > 0; X > 0 if Cumulative is FALSE
///
/// __Semantics__:
/// If Cumulative is FALSE, LOGNORMDIST returns the value
/// 
/// If Cumulative is TRUE, LOGNORMDIST returns the value
/// 
/// if X > 0 and 0 otherwise.
///
/// __See also__: [crate::of::lognormdist()], [crate::of::lognormdist__()], [crate::of::lognormdist___()], 
#[inline]
pub fn lognormdist_<A: Number, B: Number>(x: A, mu: B) -> FnNumber2<A, B> {
    FnNumber2("LOGNORMDIST", x, mu)
}

/// returns the value of the probability density function or the cumulative 
/// distribution function for the lognormal distribution with the mean and 
/// standard deviation given.
///
/// [documentfoundation->LOGNORMDIST](https://wiki.documentfoundation.org/Documentation/Calc_Functions/LOGNORMDIST)
///
/// __Syntax__: 
/// ```ods
///     LOGNORMDIST( X: Number; mu: Number; sigma: Number )
/// ```
///
/// __Constraints__:
/// sigma > 0; X > 0 if Cumulative is FALSE
///
/// __Semantics__:
/// If Cumulative is FALSE, LOGNORMDIST returns the value
/// 
/// If Cumulative is TRUE, LOGNORMDIST returns the value
/// 
/// if X > 0 and 0 otherwise.
///
/// __See also__: [crate::of::lognormdist()], [crate::of::lognormdist_()], [crate::of::lognormdist___()], 
#[inline]
pub fn lognormdist__<A: Number, B: Number, C: Number>(x: A, mu: B, sigma: C) -> FnNumber3<A, B, C> {
    FnNumber3("LOGNORMDIST", x, mu, sigma)
}

/// returns the value of the probability density function or the cumulative 
/// distribution function for the lognormal distribution with the mean and 
/// standard deviation given.
///
/// [documentfoundation->LOGNORMDIST](https://wiki.documentfoundation.org/Documentation/Calc_Functions/LOGNORMDIST)
///
/// __Syntax__: 
/// ```ods
///     LOGNORMDIST( X: Number; mu: Number; sigma: Number; Cumulative: Logical )
/// ```
///
/// __Constraints__:
/// sigma > 0; X > 0 if Cumulative is FALSE
///
/// __Semantics__:
/// If Cumulative is FALSE, LOGNORMDIST returns the value
/// 
/// If Cumulative is TRUE, LOGNORMDIST returns the value
/// 
/// if X > 0 and 0 otherwise.
///
/// __See also__: [crate::of::lognormdist()], [crate::of::lognormdist_()], [crate::of::lognormdist__()], 
#[inline]
pub fn lognormdist___<A: Number, B: Number, C: Number, D: Logical>(x: A, mu: B, sigma: C, cumulative: D) -> FnNumber4<A, B, C, D> {
    FnNumber4("LOGNORMDIST", x, mu, sigma, cumulative)
}

/// Return the maximum from a set of numbers.
///
/// [documentfoundation->MAX](https://wiki.documentfoundation.org/Documentation/Calc_Functions/MAX)
///
/// __Syntax__: 
/// ```ods
///     MAX({ N: NumberSequenceList}+ )
/// ```
///
/// __Constraints__:
/// None.
///
/// __Semantics__:
/// Returns the value of the maximum number in the list passed in. Non-numbers 
/// are ignored. Note that if Logical types are a distinct type, they are not 
/// included.
///
/// __See also__: [crate::of::maxa()], [crate::of::min()], 
#[inline]
pub fn max<A: Sequence>(n: A) -> FnNumber1<A> {
    FnNumber1("MAX", n)
}

/// Return the maximum from a set of values, including values of type Text and 
/// Logical.
///
/// [documentfoundation->MAXA](https://wiki.documentfoundation.org/Documentation/Calc_Functions/MAXA)
///
/// __Syntax__: 
/// ```ods
///     MAXA({ N: Any}+ )
/// ```
///
/// __Constraints__:
/// None.
///
/// __Semantics__:
/// A variation of the MAX function that includes values of type Text and 
/// Logical. Text values are treated as number 0. Logical TRUE is treated as 1, 
/// and FALSE is treated as 0. Empty cells are not included. Any N may be of 
/// type ReferenceList.
///
/// __See also__: [crate::of::max()], [crate::of::min()], [crate::of::mina()], 
#[inline]
pub fn maxa<A: Sequence>(n: A) -> FnNumber1<A> {
    FnNumber1("MAXA", n)
}

/// Returns the median (middle) value in the list.
///
/// [documentfoundation->MEDIAN](https://wiki.documentfoundation.org/Documentation/Calc_Functions/MEDIAN)
///
/// __Syntax__: 
/// ```ods
///     MEDIAN({ X: NumberSequenceList}+ )
/// ```
///
/// __Semantics__:
/// 
/// MEDIAN logically ranks the numbers (lowest to highest). If given an odd 
/// number of values, MEDIAN returns the middle value. If given an even number 
/// of values, MEDIAN returns the arithmetic average of the two middle values.
///
/// __See also__: 
#[inline]
pub fn median<A: Sequence>(x: A) -> FnNumber1<A> {
    FnNumber1("MEDIAN", x)
}

/// Return the minimum from a set of numbers.
///
/// [documentfoundation->MIN](https://wiki.documentfoundation.org/Documentation/Calc_Functions/MIN)
///
/// __Syntax__: 
/// ```ods
///     MIN({ N: NumberSequenceList}+ )
/// ```
///
/// __Constraints__:
/// None.
///
/// __Semantics__:
/// Returns the value of the minimum number in the list passed in. Returns zero 
/// if no numbers are provided in the list. What happens when MIN is provided 0 
/// parameters is implementation-defined, but MIN() with no parameters should 
/// return 0.
///
/// __See also__: [crate::of::max()], [crate::of::mina()], 
#[inline]
pub fn min<A: Sequence>(n: A) -> FnNumber1<A> {
    FnNumber1("MIN", n)
}

/// Return the minimum from a set of values, including values of type Text and 
/// Logical.
///
/// [documentfoundation->MINA](https://wiki.documentfoundation.org/Documentation/Calc_Functions/MINA)
///
/// __Syntax__: 
/// ```ods
///     MINA({ N: Any}+ )
/// ```
///
/// __Constraints__:
/// None.
///
/// __Semantics__:
/// A variation of the MIN function that includes values of type Text and 
/// Logical. Text values are treated as number 0. Logical TRUE is treated as 1, 
/// and FALSE is treated as 0. Empty cells are not included. What happens when 
/// MINA is provided 0 parameters is implementation-defined. Any N may be of 
/// type ReferenceList.
///
/// __See also__: [crate::of::min()], [crate::of::maxa()], 
#[inline]
pub fn mina<A: Sequence>(n: A) -> FnNumber1<A> {
    FnNumber1("MINA", n)
}

/// Returns the most common value in a data set.
///
/// [documentfoundation->MODE](https://wiki.documentfoundation.org/Documentation/Calc_Functions/MODE)
///
/// __Syntax__: 
/// ```ods
///     MODE({ N: NumberSequence}+ )
/// ```
///
/// __Semantics__:
/// Returns the most common value in a data set. If there are more than one 
/// values with the same largest frequency, returns the smallest value. If the 
/// number sequence does no contain at least two equal values, the MODE is not 
/// defined, as no most common value can be found, and an Error is returned.
///
/// __See also__: 
#[inline]
pub fn mode<A: Sequence>(n: A) -> FnNumber1<A> {
    FnNumber1("MODE", n)
}

/// Returns the negative binomial distribution.
///
/// [documentfoundation->NEGBINOMDIST](https://wiki.documentfoundation.org/Documentation/Calc_Functions/NEGBINOMDIST)
///
/// __Syntax__: 
/// ```ods
///     NEGBINOMDIST( X: Integer; R: Integer; Prob: Number )
/// ```
///
/// __Arguments__:
/// 
/// •X: The number of failures.
/// 
/// •R: The threshold number of successes.
/// 
/// •Prob: The probability of a success.
///
/// __Constraints__:
/// 
/// •If (X + R - 1) ≤ 0, NEGBINOMDIST returns an Error.
/// 
/// •If Prob < 0 or Prob > 1, NEGBINOMDIST returns an Error.
///
/// __Semantics__:
/// 
/// NEGBINOMDIST returns the probability that there will be X failures before 
/// the R-th success, when the constant probability of a success is Prob.
///
/// __Note__:
/// This function is similar to the binomial distribution, except that the 
/// number of successes is fixed, and the number of trials is variable. Like 
/// the binomial, trials are assumed to be independent.
///
/// __See also__: 
#[inline]
pub fn negbinomdist<A: Number, B: Number, C: Number>(x: A, r: B, prob: C) -> FnNumber3<A, B, C> {
    FnNumber3("NEGBINOMDIST", x, r, prob)
}

/// returns the value of the probability density function or the cumulative 
/// distribution function for the normal distribution with the mean and 
/// standard deviation given.
///
/// [documentfoundation->NORMDIST](https://wiki.documentfoundation.org/Documentation/Calc_Functions/NORMDIST)
///
/// __Syntax__: 
/// ```ods
///     NORMDIST( X: Number; Mean: Number; StandardDeviation: Number )
/// ```
///
/// __Constraints__:
/// StandardDeviation > 0.
///
/// __Semantics__:
/// In the following mu is Mean and sigma is StandardDeviation.
/// 
/// If Cumulative is FALSE, NORMDIST returns the value
/// 
/// If Cumulative is TRUE, NORMDIST returns the value
///
/// __See also__: [crate::of::legacy_normsdist()], [crate::of::normdist_()], 
#[inline]
pub fn normdist<A: Number, B: Number, C: Number>(x: A, mean: B, standard_deviation: C) -> FnNumber3<A, B, C> {
    FnNumber3("NORMDIST", x, mean, standard_deviation)
}

/// returns the value of the probability density function or the cumulative 
/// distribution function for the normal distribution with the mean and 
/// standard deviation given.
///
/// [documentfoundation->NORMDIST](https://wiki.documentfoundation.org/Documentation/Calc_Functions/NORMDIST)
///
/// __Syntax__: 
/// ```ods
///     NORMDIST( X: Number; Mean: Number; StandardDeviation: Number; Cumulative: Logical )
/// ```
///
/// __Constraints__:
/// StandardDeviation > 0.
///
/// __Semantics__:
/// In the following mu is Mean and sigma is StandardDeviation.
/// 
/// If Cumulative is FALSE, NORMDIST returns the value
/// 
/// If Cumulative is TRUE, NORMDIST returns the value
///
/// __See also__: [crate::of::legacy_normsdist()], [crate::of::normdist()], 
#[inline]
pub fn normdist_<A: Number, B: Number, C: Number, D: Logical>(x: A, mean: B, standard_deviation: C, cumulative: D) -> FnNumber4<A, B, C, D> {
    FnNumber4("NORMDIST", x, mean, standard_deviation, cumulative)
}

/// returns the inverse of NORMDIST(x;Mean;StandardDeviation,TRUE()).
///
/// [documentfoundation->NORMINV](https://wiki.documentfoundation.org/Documentation/Calc_Functions/NORMINV)
///
/// __Syntax__: 
/// ```ods
///     NORMINV( P: Number; Mean: Number; StandardDeviation: Number )
/// ```
///
/// __Constraints__:
/// StandardDeviation > 0 and 0 < P < 1.
///
/// __Semantics__:
/// NORMINV returns the unique number x such that 
/// NORMDIST(x;Mean;StandardDeviation;TRUE()) = P.
///
/// __See also__: [crate::of::normdist()], 
#[inline]
pub fn norminv<A: Number, B: Number, C: Number>(p: A, mean: B, standard_deviation: C) -> FnNumber3<A, B, C> {
    FnNumber3("NORMINV", p, mean, standard_deviation)
}

/// returns the value of the cumulative distribution function for the standard 
/// normal distribution.
///
/// [documentfoundation->LEGACY.NORMSDIST](https://wiki.documentfoundation.org/Documentation/Calc_Functions/LEGACY.NORMSDIST)
///
/// __Syntax__: 
/// ```ods
///     LEGACY.NORMSDIST( X: Number )
/// ```
///
/// __Constraints__:
/// None
///
/// __Semantics__:
/// LEGACY.NORMSDIST returns the value
/// 
/// This is exactly NORMDIST(X;0;1;TRUE()).
///
/// __See also__: [crate::of::normdist()], [crate::of::legacy_normsinv()], 
#[inline]
pub fn legacy_normsdist<A: Number>(x: A) -> FnNumber1<A> {
    FnNumber1("LEGACY.NORMSDIST", x)
}

/// returns the inverse of LEGACY.NORMSDIST(X).
///
/// [documentfoundation->LEGACY.NORMSINV](https://wiki.documentfoundation.org/Documentation/Calc_Functions/LEGACY.NORMSINV)
///
/// __Syntax__: 
/// ```ods
///     LEGACY.NORMSINV( P: Number )
/// ```
///
/// __Constraints__:
/// 0 < P < 1.
///
/// __Semantics__:
/// LEGACY.NORMSINV returns NORMINV (P).
///
/// __See also__: [crate::of::norminv()], [crate::of::legacy_normsdist()], 
#[inline]
pub fn legacy_normsinv<A: Number>(p: A) -> FnNumber1<A> {
    FnNumber1("LEGACY.NORMSINV", p)
}

/// PEARSON returns the Pearson correlation coefficient of two data sets
///
/// [documentfoundation->PEARSON](https://wiki.documentfoundation.org/Documentation/Calc_Functions/PEARSON)
///
/// __Syntax__: 
/// ```ods
///     PEARSON( IndependentValues: Array; DependentValues: Array )
/// ```
///
/// __Constraints__:
/// COLUMNS(IndependentValues) = COLUMNS(DependentValues), 
/// ROWS(IndependentValues) = ROWS(DependentValues), both sequences shall 
/// contain at least one number at corresponding positions each.
///
/// __Semantics__:
/// 
/// •IndependentValues: represents the array of the first data set. 
/// (X-Values)
/// 
/// •DependentValues: represents the array of the second data set. (Y-Values)
/// 
/// For an empty element or an element of type Text or Boolean in 
/// IndependentValues the element at the corresponding position of 
/// DependentValues is ignored, and vice versa.
///
/// __See also__: [crate::of::columns()], [crate::of::rows()], 
#[inline]
pub fn pearson<A: Array, B: Array>(independent_values: A, dependent_values: B) -> FnNumber2<A, B> {
    FnNumber2("PEARSON", independent_values, dependent_values)
}

/// Calculates the x-th sample percentile among the values in range.
///
/// [documentfoundation->PERCENTILE](https://wiki.documentfoundation.org/Documentation/Calc_Functions/PERCENTILE)
///
/// __Syntax__: 
/// ```ods
///     PERCENTILE( Data: NumberSequenceList; X: Number )
/// ```
///
/// __Constraints__:
/// 
/// •COUNT(Data) > 0
/// 
/// •0 ≤ X ≤ 1
/// 
/// •Semantics:
/// 
/// •Data: The array or range of values to get the percentile from.
/// •X: The percentile value between 0 and 1, inclusive. If X is not a 
/// multiple of
/// 
/// , PERCENTILE interpolates to obtain the value between two data points.
/// , PERCENTILE interpolates to obtain the value between two data points.
/// 
/// Returns the X-th sample percentile of data values in Data. A percentile 
/// returns the scale value for a data series which goes from the smallest 
/// (Alpha = 0) to the largest value (Alpha = 1) of a data series. For Alpha = 
/// 25%, the percentile means the first quartile; Alpha = 50% is the MEDIAN.
///
/// __See also__: [crate::of::count()], [crate::of::max()], [crate::of::max()], [crate::of::median()], [crate::of::min()], [crate::of::percentrank()], [crate::of::quartile()], [crate::of::rank()], 
#[inline]
pub fn percentile<A: Sequence, B: Number>(data: A, x: B) -> FnNumber2<A, B> {
    FnNumber2("PERCENTILE", data, x)
}

/// Returns the percentage rank of a value in a sample.
///
/// [documentfoundation->PERCENTRANK](https://wiki.documentfoundation.org/Documentation/Calc_Functions/PERCENTRANK)
///
/// __Syntax__: 
/// ```ods
///     PERCENTRANK( Data: NumberSequenceList; X: Number )
/// ```
///
/// __Constraints__:
/// 
/// •COUNT(Data) > 0
/// 
/// •MIN(Data) ≤ X ≤ MAX(Data)
/// 
/// •INT(Significance) = Significance; Significance ≥ 1
///
/// __Semantics__:
/// 
/// •Data: the array or range of data with numeric values.
/// 
/// •X: the value whose rank is to be determined.
/// 
/// •Significance: an optional value that identifies the number of 
/// significant digits for the returned percentage value. If omitted, a value 
/// of 3 is used (0.xxx).
/// 
/// Returns the rank of a value in a data set Data as a percentage of the data 
/// set, a value between 0 and 1, inclusive. This function can be used to 
/// evaluate the relative standing of a value within a data set.
/// 
/// For COUNT(Data) > 1, PERCENTRANK returns r / (COUNT(Data) -1), where r is 
/// the rank of X in Data. The rank of the lowest number in Data is 0, and of 
/// the next lowest number 1, and so on. If X is not in Data, it is assigned a 
/// fractional rank proportionately between the rank of the numbers on either 
/// side. Specifically, if X lies between Y and Z = Y + 1 (Y < X < Z) with Y 
/// being the largest number smaller than X and Z the smallest number larger 
/// than X, and where Y has rank ry, the rank of X is calculated as
/// 
/// In the special case where COUNT(Data) = 1, the only valid value for X is 
/// the single value in Data, in which case PERCENTRANK returns 1.
///
/// __See also__: [crate::of::count()], [crate::of::int()], [crate::of::max()], [crate::of::min()], [crate::of::percentile()], [crate::of::rank()], [crate::of::percentrank_()], 
#[inline]
pub fn percentrank<A: Sequence, B: Number>(data: A, x: B) -> FnNumber2<A, B> {
    FnNumber2("PERCENTRANK", data, x)
}

/// Returns the percentage rank of a value in a sample.
///
/// [documentfoundation->PERCENTRANK](https://wiki.documentfoundation.org/Documentation/Calc_Functions/PERCENTRANK)
///
/// __Syntax__: 
/// ```ods
///     PERCENTRANK( Data: NumberSequenceList; X: Number; Significance: Integer )
/// ```
///
/// __Constraints__:
/// 
/// •COUNT(Data) > 0
/// 
/// •MIN(Data) ≤ X ≤ MAX(Data)
/// 
/// •INT(Significance) = Significance; Significance ≥ 1
///
/// __Semantics__:
/// 
/// •Data: the array or range of data with numeric values.
/// 
/// •X: the value whose rank is to be determined.
/// 
/// •Significance: an optional value that identifies the number of 
/// significant digits for the returned percentage value. If omitted, a value 
/// of 3 is used (0.xxx).
/// 
/// Returns the rank of a value in a data set Data as a percentage of the data 
/// set, a value between 0 and 1, inclusive. This function can be used to 
/// evaluate the relative standing of a value within a data set.
/// 
/// For COUNT(Data) > 1, PERCENTRANK returns r / (COUNT(Data) -1), where r is 
/// the rank of X in Data. The rank of the lowest number in Data is 0, and of 
/// the next lowest number 1, and so on. If X is not in Data, it is assigned a 
/// fractional rank proportionately between the rank of the numbers on either 
/// side. Specifically, if X lies between Y and Z = Y + 1 (Y < X < Z) with Y 
/// being the largest number smaller than X and Z the smallest number larger 
/// than X, and where Y has rank ry, the rank of X is calculated as
/// 
/// In the special case where COUNT(Data) = 1, the only valid value for X is 
/// the single value in Data, in which case PERCENTRANK returns 1.
///
/// __See also__: [crate::of::count()], [crate::of::int()], [crate::of::max()], [crate::of::min()], [crate::of::percentile()], [crate::of::rank()], [crate::of::percentrank()], 
#[inline]
pub fn percentrank_<A: Sequence, B: Number, C: Number>(data: A, x: B, significance: C) -> FnNumber3<A, B, C> {
    FnNumber3("PERCENTRANK", data, x, significance)
}

/// returns the number of permutations of k objects taken from n objects.
///
/// [documentfoundation->PERMUT](https://wiki.documentfoundation.org/Documentation/Calc_Functions/PERMUT)
///
/// __Syntax__: 
/// ```ods
///     PERMUT( N: Integer; K: Integer )
/// ```
///
/// __Constraints__:
/// N ≥ 0; K ≥ 0; N ≥ K
///
/// __Semantics__:
/// PERMUT returns
///
/// __See also__: 
#[inline]
pub fn permut<A: Number, B: Number>(n: A, k: B) -> FnNumber2<A, B> {
    FnNumber2("PERMUT", n, k)
}

/// Returns the number of permutations for a given number of objects 
/// (repetition allowed).
///
/// [documentfoundation->PERMUTATIONA](https://wiki.documentfoundation.org/Documentation/Calc_Functions/PERMUTATIONA)
///
/// __Syntax__: 
/// ```ods
///     PERMUTATIONA( Total: Integer; Chosen: Integer )
/// ```
///
/// __Constraints__:
/// Total ≥ 0, Chosen ≥ 0
///
/// __Semantics__:
/// Given Total number of objects, return the number of permutations containing 
/// Chosen number of objects, with repetition permitted. The result is 1 if 
/// Total = 0 and Chosen = 0, otherwise the result is
///
/// __See also__: 
#[inline]
pub fn permutationa<A: Number, B: Number>(total: A, chosen: B) -> FnNumber2<A, B> {
    FnNumber2("PERMUTATIONA", total, chosen)
}

/// Returns the values of the density function for a standard normal 
/// distribution.
///
/// [documentfoundation->PHI](https://wiki.documentfoundation.org/Documentation/Calc_Functions/PHI)
///
/// __Syntax__: 
/// ```ods
///     PHI( N: Number )
/// ```
///
/// __Semantics__:
/// PHI(N) is a synonym for NORMDIST(N,0,1,FALSE()).
///
/// __See also__: [crate::of::normdist()], 
#[inline]
pub fn phi<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("PHI", n)
}

/// returns the probability or the cumulative distribution function for the 
/// Poisson distribution
///
/// [documentfoundation->POISSON](https://wiki.documentfoundation.org/Documentation/Calc_Functions/POISSON)
///
/// __Syntax__: 
/// ```ods
///     POISSON( X: Integer; lambda: Number )
/// ```
///
/// __Constraints__:
/// lambda > 0, X ≥ 0
///
/// __Semantics__:
/// If Cumulative is FALSE, POISSON returns the value
/// 
/// If Cumulative is TRUE, POISSON returns the value
///
/// __See also__: [crate::of::poisson_()], 
#[inline]
pub fn poisson<A: Number, B: Number>(x: A, lambda: B) -> FnNumber2<A, B> {
    FnNumber2("POISSON", x, lambda)
}

/// returns the probability or the cumulative distribution function for the 
/// Poisson distribution
///
/// [documentfoundation->POISSON](https://wiki.documentfoundation.org/Documentation/Calc_Functions/POISSON)
///
/// __Syntax__: 
/// ```ods
///     POISSON( X: Integer; lambda: Number; Cumulative: Logical )
/// ```
///
/// __Constraints__:
/// lambda > 0, X ≥ 0
///
/// __Semantics__:
/// If Cumulative is FALSE, POISSON returns the value
/// 
/// If Cumulative is TRUE, POISSON returns the value
///
/// __See also__: [crate::of::poisson()], 
#[inline]
pub fn poisson_<A: Number, B: Number, C: Logical>(x: A, lambda: B, cumulative: C) -> FnNumber3<A, B, C> {
    FnNumber3("POISSON", x, lambda, cumulative)
}

/// Returns the probability that a discrete random variable lies between two 
/// limits.
///
/// [documentfoundation->PROB](https://wiki.documentfoundation.org/Documentation/Calc_Functions/PROB)
///
/// __Syntax__: 
/// ```ods
///     PROB( Data: Array; Probability: Array; Start: Number )
/// ```
///
/// __Constraints__:
/// 
/// •The sum of the probabilities in Probability shall equal 1.
/// 
/// •All values in Probability shall be > 0 and ≤ 1.
/// 
/// •COUNT(Data) = COUNT(Probability)
///
/// __Semantics__:
/// 
/// •Data: the array or range of data in the sample ( the Number values in 
/// this array or range are referred to below as
/// 
/// ).
/// ).
/// •Probability: the array or range of the corresponding probabilities ( the 
/// Number values in this array or range are referred to below as
/// 
/// ).
/// ).
/// 
/// •Start: the start value (lower bound) of the interval whose probabilities 
/// are to be summed.
/// 
/// •End: (optional) the end value (upper bound) of the interval whose 
/// probabilities are to be summed. If omitted, End = Start is used.
/// 
/// Suppose that
/// denotes the indicator function that is 1 if
/// and 0 otherwise.
/// 
/// Then PROB returns
/// 
/// i.e. the sum of all probabilities
/// whose corresponding data value
/// satisfies
/// . Note that if
/// then PROB returns 0 since in this case
/// for all i.
///
/// __See also__: [crate::of::count()], [crate::of::prob_()], 
#[inline]
pub fn prob<A: Array, B: Array, C: Number>(data: A, probability: B, start: C) -> FnNumber3<A, B, C> {
    FnNumber3("PROB", data, probability, start)
}

/// Returns the probability that a discrete random variable lies between two 
/// limits.
///
/// [documentfoundation->PROB](https://wiki.documentfoundation.org/Documentation/Calc_Functions/PROB)
///
/// __Syntax__: 
/// ```ods
///     PROB( Data: Array; Probability: Array; Start: Number; End: Number )
/// ```
///
/// __Constraints__:
/// 
/// •The sum of the probabilities in Probability shall equal 1.
/// 
/// •All values in Probability shall be > 0 and ≤ 1.
/// 
/// •COUNT(Data) = COUNT(Probability)
///
/// __Semantics__:
/// 
/// •Data: the array or range of data in the sample ( the Number values in 
/// this array or range are referred to below as
/// 
/// ).
/// ).
/// •Probability: the array or range of the corresponding probabilities ( the 
/// Number values in this array or range are referred to below as
/// 
/// ).
/// ).
/// 
/// •Start: the start value (lower bound) of the interval whose probabilities 
/// are to be summed.
/// 
/// •End: (optional) the end value (upper bound) of the interval whose 
/// probabilities are to be summed. If omitted, End = Start is used.
/// 
/// Suppose that
/// denotes the indicator function that is 1 if
/// and 0 otherwise.
/// 
/// Then PROB returns
/// 
/// i.e. the sum of all probabilities
/// whose corresponding data value
/// satisfies
/// . Note that if
/// then PROB returns 0 since in this case
/// for all i.
///
/// __See also__: [crate::of::count()], [crate::of::prob()], 
#[inline]
pub fn prob_<A: Array, B: Array, C: Number, D: Number>(data: A, probability: B, start: C, end: D) -> FnNumber4<A, B, C, D> {
    FnNumber4("PROB", data, probability, start, end)
}

/// Returns a quartile of a set of data points.
///
/// [documentfoundation->QUARTILE](https://wiki.documentfoundation.org/Documentation/Calc_Functions/QUARTILE)
///
/// __Syntax__: 
/// ```ods
///     QUARTILE( Data: NumberSequence; Quart: Integer )
/// ```
///
/// __Constraints__:
/// 
/// •COUNT(Data) > 0
/// 
/// •0 ≤ Quart ≤ 4
///
/// __Semantics__:
/// 
/// •Data: The cell range or data array of numeric values.
/// 
/// •Quart: The number of the quartile to return.
/// 
/// If Quart = 0, the minimum value is returned, which is equivalent to the 
/// MIN() function.
/// 
/// If Quart = 1, the value of the 25th percentile is returned.
/// 
/// If Quart = 2, the value of the 50th percentile is returned, which is 
/// equivalent to the MEDIAN() function.
/// 
/// If Quart = 3, the value of the 75th percentile is returned.
/// 
/// If Quart = 4, the maximum value is returned, which is equivalent to the 
/// MAX() function.
/// 
/// Based on the statistical rank of the data points in Data, QUARTILE returns 
/// the percentile value indicated by Quart. The percentile is calculated as 
/// Quart divided by 4. An algorithm to calculate the percentile for a set of 
/// data points is given in the definition of PERCENTILE.
///
/// __See also__: [crate::of::count()], [crate::of::max()], [crate::of::median()], [crate::of::min()], [crate::of::percentile()], [crate::of::percentrank()], [crate::of::rank()], 
#[inline]
pub fn quartile<A: Sequence, B: Number>(data: A, quart: B) -> FnNumber2<A, B> {
    FnNumber2("QUARTILE", data, quart)
}

/// Returns the rank of a number in a list of numbers.
///
/// [documentfoundation->RANK](https://wiki.documentfoundation.org/Documentation/Calc_Functions/RANK)
///
/// __Syntax__: 
/// ```ods
///     RANK( Value: Number; Data: NumberSequenceList )
/// ```
///
/// __Constraints__:
/// Value shall exist in Data.
///
/// __Semantics__:
/// The RANK function returns the rank of a value within a list.
/// 
/// •Value: the number for which to determine the rank.
/// 
/// •Data: numbers used to determine the ranking.
/// 
/// •Order: specifies how to rank the numbers:
/// If 0 or omitted, Data is ranked in descending order.
/// If not 0, Data is ranked in ascending order.
/// 
/// If a number in Data occurs more than once it is given the same rank, but 
/// increments the rank for subsequent different numbers. If Value does not 
/// exist in Data an Error is returned.
///
/// __See also__: [crate::of::rank_()], 
#[inline]
pub fn rank<A: Number, B: Sequence>(value: A, data: B) -> FnNumber2<A, B> {
    FnNumber2("RANK", value, data)
}

/// Returns the rank of a number in a list of numbers.
///
/// [documentfoundation->RANK](https://wiki.documentfoundation.org/Documentation/Calc_Functions/RANK)
///
/// __Syntax__: 
/// ```ods
///     RANK( Value: Number; Data: NumberSequenceList; Order: Number )
/// ```
///
/// __Constraints__:
/// Value shall exist in Data.
///
/// __Semantics__:
/// The RANK function returns the rank of a value within a list.
/// 
/// •Value: the number for which to determine the rank.
/// 
/// •Data: numbers used to determine the ranking.
/// 
/// •Order: specifies how to rank the numbers:
/// If 0 or omitted, Data is ranked in descending order.
/// If not 0, Data is ranked in ascending order.
/// 
/// If a number in Data occurs more than once it is given the same rank, but 
/// increments the rank for subsequent different numbers. If Value does not 
/// exist in Data an Error is returned.
///
/// __See also__: [crate::of::rank()], 
#[inline]
pub fn rank_<A: Number, B: Sequence, C: Number>(value: A, data: B, order: C) -> FnNumber3<A, B, C> {
    FnNumber3("RANK", value, data, order)
}

/// Returns the square of the Pearson product moment correlation coefficient.
///
/// [documentfoundation->RSQ](https://wiki.documentfoundation.org/Documentation/Calc_Functions/RSQ)
///
/// __Syntax__: 
/// ```ods
///     RSQ( ArrayY: Array; ArrayX: Array )
/// ```
///
/// __Constraints__:
/// 
/// The arguments shall be either numbers or names, arrays, or references that 
/// contain numbers.
/// 
/// If an array or reference argument contains Text, Logical values, or empty 
/// cells, those values are ignored; however, cells with the value zero are 
/// included.
/// 
/// If ArrayY and ArrayX are empty or have a different number of data points, 
/// then #N/A is returned.
/// 
/// COLUMNS(ArrayY) = COLUMNS(ArrayX), ROWS(ArrayY) = ROWS(ArrayX)Semantics: 
/// The r-squared value can be interpreted as the proportion of the variance in 
/// y attributable to the variance in x.
/// 
/// The result of the RSQ function is the same as PEARSON * PEARSON.
/// 
/// For an empty element or an element of type Text or Boolean in ArrayY the 
/// element at the corresponding position of ArrayX is ignored, and vice versa.
///
/// __See also__: [crate::of::columns()], [crate::of::rows()], [crate::of::pearson()], 
#[inline]
pub fn rsq<A: Array, B: Array>(array_y: A, array_x: B) -> FnNumber2<A, B> {
    FnNumber2("RSQ", array_y, array_x)
}

/// Estimates the skewness of a distribution using a sample set of numbers.
///
/// [documentfoundation->SKEW](https://wiki.documentfoundation.org/Documentation/Calc_Functions/SKEW)
///
/// __Syntax__: 
/// ```ods
///     SKEW({ Sample: NumberSequenceList}+ )
/// ```
///
/// __Constraints__:
/// The sequence shall contain three numbers at least.
///
/// __Semantics__:
/// Estimates the skewness of a distribution using a sample set of numbers.
/// Given the expectation value
/// and the standard deviation estimate
/// , the skewness becomes
///
/// __See also__: [crate::of::skewp()], 
#[inline]
pub fn skew<A: Sequence>(sample: A) -> FnNumber1<A> {
    FnNumber1("SKEW", sample)
}

/// Calculates the skewness of a distribution using the population of a random 
/// variable.
///
/// [documentfoundation->SKEWP](https://wiki.documentfoundation.org/Documentation/Calc_Functions/SKEWP)
///
/// __Syntax__: 
/// ```ods
///     SKEWP({ Population: NumberSequence}+ )
/// ```
///
/// __Constraints__:
/// The sequence shall contain three numbers at least.
///
/// __Semantics__:
/// Calculates the skewness of a distribution using the population, i.e. the 
/// possible outcomes, of a random variable.
/// Given the expectation value
/// and the standard deviation sigma,the skewness becomes
///
/// __See also__: [crate::of::skew()], 
#[inline]
pub fn skewp<A: Sequence>(population: A) -> FnNumber1<A> {
    FnNumber1("SKEWP", population)
}

/// Calculates the slope of the linear regression line.
///
/// [documentfoundation->SLOPE](https://wiki.documentfoundation.org/Documentation/Calc_Functions/SLOPE)
///
/// __Syntax__: 
/// ```ods
///     SLOPE( Y: Array; X: Array )
/// ```
///
/// __Constraints__:
/// COLUMNS(Y) = COLUMNS(X), ROWS(Y) = ROWS(X), both sequences shall contain at 
/// least one number at corresponding positions each.
///
/// __Semantics__:
/// Calculates the slope of the linear regression line.
/// 
/// For an empty element or an element of type Text or Boolean in Y the element 
/// at the corresponding position of X is ignored, and vice versa.
///
/// __See also__: [crate::of::columns()], [crate::of::rows()], [crate::of::intercept()], [crate::of::steyx()], 
#[inline]
pub fn slope<A: Array, B: Array>(y: A, x: B) -> FnNumber2<A, B> {
    FnNumber2("SLOPE", y, x)
}

/// Finds the nth smallest value in a list.
///
/// [documentfoundation->SMALL](https://wiki.documentfoundation.org/Documentation/Calc_Functions/SMALL)
///
/// __Syntax__: 
/// ```ods
///     SMALL( List: NumberSequenceList; N: Integer|Array )
/// ```
///
/// __Constraints__:
/// ROUNDDOWN(N;0) = N, effectively being INT(N) = N for positive numbers. If 
/// the resulting N is <1 or larger than the size of List, Error is returned.
///
/// __Semantics__:
/// If N is an array of numbers, an array of smallest values is returned.
///
/// __See also__: [crate::of::int()], [crate::of::large()], [crate::of::rounddown()], 
#[inline]
pub fn small<A: Sequence, B: NumberOrArray>(list: A, n: B) -> FnArray2<A, B> {
    FnArray2("SMALL", list, n)
}

/// Calculates a normalized value of a random variable.
///
/// [documentfoundation->STANDARDIZE](https://wiki.documentfoundation.org/Documentation/Calc_Functions/STANDARDIZE)
///
/// __Syntax__: 
/// ```ods
///     STANDARDIZE( Value: Number; Mean: Number; Sigma: Number )
/// ```
///
/// __Constraints__:
/// Sigma > 0
///
/// __Semantics__:
/// Calculates a normalized value of a random variable.
///
/// __See also__: [crate::of::gauss()], 
#[inline]
pub fn standardize<A: Number, B: Number, C: Number>(value: A, mean: B, sigma: C) -> FnNumber3<A, B, C> {
    FnNumber3("STANDARDIZE", value, mean, sigma)
}

/// Compute the sample standard deviation of a set of numbers.
///
/// [documentfoundation->STDEV](https://wiki.documentfoundation.org/Documentation/Calc_Functions/STDEV)
///
/// __Syntax__: 
/// ```ods
///     STDEV({ N: NumberSequenceList}+ )
/// ```
///
/// __Constraints__:
/// At least two numbers shall be included. Returns an Error if less than two 
/// Numbers are provided.
///
/// __Semantics__:
/// Computes the sample standard deviation s, where
/// 
/// Note that s is not the same as the standard deviation of the set, sigma, 
/// which uses n rather than n − 1.
///
/// __See also__: [crate::of::stdevp()], [crate::of::average()], 
#[inline]
pub fn stdev<A: Sequence>(n: A) -> FnNumber1<A> {
    FnNumber1("STDEV", n)
}

/// Calculate the standard deviation using a sample set of values, including 
/// values of type Text and Logical.
///
/// [documentfoundation->STDEVA](https://wiki.documentfoundation.org/Documentation/Calc_Functions/STDEVA)
///
/// __Syntax__: 
/// ```ods
///     STDEVA({ Sample: Any}+ )
/// ```
///
/// __Constraints__:
/// COUNTA(Sample) > 1.
///
/// __Semantics__:
/// Unlike the STDEV function, includes values of type Text and Logical. Text 
/// values are treated as number 0. Logical TRUE is treated as 1, and FALSE is 
/// treated as 0. Empty cells are not included.
/// 
/// The handling of string constants as parameters is implementation-defined. 
/// Either, string constants are converted to numbers, if possible and 
/// otherwise, they are treated as 0, or string constants are always treated as 
/// 0.
/// 
/// Suppose the resulting sequence of values is x1, x2, …, xn. Then let
/// 
/// STDEVA returns
///
/// __See also__: [crate::of::counta()], [crate::of::stdev()], 
#[inline]
pub fn stdeva<A: Sequence>(sample: A) -> FnNumber1<A> {
    FnNumber1("STDEVA", sample)
}

/// Calculates the standard deviation using the population of a random 
/// variable, including values of type Text and Logical.
///
/// [documentfoundation->STDEVP](https://wiki.documentfoundation.org/Documentation/Calc_Functions/STDEVP)
///
/// __Syntax__: 
/// ```ods
///     STDEVP({ N: NumberSequence}+ )
/// ```
///
/// __Constraints__:
/// COUNT(N) ≥ 1.
///
/// __Semantics__:
/// Computes the standard deviation of the set sigma, where
/// 
/// Note that sigma is not the same as the sample standard deviation, s, which 
/// uses n − 1 rather than n.
///
/// __See also__: [crate::of::count()], [crate::of::stdev()], [crate::of::average()], 
#[inline]
pub fn stdevp<A: Sequence>(n: A) -> FnNumber1<A> {
    FnNumber1("STDEVP", n)
}

/// Calculates the standard deviation using the population of a random 
/// variable, including values of type Text and Logical.
///
/// [documentfoundation->STDEVPA](https://wiki.documentfoundation.org/Documentation/Calc_Functions/STDEVPA)
///
/// __Syntax__: 
/// ```ods
///     STDEVPA({ Sample: Any}+ )
/// ```
///
/// __Constraints__:
/// COUNTA(Sample) ≥ 1.
///
/// __Semantics__:
/// Unlike the STDEV function, includes values of type Text and Logical. Text 
/// values are treated as number 0. Logical TRUE is treated as 1, and FALSE is 
/// treated as 0. Empty cells are not included.
/// Given the expectation value
/// the standard deviation becomes
/// 
/// In the sequence, only Numbers and Logical types are considered; cells with 
/// Text are converted to 0; other types are ignored. If Logical types are a 
/// distinct type, they are still included, with TRUE considered 1 and FALSE 
/// considered 0. Any Sample may be of type ReferenceList.
/// 
/// The handling of string constants as parameters is implementation-defined. 
/// Either, string constants are converted to numbers, if possible and 
/// otherwise, they are treated as zero, or string constants are always treated 
/// as zero.
///
/// __See also__: [crate::of::counta()], [crate::of::stdevp()], 
#[inline]
pub fn stdevpa<A: Sequence>(sample: A) -> FnNumber1<A> {
    FnNumber1("STDEVPA", sample)
}

/// Calculates the standard error of the predicted y value for each x in the 
/// regression.
///
/// [documentfoundation->STEYX](https://wiki.documentfoundation.org/Documentation/Calc_Functions/STEYX)
///
/// __Syntax__: 
/// ```ods
///     STEYX( MeasuredY: Array; X: Array )
/// ```
///
/// __Constraints__:
/// COLUMNS(MeasuredY) = COLUMNS(X), ROWS(MeasuredY) = ROWS(X), both sequences 
/// shall contain at least three numbers at corresponding positions each.
///
/// __Semantics__:
/// Calculates the standard error of the predicted y value for each x in the 
/// regression.
/// 
/// For an empty element or an element of type Text or Boolean in MeasuredY the 
/// element at the corresponding position of X is ignored, and vice versa.
///
/// __See also__: [crate::of::columns()], [crate::of::rows()], [crate::of::intercept()], [crate::of::slope()], 
#[inline]
pub fn steyx<A: Array, B: Array>(measured_y: A, x: B) -> FnNumber2<A, B> {
    FnNumber2("STEYX", measured_y, x)
}

/// Returns the area to the tail or tails of the probability density function 
/// of the t-distribution.
///
/// [documentfoundation->LEGACY.TDIST](https://wiki.documentfoundation.org/Documentation/Calc_Functions/LEGACY.TDIST)
///
/// __Syntax__: 
/// ```ods
///     LEGACY.TDIST( X: Number; Df: Integer; Tails: Integer )
/// ```
///
/// __Constraints__:
/// X ≥ 0, Df ≥ 1, Tails = 1 or 2
///
/// __Semantics__:
/// Then LEGACY.TDIST returns
/// 
/// where
/// 
/// Note that Df denotes the degrees of freedom of the t-distribution and Γ is 
/// the Gamma function.
///
/// __See also__: [crate::of::gamma()], [crate::of::betadist()], [crate::of::binomdist()], [crate::of::chisqdist()], [crate::of::expondist()], [crate::of::fdist()], [crate::of::gammadist()], [crate::of::gauss()], [crate::of::hypgeomdist()], [crate::of::lognormdist()], [crate::of::negbinomdist()], [crate::of::normdist()], [crate::of::poisson()], [crate::of::weibull()], 
#[inline]
pub fn legacy_tdist<A: Number, B: Number, C: Number>(x: A, df: B, tails: C) -> FnNumber3<A, B, C> {
    FnNumber3("LEGACY.TDIST", x, df, tails)
}

/// Calculates the inverse of the two-tailed t-distribution.
///
/// [documentfoundation->TINV](https://wiki.documentfoundation.org/Documentation/Calc_Functions/TINV)
///
/// __Syntax__: 
/// ```ods
///     TINV( Probability: Number; DegreeOfFreedom: Integer )
/// ```
///
/// __Constraints__:
/// 0 < Probability ≤ 1, DegreeOfFreedom ≥ 1
///
/// __Semantics__:
/// Calculates the inverse of the two-tailed t-distribution.
///
/// __See also__: [crate::of::legacy_tdist()], 
#[inline]
pub fn tinv<A: Number, B: Number>(probability: A, degree_of_freedom: B) -> FnNumber2<A, B> {
    FnNumber2("TINV", probability, degree_of_freedom)
}

/// Returns predicted values based on a simple or multiple linear regression.
///
/// [documentfoundation->TREND](https://wiki.documentfoundation.org/Documentation/Calc_Functions/TREND)
///
/// __Syntax__: 
/// ```ods
///     TREND( KnownY: Array )
/// ```
///
/// __Constraints__:
/// (COLUMNS(KnownY) = COLUMNS(KnownX) and ROWS(KnownY) = ROWS(KnownX)) or 
/// (COLUMNS(KnownY) = 1 and ROWS(KnownY) = ROWS(KnownX) and COLUMNS(KnownX) = 
/// COLUMNS(NewX)) or (COLUMNS(KnownY) = COLUMNS(KnownX) and ROWS(KnownY) = 1 
/// and ROWS(KnownX) = ROWS(NewX))
///
/// __Semantics__:
/// 
/// KnownY: The set of known y-values to be used to determine the regression 
/// equation
/// 
/// KnownX: The set of known x-values to be used to determine the regression 
/// equation. If omitted or an empty parameter, it is set to the sequence 
/// 1,2,3,…,k, where k = ROWS(KnownY) ∙ COLUMNS(KnownY).
/// 
/// NewX: The set of x-values for which predicted y-values are to be 
/// calculated. If omitted or an empty parameter, it is set to KnownX.
/// 
/// Const: If set to FALSE, the model constant a is equal to 0.
/// LINEST(KnownY; KnownX; Const; FALSE()) either returns an error an array 
/// with 1 row and n + 1 columns. If it returns an error then so does TREND. If 
/// it returns an array, we call the entries in that array
/// .
/// Let
/// denote the entry in the ith row and jth column of NewX.
/// If COLUMNS(KnownY) = COLUMNS(KnownX) and ROWS(KnownY) = ROWS(KnownX), then 
/// TREND returns an array with ROWS(NewX) rows and COLUMNS(NewX) column, such 
/// that the entry in its ith row and jth column is
/// .
/// Otherwise, if COLUMNS(KnownY) = 1 and ROWS(KnownY) = ROWS(KnownX) and 
/// COLUMNS(KnownX) = COLUMNS(NewX), then TREND returns an array with 
/// ROWS(NewX) rows and 1 column, such that the entry in the ith row is
/// .
/// Otherwise, if COLUMNS(KnownY) = COLUMNS(KnownX) and ROWS(KnownY) = 1 and 
/// ROWS(KnownX) = ROWS(NewX), then TREND returns an array with 1 row and 
/// COLUMNS(NewX) columns, such that the entry in the jth column is
/// .
///
/// __See also__: [crate::of::columns()], [crate::of::rows()], [crate::of::intercept()], [crate::of::linest()], [crate::of::slope()], [crate::of::steyx()], [crate::of::trend_()], [crate::of::trend__()], [crate::of::trend___()], 
#[inline]
pub fn trend<A: Array>(known_y: A) -> FnArray1<A> {
    FnArray1("TREND", known_y)
}

/// Returns predicted values based on a simple or multiple linear regression.
///
/// [documentfoundation->TREND](https://wiki.documentfoundation.org/Documentation/Calc_Functions/TREND)
///
/// __Syntax__: 
/// ```ods
///     TREND( KnownY: Array; KnownX: Array )
/// ```
///
/// __Constraints__:
/// (COLUMNS(KnownY) = COLUMNS(KnownX) and ROWS(KnownY) = ROWS(KnownX)) or 
/// (COLUMNS(KnownY) = 1 and ROWS(KnownY) = ROWS(KnownX) and COLUMNS(KnownX) = 
/// COLUMNS(NewX)) or (COLUMNS(KnownY) = COLUMNS(KnownX) and ROWS(KnownY) = 1 
/// and ROWS(KnownX) = ROWS(NewX))
///
/// __Semantics__:
/// 
/// KnownY: The set of known y-values to be used to determine the regression 
/// equation
/// 
/// KnownX: The set of known x-values to be used to determine the regression 
/// equation. If omitted or an empty parameter, it is set to the sequence 
/// 1,2,3,…,k, where k = ROWS(KnownY) ∙ COLUMNS(KnownY).
/// 
/// NewX: The set of x-values for which predicted y-values are to be 
/// calculated. If omitted or an empty parameter, it is set to KnownX.
/// 
/// Const: If set to FALSE, the model constant a is equal to 0.
/// LINEST(KnownY; KnownX; Const; FALSE()) either returns an error an array 
/// with 1 row and n + 1 columns. If it returns an error then so does TREND. If 
/// it returns an array, we call the entries in that array
/// .
/// Let
/// denote the entry in the ith row and jth column of NewX.
/// If COLUMNS(KnownY) = COLUMNS(KnownX) and ROWS(KnownY) = ROWS(KnownX), then 
/// TREND returns an array with ROWS(NewX) rows and COLUMNS(NewX) column, such 
/// that the entry in its ith row and jth column is
/// .
/// Otherwise, if COLUMNS(KnownY) = 1 and ROWS(KnownY) = ROWS(KnownX) and 
/// COLUMNS(KnownX) = COLUMNS(NewX), then TREND returns an array with 
/// ROWS(NewX) rows and 1 column, such that the entry in the ith row is
/// .
/// Otherwise, if COLUMNS(KnownY) = COLUMNS(KnownX) and ROWS(KnownY) = 1 and 
/// ROWS(KnownX) = ROWS(NewX), then TREND returns an array with 1 row and 
/// COLUMNS(NewX) columns, such that the entry in the jth column is
/// .
///
/// __See also__: [crate::of::columns()], [crate::of::rows()], [crate::of::intercept()], [crate::of::linest()], [crate::of::slope()], [crate::of::steyx()], [crate::of::trend()], [crate::of::trend__()], [crate::of::trend___()], 
#[inline]
pub fn trend_<A: Array, B: Array>(known_y: A, known_x: B) -> FnArray2<A, B> {
    FnArray2("TREND", known_y, known_x)
}

/// Returns predicted values based on a simple or multiple linear regression.
///
/// [documentfoundation->TREND](https://wiki.documentfoundation.org/Documentation/Calc_Functions/TREND)
///
/// __Syntax__: 
/// ```ods
///     TREND( KnownY: Array; KnownX: Array; NewX: Array )
/// ```
///
/// __Constraints__:
/// (COLUMNS(KnownY) = COLUMNS(KnownX) and ROWS(KnownY) = ROWS(KnownX)) or 
/// (COLUMNS(KnownY) = 1 and ROWS(KnownY) = ROWS(KnownX) and COLUMNS(KnownX) = 
/// COLUMNS(NewX)) or (COLUMNS(KnownY) = COLUMNS(KnownX) and ROWS(KnownY) = 1 
/// and ROWS(KnownX) = ROWS(NewX))
///
/// __Semantics__:
/// 
/// KnownY: The set of known y-values to be used to determine the regression 
/// equation
/// 
/// KnownX: The set of known x-values to be used to determine the regression 
/// equation. If omitted or an empty parameter, it is set to the sequence 
/// 1,2,3,…,k, where k = ROWS(KnownY) ∙ COLUMNS(KnownY).
/// 
/// NewX: The set of x-values for which predicted y-values are to be 
/// calculated. If omitted or an empty parameter, it is set to KnownX.
/// 
/// Const: If set to FALSE, the model constant a is equal to 0.
/// LINEST(KnownY; KnownX; Const; FALSE()) either returns an error an array 
/// with 1 row and n + 1 columns. If it returns an error then so does TREND. If 
/// it returns an array, we call the entries in that array
/// .
/// Let
/// denote the entry in the ith row and jth column of NewX.
/// If COLUMNS(KnownY) = COLUMNS(KnownX) and ROWS(KnownY) = ROWS(KnownX), then 
/// TREND returns an array with ROWS(NewX) rows and COLUMNS(NewX) column, such 
/// that the entry in its ith row and jth column is
/// .
/// Otherwise, if COLUMNS(KnownY) = 1 and ROWS(KnownY) = ROWS(KnownX) and 
/// COLUMNS(KnownX) = COLUMNS(NewX), then TREND returns an array with 
/// ROWS(NewX) rows and 1 column, such that the entry in the ith row is
/// .
/// Otherwise, if COLUMNS(KnownY) = COLUMNS(KnownX) and ROWS(KnownY) = 1 and 
/// ROWS(KnownX) = ROWS(NewX), then TREND returns an array with 1 row and 
/// COLUMNS(NewX) columns, such that the entry in the jth column is
/// .
///
/// __See also__: [crate::of::columns()], [crate::of::rows()], [crate::of::intercept()], [crate::of::linest()], [crate::of::slope()], [crate::of::steyx()], [crate::of::trend()], [crate::of::trend_()], [crate::of::trend___()], 
#[inline]
pub fn trend__<A: Array, B: Array, C: Array>(known_y: A, known_x: B, new_x: C) -> FnArray3<A, B, C> {
    FnArray3("TREND", known_y, known_x, new_x)
}

/// Returns predicted values based on a simple or multiple linear regression.
///
/// [documentfoundation->TREND](https://wiki.documentfoundation.org/Documentation/Calc_Functions/TREND)
///
/// __Syntax__: 
/// ```ods
///     TREND( KnownY: Array; KnownX: Array; NewX: Array; Const: Logical )
/// ```
///
/// __Constraints__:
/// (COLUMNS(KnownY) = COLUMNS(KnownX) and ROWS(KnownY) = ROWS(KnownX)) or 
/// (COLUMNS(KnownY) = 1 and ROWS(KnownY) = ROWS(KnownX) and COLUMNS(KnownX) = 
/// COLUMNS(NewX)) or (COLUMNS(KnownY) = COLUMNS(KnownX) and ROWS(KnownY) = 1 
/// and ROWS(KnownX) = ROWS(NewX))
///
/// __Semantics__:
/// 
/// KnownY: The set of known y-values to be used to determine the regression 
/// equation
/// 
/// KnownX: The set of known x-values to be used to determine the regression 
/// equation. If omitted or an empty parameter, it is set to the sequence 
/// 1,2,3,…,k, where k = ROWS(KnownY) ∙ COLUMNS(KnownY).
/// 
/// NewX: The set of x-values for which predicted y-values are to be 
/// calculated. If omitted or an empty parameter, it is set to KnownX.
/// 
/// Const: If set to FALSE, the model constant a is equal to 0.
/// LINEST(KnownY; KnownX; Const; FALSE()) either returns an error an array 
/// with 1 row and n + 1 columns. If it returns an error then so does TREND. If 
/// it returns an array, we call the entries in that array
/// .
/// Let
/// denote the entry in the ith row and jth column of NewX.
/// If COLUMNS(KnownY) = COLUMNS(KnownX) and ROWS(KnownY) = ROWS(KnownX), then 
/// TREND returns an array with ROWS(NewX) rows and COLUMNS(NewX) column, such 
/// that the entry in its ith row and jth column is
/// .
/// Otherwise, if COLUMNS(KnownY) = 1 and ROWS(KnownY) = ROWS(KnownX) and 
/// COLUMNS(KnownX) = COLUMNS(NewX), then TREND returns an array with 
/// ROWS(NewX) rows and 1 column, such that the entry in the ith row is
/// .
/// Otherwise, if COLUMNS(KnownY) = COLUMNS(KnownX) and ROWS(KnownY) = 1 and 
/// ROWS(KnownX) = ROWS(NewX), then TREND returns an array with 1 row and 
/// COLUMNS(NewX) columns, such that the entry in the jth column is
/// .
///
/// __See also__: [crate::of::columns()], [crate::of::rows()], [crate::of::intercept()], [crate::of::linest()], [crate::of::slope()], [crate::of::steyx()], [crate::of::trend()], [crate::of::trend_()], [crate::of::trend__()], 
#[inline]
pub fn trend___<A: Array, B: Array, C: Array, D: Logical>(known_y: A, known_x: B, new_x: C, const_: D) -> FnArray4<A, B, C, D> {
    FnArray4("TREND", known_y, known_x, new_x, const_)
}

/// Returns the mean of a data set, ignoring a proportion of high and low 
/// values.
///
/// [documentfoundation->TRIMMEAN](https://wiki.documentfoundation.org/Documentation/Calc_Functions/TRIMMEAN)
///
/// __Syntax__: 
/// ```ods
///     TRIMMEAN( DataSet: NumberSequenceList; CutOffFraction: Number )
/// ```
///
/// __Constraints__:
/// 0 ≤ CutOffFraction < 1
///
/// __Semantics__:
/// Returns the mean of a data set, ignoring a proportion of high and low 
/// values.
/// 
/// Let n denote the number of elements in the data set and let
/// 
/// be the values in the data set sorted in ascending order. Moreover let
/// 
/// Then TRIMMEAN returns the value
///
/// __See also__: [crate::of::average()], [crate::of::geomean()], [crate::of::harmean()], 
#[inline]
pub fn trimmean<A: Sequence, B: Number>(data_set: A, cut_off_fraction: B) -> FnNumber2<A, B> {
    FnNumber2("TRIMMEAN", data_set, cut_off_fraction)
}

/// Calculates the p-value of a 2-sample t-test.
///
/// [documentfoundation->TTEST](https://wiki.documentfoundation.org/Documentation/Calc_Functions/TTEST)
///
/// __Syntax__: 
/// ```ods
///     TTEST( X: Array; Y: Array; Tails: Integer; Type: Integer )
/// ```
///
/// __Constraints__:
/// COUNT(X) > 1, COUNT(Y) > 1, Tails = 1 or 2, Type = 1,2, or 3,
/// (COUNT(X) = COUNT(Y) or Type ≠ 1)
/// 
/// COLUMNS(X) = COLUMNS(Y), ROWS(X) = ROWS(Y)
///
/// __Semantics__:
/// Let X1, X2, …,Xn be the numbers in the sequence X and Y1, Y2, …,Ym be 
/// the numbers in the sequence Y. Then
/// 
/// and
/// 
/// Moreover let
/// 
/// and
/// 
/// where Γ is the Gamma function.
/// 
/// (1)If type = 1, TTEST calculates the p-value for a paired-sample comparison 
/// of means test. Note that in this case due to the above constraints n = m. 
/// With
/// 
/// and
/// 
/// TTEST returns
/// 
/// (2)If Type = 2, TTEST calculates the p-value of a comparison of means for 
/// independent samples from populations with equal variance. With
/// 
/// (1)
/// 
/// and
/// 
/// TTEST returns
/// 
/// (3)If Type = 3, TTEST calculates the p-value of a comparison of means for 
/// independent samples from populations with not necessarily equal variances. 
/// With
/// 
/// (2)
/// 
/// and
/// 
/// TTEST returns
/// 
/// For an empty element or an element of type Text or Boolean in X the element 
/// at the corresponding position of Y is ignored, and vice versa.
///
/// __See also__: [crate::of::columns()], [crate::of::count()], [crate::of::rows()], [crate::of::ftest()], [crate::of::legacy_tdist()], [crate::of::ztest()], 
#[inline]
pub fn ttest<A: Array, B: Array, C: Number, D: Number>(x: A, y: B, tails: C, type_: D) -> FnNumber4<A, B, C, D> {
    FnNumber4("TTEST", x, y, tails, type_)
}

/// Compute the sample variance of a set of numbers.
///
/// [documentfoundation->VAR](https://wiki.documentfoundation.org/Documentation/Calc_Functions/VAR)
///
/// __Syntax__: 
/// ```ods
///     VAR({ N: NumberSequence}+ )
/// ```
///
/// __Constraints__:
/// At least two numbers shall be included. Returns an Error if less than two 
/// Numbers are provided.
///
/// __Semantics__:
/// Computes the sample variance s2, where
/// 
/// Note that s2 is not the same as the variance of the set, sigma2, which uses 
/// n rather than n − 1.
///
/// __See also__: [crate::of::varp()], [crate::of::stdev()], [crate::of::average()], 
#[inline]
pub fn var<A: Sequence>(n: A) -> FnNumber1<A> {
    FnNumber1("VAR", n)
}

/// Estimates the variance using a sample set of values, including values of 
/// type Text and Logical.
///
/// [documentfoundation->VARA](https://wiki.documentfoundation.org/Documentation/Calc_Functions/VARA)
///
/// __Syntax__: 
/// ```ods
///     VARA({ Sample: Any}+ )
/// ```
///
/// __Constraints__:
/// The sequence shall contain two numbers at least.
///
/// __Semantics__:
/// Unlike the VAR function, includes values of type Text and Logical. Text 
/// values are treated as number 0. Logical TRUE is treated as 1, and FALSE is 
/// treated as 0. Empty cells are not included.
/// Given the expectation value
/// the estimated variance becomes
/// 
/// In the sequence, only Numbers and Logical types are considered; cells with 
/// Text are converted to 0; other types are ignored. If Logical types are a 
/// distinct type, they are still included, with TRUE considered 1 and FALSE 
/// considered 0. Any Sample may be of type ReferenceList.
/// 
/// The handling of string constants as parameters is implementation-defined. 
/// Either, string constants are converted to numbers, if possible and 
/// otherwise, they are treated as zero, or string constants are always treated 
/// as zero.
///
/// __See also__: [crate::of::var()], 
#[inline]
pub fn vara<A: Sequence>(sample: A) -> FnNumber1<A> {
    FnNumber1("VARA", sample)
}

/// Compute the variance of the set for a set of numbers.
///
/// [documentfoundation->VARP](https://wiki.documentfoundation.org/Documentation/Calc_Functions/VARP)
///
/// __Syntax__: 
/// ```ods
///     VARP({ N: NumberSequence}+ )
/// ```
///
/// __Constraints__:
/// COUNT(N) ≥ 1
///
/// __Semantics__:
/// Computes the variance of the set sigma2, where
/// 
/// Note that sigma2 is not the same as the sample variance, s2, which uses n 
/// − 1 rather than n.
/// 
/// If only one number is provided, returns 0.
///
/// __See also__: [crate::of::count()], [crate::of::var()], [crate::of::stdevp()], [crate::of::average()], 
#[inline]
pub fn varp<A: Sequence>(n: A) -> FnNumber1<A> {
    FnNumber1("VARP", n)
}

/// Calculates the variance using the population of the distribution, including 
/// values of type Text and Logical.
///
/// [documentfoundation->VARPA](https://wiki.documentfoundation.org/Documentation/Calc_Functions/VARPA)
///
/// __Syntax__: 
/// ```ods
///     VARPA({ Sample: Any}+ )
/// ```
///
/// __Constraints__:
/// COUNTA(Sample) ≥ 1.
///
/// __Semantics__:
/// Unlike the VARP function, includes values of type Text and Logical. Text 
/// values are treated as number 0. Logical TRUE is treated as 1, and FALSE is 
/// treated as 0. Empty cells are not included.
/// Given the expectation value
/// the variance becomes
/// 
/// In the sequence, only Numbers and Logical types are considered; cells with 
/// Text are converted to 0; other types are ignored. If Logical types are a 
/// distinct type, they are still included, with TRUE considered 1 and FALSE 
/// considered 0. Any Sample may be of type ReferenceList.
/// 
/// The handling of string constants as parameters is implementation-defined. 
/// Either, string constants are converted to numbers, if possible and 
/// otherwise, they are treated as zero, or string constants are always treated 
/// as zero.
///
/// __See also__: [crate::of::counta()], [crate::of::varp()], 
#[inline]
pub fn varpa<A: Sequence>(sample: A) -> FnNumber1<A> {
    FnNumber1("VARPA", sample)
}

/// Calculates the Weibull distribution.
///
/// [documentfoundation->WEIBULL](https://wiki.documentfoundation.org/Documentation/Calc_Functions/WEIBULL)
///
/// __Syntax__: 
/// ```ods
///     WEIBULL( Value: Number; Shape: Number; Scale: Number; Cumulative: Logical )
/// ```
///
/// __Constraints__:
/// Value ≥ 0; Shape > 0; Scale > 0
///
/// __Semantics__:
/// Calculates the Weibull distribution at the position Value.
/// 
/// If Cumulative is FALSE, the probability density function is calculated:
/// 
/// If Cumulative is TRUE, the cumulative distribution function is calculated:
///
/// __See also__: [crate::of::betadist()], [crate::of::binomdist()], [crate::of::chisqdist()], [crate::of::expondist()], [crate::of::fdist()], [crate::of::gammadist()], [crate::of::gauss()], [crate::of::hypgeomdist()], [crate::of::lognormdist()], [crate::of::negbinomdist()], [crate::of::normdist()], [crate::of::poisson()], [crate::of::legacy_tdist()], 
#[inline]
pub fn weibull<A: Number, B: Number, C: Number, D: Logical>(value: A, shape: B, scale: C, cumulative: D) -> FnNumber4<A, B, C, D> {
    FnNumber4("WEIBULL", value, shape, scale, cumulative)
}

/// Calculates the probability of observing a sample mean as large or larger 
/// than the mean of the given sample for samples drawn from a normal 
/// distribution.
///
/// [documentfoundation->ZTEST](https://wiki.documentfoundation.org/Documentation/Calc_Functions/ZTEST)
///
/// __Syntax__: 
/// ```ods
///     ZTEST( Sample: NumberSequenceList; Mean: Number )
/// ```
///
/// __Constraints__:
/// The sequence Sample shall contain at least two numbers.
///
/// __Semantics__:
/// Calculates the probability of observing a sample mean as large or larger 
/// than the mean of the given Sample for samples drawn from a normal 
/// distribution with the given mean Mean and the given standard deviation 
/// Sigma. If Sigma is omitted, it is estimated from Sample, using STDEV.
///
/// __See also__: [crate::of::ftest()], [crate::of::ttest()], [crate::of::ztest_()], 
#[inline]
pub fn ztest<A: Sequence, B: Number>(sample: A, mean: B) -> FnNumber2<A, B> {
    FnNumber2("ZTEST", sample, mean)
}

/// Calculates the probability of observing a sample mean as large or larger 
/// than the mean of the given sample for samples drawn from a normal 
/// distribution.
///
/// [documentfoundation->ZTEST](https://wiki.documentfoundation.org/Documentation/Calc_Functions/ZTEST)
///
/// __Syntax__: 
/// ```ods
///     ZTEST( Sample: NumberSequenceList; Mean: Number; Sigma: Number )
/// ```
///
/// __Constraints__:
/// The sequence Sample shall contain at least two numbers.
///
/// __Semantics__:
/// Calculates the probability of observing a sample mean as large or larger 
/// than the mean of the given Sample for samples drawn from a normal 
/// distribution with the given mean Mean and the given standard deviation 
/// Sigma. If Sigma is omitted, it is estimated from Sample, using STDEV.
///
/// __See also__: [crate::of::ftest()], [crate::of::ttest()], [crate::of::ztest()], 
#[inline]
pub fn ztest_<A: Sequence, B: Number, C: Number>(sample: A, mean: B, sigma: C) -> FnNumber3<A, B, C> {
    FnNumber3("ZTEST", sample, mean, sigma)
}

/// Uses the probability mass function of the binomial distribution to 
/// calculate the probability of a specific number of successful trial 
/// outcomes, or a range of successful trial outcomes. The binomial 
/// distribution is a discrete probability distribution that is used to analyze 
/// data in many domains.
///
/// [documentfoundation->B](https://wiki.documentfoundation.org/Documentation/Calc_Functions/B)
///
/// __Syntax__: 
/// ```ods
///     B( Trials: Integer; SP: Number; T_1: Integer )
/// ```
///
/// __Info2__:
/// Returns a real number in the range [0, 1], which is the probability for the 
/// given arguments.
///
/// __Semantics__:
/// 
/// Trials is a non-negative integer, or a reference to a cell containing that 
/// integer, that is the total number of independent trials.
/// 
/// SP is a real number (expressed as a percentage, such as 2.5%, or a decimal 
/// fraction, such as 0.025), or a reference to a cell containing that number, 
/// that is the probability of a successful outcome on each trial. As a 
/// probability, SP lies in the range [0, 1] (or equivalently 0% ≤ SP ≤ 
/// 100%).
/// 
/// T 1 is a non-negative integer, or a reference to a cell containing that 
/// integer, that specifies the lower limit for the number of successful 
/// trials.
/// 
/// T 2 is a non-negative integer, or a reference to a cell containing that 
/// integer, that specifies the upper limit for the number of successful 
/// trials. If T 2 is omitted, the function calculates the probability that the 
/// number of successful trials shall be exactly T 1. If T 2 is provided, the 
/// function calculates the probability that the number of successful trials 
/// shall lie between T 1 and T 2 inclusive.
/// 
/// If any of Trials, SP, T 1, and T 2 is non-numeric, then B reports a #VALUE! 
/// error.
/// If any of Trials, T 1, and T 2 is a non-integer value, then B truncates it 
/// to an integer value.
/// If SP is less than 0.0 or greater than 1.0, then B reports an invalid 
/// argument error (Err:502).
/// For the case when T 2 is omitted, B checks (after any truncation) that 
/// Trials ≥ 0, T 1 ≥ 0, and Trials ≥ T 1. If any of these checks fail, 
/// then B reports an invalid argument error (Err:502).
/// For the case when T 2 is provided, B checks (after any truncation) that T 1 
/// ≥ 0, T 2 ≥ T 1, and Trials ≥ T 2. If any of these checks fail, then B 
/// reports an invalid argument error (Err:502).
/// 
/// Info:
/// 
/// The formula for B is:
/// 
/// <math xmlns="http://www.w3.org/1998/Math/MathML"><mstyle 
/// displaystyle="true" scriptlevel="0"><mrow><mtext>B</mtext><mo 
/// stretchy="false">(</mo><mi>n</mi><mo>;</mo><mstyle scriptlevel="0"><mspace 
/// width="mediummathspace"></mspace></mstyle><mi>p</mi><mo>;</mo><mstyle 
/// scriptlevel="0"><mspace 
/// width="mediummathspace"></mspace></mstyle><mi>k</mi><mn>1</mn><mo>;</mo><mstyle 
/// scriptlevel="0"><mspace 
/// width="mediummathspace"></mspace></mstyle><mi>k</mi><mn>2</mn><mo 
/// stretchy="false">)</mo><mtext>&#xA0;</mtext><mo>=</mo><mtext>&#xA0;</mtext><munderover><mo 
/// data-mjx-texclass="OP">&#x2211;</mo><mrow><mi>i</mi><mo>=</mo><mi>k</mi><mn>1</mn></mrow><mrow><mi>k</mi><mn>2</mn></mrow></munderover><mfrac><mrow><mi>n</mi><mo>!</mo></mrow><mrow><mi>i</mi><mo>!</mo><mo 
/// stretchy="false">(</mo><mi>n</mi><mo>&#x2212;</mo><mi>i</mi><mo 
/// stretchy="false">)</mo><mo>!</mo></mrow></mfrac><mstyle 
/// scriptlevel="0"><mspace 
/// width="mediummathspace"></mspace></mstyle><mo>&#xD7;</mo><mstyle 
/// scriptlevel="0"><mspace 
/// width="mediummathspace"></mspace></mstyle><msup><mi>p</mi><mrow><mi>i</mi></mrow></msup><mstyle 
/// scriptlevel="0"><mspace 
/// width="mediummathspace"></mspace></mstyle><mo>&#xD7;</mo><mstyle 
/// scriptlevel="0"><mspace width="mediummathspace"></mspace></mstyle><mo 
/// stretchy="false">(</mo><mn>1</mn><mo>&#x2212;</mo><mi>p</mi><msup><mo 
/// stretchy="false">)</mo><mrow><mi>n</mi><mo>&#x2212;</mo><mi>i</mi></mrow></msup></mrow></mstyle></math>
/// 
/// If the final argument (T 2) is omitted, then set k2 to the value of k1 in 
/// this equation, effectively removing the summation operation.
///
/// __See also__: [crate::of::b_()], 
#[inline]
pub fn b<A: Number, B: Number, C: Number>(trials: A, s_p: B, t_1: C) -> FnNumber3<A, B, C> {
    FnNumber3("B", trials, s_p, t_1)
}

/// Uses the probability mass function of the binomial distribution to 
/// calculate the probability of a specific number of successful trial 
/// outcomes, or a range of successful trial outcomes. The binomial 
/// distribution is a discrete probability distribution that is used to analyze 
/// data in many domains.
///
/// [documentfoundation->B](https://wiki.documentfoundation.org/Documentation/Calc_Functions/B)
///
/// __Syntax__: 
/// ```ods
///     B( Trials: Integer; SP: Number; T_1: Integer; T_2: Integer )
/// ```
///
/// __Info2__:
/// Returns a real number in the range [0, 1], which is the probability for the 
/// given arguments.
///
/// __Semantics__:
/// 
/// Trials is a non-negative integer, or a reference to a cell containing that 
/// integer, that is the total number of independent trials.
/// 
/// SP is a real number (expressed as a percentage, such as 2.5%, or a decimal 
/// fraction, such as 0.025), or a reference to a cell containing that number, 
/// that is the probability of a successful outcome on each trial. As a 
/// probability, SP lies in the range [0, 1] (or equivalently 0% ≤ SP ≤ 
/// 100%).
/// 
/// T 1 is a non-negative integer, or a reference to a cell containing that 
/// integer, that specifies the lower limit for the number of successful 
/// trials.
/// 
/// T 2 is a non-negative integer, or a reference to a cell containing that 
/// integer, that specifies the upper limit for the number of successful 
/// trials. If T 2 is omitted, the function calculates the probability that the 
/// number of successful trials shall be exactly T 1. If T 2 is provided, the 
/// function calculates the probability that the number of successful trials 
/// shall lie between T 1 and T 2 inclusive.
/// 
/// If any of Trials, SP, T 1, and T 2 is non-numeric, then B reports a #VALUE! 
/// error.
/// If any of Trials, T 1, and T 2 is a non-integer value, then B truncates it 
/// to an integer value.
/// If SP is less than 0.0 or greater than 1.0, then B reports an invalid 
/// argument error (Err:502).
/// For the case when T 2 is omitted, B checks (after any truncation) that 
/// Trials ≥ 0, T 1 ≥ 0, and Trials ≥ T 1. If any of these checks fail, 
/// then B reports an invalid argument error (Err:502).
/// For the case when T 2 is provided, B checks (after any truncation) that T 1 
/// ≥ 0, T 2 ≥ T 1, and Trials ≥ T 2. If any of these checks fail, then B 
/// reports an invalid argument error (Err:502).
/// 
/// Info:
/// 
/// The formula for B is:
/// 
/// <math xmlns="http://www.w3.org/1998/Math/MathML"><mstyle 
/// displaystyle="true" scriptlevel="0"><mrow><mtext>B</mtext><mo 
/// stretchy="false">(</mo><mi>n</mi><mo>;</mo><mstyle scriptlevel="0"><mspace 
/// width="mediummathspace"></mspace></mstyle><mi>p</mi><mo>;</mo><mstyle 
/// scriptlevel="0"><mspace 
/// width="mediummathspace"></mspace></mstyle><mi>k</mi><mn>1</mn><mo>;</mo><mstyle 
/// scriptlevel="0"><mspace 
/// width="mediummathspace"></mspace></mstyle><mi>k</mi><mn>2</mn><mo 
/// stretchy="false">)</mo><mtext>&#xA0;</mtext><mo>=</mo><mtext>&#xA0;</mtext><munderover><mo 
/// data-mjx-texclass="OP">&#x2211;</mo><mrow><mi>i</mi><mo>=</mo><mi>k</mi><mn>1</mn></mrow><mrow><mi>k</mi><mn>2</mn></mrow></munderover><mfrac><mrow><mi>n</mi><mo>!</mo></mrow><mrow><mi>i</mi><mo>!</mo><mo 
/// stretchy="false">(</mo><mi>n</mi><mo>&#x2212;</mo><mi>i</mi><mo 
/// stretchy="false">)</mo><mo>!</mo></mrow></mfrac><mstyle 
/// scriptlevel="0"><mspace 
/// width="mediummathspace"></mspace></mstyle><mo>&#xD7;</mo><mstyle 
/// scriptlevel="0"><mspace 
/// width="mediummathspace"></mspace></mstyle><msup><mi>p</mi><mrow><mi>i</mi></mrow></msup><mstyle 
/// scriptlevel="0"><mspace 
/// width="mediummathspace"></mspace></mstyle><mo>&#xD7;</mo><mstyle 
/// scriptlevel="0"><mspace width="mediummathspace"></mspace></mstyle><mo 
/// stretchy="false">(</mo><mn>1</mn><mo>&#x2212;</mo><mi>p</mi><msup><mo 
/// stretchy="false">)</mo><mrow><mi>n</mi><mo>&#x2212;</mo><mi>i</mi></mrow></msup></mrow></mstyle></math>
/// 
/// If the final argument (T 2) is omitted, then set k2 to the value of k1 in 
/// this equation, effectively removing the summation operation.
///
/// __See also__: [crate::of::b()], 
#[inline]
pub fn b_<A: Number, B: Number, C: Number, D: Number>(trials: A, s_p: B, t_1: C, t_2: D) -> FnNumber4<A, B, C, D> {
    FnNumber4("B", trials, s_p, t_1, t_2)
}

/// Calculates beta distribution values from either the probability density 
/// function or the cumulative distribution function.
/// 
/// The beta distribution is a family of continuous probability distributions 
/// that can be used to model random variables that lie within finite bounds. 
/// The distribution has two characteristic positive real numbers, usually 
/// denoted as alpha (α) and beta (β), that control its shape. In many cases 
/// the beta distribution is defined on the range [0, 1] but in the more 
/// general case can be defined on a range [a, b], where a and b are the lower 
/// and upper bounds of the distribution (a < b).
///
/// [documentfoundation->BETA.DIST](https://wiki.documentfoundation.org/Documentation/Calc_Functions/BETA.DIST)
///
/// __Syntax__: 
/// ```ods
///     BETA.DIST( Number: Number; Alpha: Number; Beta: Number; Cumulative: Logical )
/// ```
///
/// __Info2__:
/// Returns a non-negative real number, which is the beta distribution value 
/// for the given arguments. For the probability density function, the value 
/// returned lies in the range [0, +∞). For the cumulative distribution 
/// function, the value returned lies in the range [0, 1].
///
/// __Semantics__:
/// 
/// Number is a real number, or a reference to a cell containing that number, 
/// which is the value of the random variable that is to be used in the 
/// calculation. Number must lie in the range [Start, End].
/// 
/// Alpha is a positive real number, or a reference to a cell containing that 
/// number, which is the value of one of the two parameters that control the 
/// shape of the beta distribution.
/// 
/// Beta is a positive real number, or a reference to a cell containing that 
/// number, which is the value of the second of the two parameters that control 
/// the shape of the beta distribution.
/// 
/// Cumulative is a logical value, or a reference to a cell containing that 
/// value, that determines whether the required probability is taken from the 
/// probability density function or the cumulative distribution function. If 
/// Cumulative is set to 0 or FALSE, a value from the probability density 
/// function is calculated. For any other values of Cumulative, a value from 
/// the cumulative distribution function is calculated.
/// 
/// Start is a real number, or a reference to a cell containing that number, 
/// which is the lower bound of the distribution. If omitted, the default value 
/// of 0.0 is used.
/// 
/// End is a real number, or a reference to a cell containing that number, 
/// which is the upper bound of the distribution. If omitted, the default value 
/// of 1.0 is used.
/// 
/// If any of Number, Alpha, Beta, Start, or End is non-numeric, then BETA.DIST 
/// reports a #VALUE! error.
/// If either Alpha or Beta is less than or equal to 0.0, then BETA.DIST 
/// reports an invalid argument error (Err:502).
/// If Number is less than Start or greater than End, then BETA.DIST reports an 
/// invalid argument error (Err:502).
/// Other errors may arise depending on the exact combination of values passed 
/// as arguments to BETA.DIST.
/// 
/// Additional details:
/// 
/// Calc's BETADIST and BETA.DIST functions perform similar calculations. 
/// However, there are differences between the two functions with respect to 
/// the order of their arguments and the conditions placed on argument values. 
/// The requirements for BETADIST are specified in ODF 1.2; BETA.DIST is 
/// provided for interoperability with Microsoft Excel.
///
/// __See also__: [crate::of::beta_dist_()], [crate::of::beta_dist__()], 
#[inline]
pub fn beta_dist<A: Number, B: Number, C: Number, D: Logical>(number: A, alpha: B, beta: C, cumulative: D) -> FnNumber4<A, B, C, D> {
    FnNumber4("BETA.DIST", number, alpha, beta, cumulative)
}

/// Calculates beta distribution values from either the probability density 
/// function or the cumulative distribution function.
/// 
/// The beta distribution is a family of continuous probability distributions 
/// that can be used to model random variables that lie within finite bounds. 
/// The distribution has two characteristic positive real numbers, usually 
/// denoted as alpha (α) and beta (β), that control its shape. In many cases 
/// the beta distribution is defined on the range [0, 1] but in the more 
/// general case can be defined on a range [a, b], where a and b are the lower 
/// and upper bounds of the distribution (a < b).
///
/// [documentfoundation->BETA.DIST](https://wiki.documentfoundation.org/Documentation/Calc_Functions/BETA.DIST)
///
/// __Syntax__: 
/// ```ods
///     BETA.DIST( Number: Number; Alpha: Number; Beta: Number; Cumulative: Logical; Start: Number )
/// ```
///
/// __Info2__:
/// Returns a non-negative real number, which is the beta distribution value 
/// for the given arguments. For the probability density function, the value 
/// returned lies in the range [0, +∞). For the cumulative distribution 
/// function, the value returned lies in the range [0, 1].
///
/// __Semantics__:
/// 
/// Number is a real number, or a reference to a cell containing that number, 
/// which is the value of the random variable that is to be used in the 
/// calculation. Number must lie in the range [Start, End].
/// 
/// Alpha is a positive real number, or a reference to a cell containing that 
/// number, which is the value of one of the two parameters that control the 
/// shape of the beta distribution.
/// 
/// Beta is a positive real number, or a reference to a cell containing that 
/// number, which is the value of the second of the two parameters that control 
/// the shape of the beta distribution.
/// 
/// Cumulative is a logical value, or a reference to a cell containing that 
/// value, that determines whether the required probability is taken from the 
/// probability density function or the cumulative distribution function. If 
/// Cumulative is set to 0 or FALSE, a value from the probability density 
/// function is calculated. For any other values of Cumulative, a value from 
/// the cumulative distribution function is calculated.
/// 
/// Start is a real number, or a reference to a cell containing that number, 
/// which is the lower bound of the distribution. If omitted, the default value 
/// of 0.0 is used.
/// 
/// End is a real number, or a reference to a cell containing that number, 
/// which is the upper bound of the distribution. If omitted, the default value 
/// of 1.0 is used.
/// 
/// If any of Number, Alpha, Beta, Start, or End is non-numeric, then BETA.DIST 
/// reports a #VALUE! error.
/// If either Alpha or Beta is less than or equal to 0.0, then BETA.DIST 
/// reports an invalid argument error (Err:502).
/// If Number is less than Start or greater than End, then BETA.DIST reports an 
/// invalid argument error (Err:502).
/// Other errors may arise depending on the exact combination of values passed 
/// as arguments to BETA.DIST.
/// 
/// Additional details:
/// 
/// Calc's BETADIST and BETA.DIST functions perform similar calculations. 
/// However, there are differences between the two functions with respect to 
/// the order of their arguments and the conditions placed on argument values. 
/// The requirements for BETADIST are specified in ODF 1.2; BETA.DIST is 
/// provided for interoperability with Microsoft Excel.
///
/// __See also__: [crate::of::beta_dist()], [crate::of::beta_dist__()], 
#[inline]
pub fn beta_dist_<A: Number, B: Number, C: Number, D: Logical, E: Number>(number: A, alpha: B, beta: C, cumulative: D, start: E) -> FnNumber5<A, B, C, D, E> {
    FnNumber5("BETA.DIST", number, alpha, beta, cumulative, start)
}

/// Calculates beta distribution values from either the probability density 
/// function or the cumulative distribution function.
/// 
/// The beta distribution is a family of continuous probability distributions 
/// that can be used to model random variables that lie within finite bounds. 
/// The distribution has two characteristic positive real numbers, usually 
/// denoted as alpha (α) and beta (β), that control its shape. In many cases 
/// the beta distribution is defined on the range [0, 1] but in the more 
/// general case can be defined on a range [a, b], where a and b are the lower 
/// and upper bounds of the distribution (a < b).
///
/// [documentfoundation->BETA.DIST](https://wiki.documentfoundation.org/Documentation/Calc_Functions/BETA.DIST)
///
/// __Syntax__: 
/// ```ods
///     BETA.DIST( Number: Number; Alpha: Number; Beta: Number; Cumulative: Logical; Start: Number; End: Number )
/// ```
///
/// __Info2__:
/// Returns a non-negative real number, which is the beta distribution value 
/// for the given arguments. For the probability density function, the value 
/// returned lies in the range [0, +∞). For the cumulative distribution 
/// function, the value returned lies in the range [0, 1].
///
/// __Semantics__:
/// 
/// Number is a real number, or a reference to a cell containing that number, 
/// which is the value of the random variable that is to be used in the 
/// calculation. Number must lie in the range [Start, End].
/// 
/// Alpha is a positive real number, or a reference to a cell containing that 
/// number, which is the value of one of the two parameters that control the 
/// shape of the beta distribution.
/// 
/// Beta is a positive real number, or a reference to a cell containing that 
/// number, which is the value of the second of the two parameters that control 
/// the shape of the beta distribution.
/// 
/// Cumulative is a logical value, or a reference to a cell containing that 
/// value, that determines whether the required probability is taken from the 
/// probability density function or the cumulative distribution function. If 
/// Cumulative is set to 0 or FALSE, a value from the probability density 
/// function is calculated. For any other values of Cumulative, a value from 
/// the cumulative distribution function is calculated.
/// 
/// Start is a real number, or a reference to a cell containing that number, 
/// which is the lower bound of the distribution. If omitted, the default value 
/// of 0.0 is used.
/// 
/// End is a real number, or a reference to a cell containing that number, 
/// which is the upper bound of the distribution. If omitted, the default value 
/// of 1.0 is used.
/// 
/// If any of Number, Alpha, Beta, Start, or End is non-numeric, then BETA.DIST 
/// reports a #VALUE! error.
/// If either Alpha or Beta is less than or equal to 0.0, then BETA.DIST 
/// reports an invalid argument error (Err:502).
/// If Number is less than Start or greater than End, then BETA.DIST reports an 
/// invalid argument error (Err:502).
/// Other errors may arise depending on the exact combination of values passed 
/// as arguments to BETA.DIST.
/// 
/// Additional details:
/// 
/// Calc's BETADIST and BETA.DIST functions perform similar calculations. 
/// However, there are differences between the two functions with respect to 
/// the order of their arguments and the conditions placed on argument values. 
/// The requirements for BETADIST are specified in ODF 1.2; BETA.DIST is 
/// provided for interoperability with Microsoft Excel.
///
/// __See also__: [crate::of::beta_dist()], [crate::of::beta_dist_()], 
#[inline]
pub fn beta_dist__<A: Number, B: Number, C: Number, D: Logical, E: Number, F: Number>(number: A, alpha: B, beta: C, cumulative: D, start: E, end: F) -> FnNumber6<A, B, C, D, E, F> {
    FnNumber6("BETA.DIST", number, alpha, beta, cumulative, start, end)
}

/// Calculates the inverse of the cumulative distribution function for a beta 
/// distribution.
/// 
/// The beta distribution is a family of continuous probability distributions 
/// that can be used to model random variables that lie within finite bounds. 
/// The distribution has two characteristic positive real numbers, usually 
/// denoted as alpha (α) and beta (β), that control its shape. In many cases 
/// the beta distribution is defined on the range [0, 1] but in the more 
/// general case can be defined on a range [a, b], where a and b are the lower 
/// and upper bounds of the distribution (a < b).
///
/// [documentfoundation->BETA.INV](https://wiki.documentfoundation.org/Documentation/Calc_Functions/BETA.INV)
///
/// __Syntax__: 
/// ```ods
///     BETA.INV( Number: Number; Alpha: Number; Beta: Number )
/// ```
///
/// __Info2__:
/// Returns a real number, which is the value of the random variable that would 
/// give the specified probability in the cumulative distribution function for 
/// the specified beta distribution. The returned value will lie within the 
/// defined range of the distribution.
///
/// __Semantics__:
/// 
/// Number is a real number, or a reference to a cell containing that number, 
/// which is a probability in the cumulative distribution function of the beta 
/// distribution function. Number lies in the range [0, 1].
/// 
/// Alpha is a positive real number, or a reference to a cell containing that 
/// number, which is the value of one of the two parameters that control the 
/// shape of the beta distribution.
/// 
/// Beta is a positive real number, or a reference to a cell containing that 
/// number, which is the value of the second of the two parameters that control 
/// the shape of the beta distribution.
/// 
/// Start is a real number, or a reference to a cell containing that number, 
/// which is the lower bound of the distribution. If omitted, the default value 
/// of 0.0 is used.
/// 
/// End is a real number, or a reference to a cell containing that number, 
/// which is the upper bound of the distribution. If omitted, the default value 
/// of 1.0 is used.
/// 
/// If any of Number, Alpha, Beta, Start, or End is non-numeric, then BETA.INV 
/// reports a #VALUE! error.
/// If either Alpha or Beta is less than or equal to 0.0, then BETA.INV reports 
/// an invalid argument error (Err:502).
/// If Number is less than 0.0 or greater than 1.0, then BETA.INV reports an 
/// invalid argument error (Err:502).
/// If Start is greater than or equal to End, then BETA.INV reports an invalid 
/// argument error (Err:502).
/// 
/// Info:
/// 
/// Calc's BETAINV and BETA.INV functions perform the same calculations. The 
/// requirements for BETAINV are specified in ODF 1.2; BETA.INV is provided for 
/// interoperability with Microsoft Excel.
///
/// __See also__: [crate::of::beta_inv_()], [crate::of::beta_inv__()], 
#[inline]
pub fn beta_inv<A: Number, B: Number, C: Number>(number: A, alpha: B, beta: C) -> FnNumber3<A, B, C> {
    FnNumber3("BETA.INV", number, alpha, beta)
}

/// Calculates the inverse of the cumulative distribution function for a beta 
/// distribution.
/// 
/// The beta distribution is a family of continuous probability distributions 
/// that can be used to model random variables that lie within finite bounds. 
/// The distribution has two characteristic positive real numbers, usually 
/// denoted as alpha (α) and beta (β), that control its shape. In many cases 
/// the beta distribution is defined on the range [0, 1] but in the more 
/// general case can be defined on a range [a, b], where a and b are the lower 
/// and upper bounds of the distribution (a < b).
///
/// [documentfoundation->BETA.INV](https://wiki.documentfoundation.org/Documentation/Calc_Functions/BETA.INV)
///
/// __Syntax__: 
/// ```ods
///     BETA.INV( Number: Number; Alpha: Number; Beta: Number; Start: Number )
/// ```
///
/// __Info2__:
/// Returns a real number, which is the value of the random variable that would 
/// give the specified probability in the cumulative distribution function for 
/// the specified beta distribution. The returned value will lie within the 
/// defined range of the distribution.
///
/// __Semantics__:
/// 
/// Number is a real number, or a reference to a cell containing that number, 
/// which is a probability in the cumulative distribution function of the beta 
/// distribution function. Number lies in the range [0, 1].
/// 
/// Alpha is a positive real number, or a reference to a cell containing that 
/// number, which is the value of one of the two parameters that control the 
/// shape of the beta distribution.
/// 
/// Beta is a positive real number, or a reference to a cell containing that 
/// number, which is the value of the second of the two parameters that control 
/// the shape of the beta distribution.
/// 
/// Start is a real number, or a reference to a cell containing that number, 
/// which is the lower bound of the distribution. If omitted, the default value 
/// of 0.0 is used.
/// 
/// End is a real number, or a reference to a cell containing that number, 
/// which is the upper bound of the distribution. If omitted, the default value 
/// of 1.0 is used.
/// 
/// If any of Number, Alpha, Beta, Start, or End is non-numeric, then BETA.INV 
/// reports a #VALUE! error.
/// If either Alpha or Beta is less than or equal to 0.0, then BETA.INV reports 
/// an invalid argument error (Err:502).
/// If Number is less than 0.0 or greater than 1.0, then BETA.INV reports an 
/// invalid argument error (Err:502).
/// If Start is greater than or equal to End, then BETA.INV reports an invalid 
/// argument error (Err:502).
/// 
/// Info:
/// 
/// Calc's BETAINV and BETA.INV functions perform the same calculations. The 
/// requirements for BETAINV are specified in ODF 1.2; BETA.INV is provided for 
/// interoperability with Microsoft Excel.
///
/// __See also__: [crate::of::beta_inv()], [crate::of::beta_inv__()], 
#[inline]
pub fn beta_inv_<A: Number, B: Number, C: Number, D: Number>(number: A, alpha: B, beta: C, start: D) -> FnNumber4<A, B, C, D> {
    FnNumber4("BETA.INV", number, alpha, beta, start)
}

/// Calculates the inverse of the cumulative distribution function for a beta 
/// distribution.
/// 
/// The beta distribution is a family of continuous probability distributions 
/// that can be used to model random variables that lie within finite bounds. 
/// The distribution has two characteristic positive real numbers, usually 
/// denoted as alpha (α) and beta (β), that control its shape. In many cases 
/// the beta distribution is defined on the range [0, 1] but in the more 
/// general case can be defined on a range [a, b], where a and b are the lower 
/// and upper bounds of the distribution (a < b).
///
/// [documentfoundation->BETA.INV](https://wiki.documentfoundation.org/Documentation/Calc_Functions/BETA.INV)
///
/// __Syntax__: 
/// ```ods
///     BETA.INV( Number: Number; Alpha: Number; Beta: Number; Start: Number; End: Number )
/// ```
///
/// __Info2__:
/// Returns a real number, which is the value of the random variable that would 
/// give the specified probability in the cumulative distribution function for 
/// the specified beta distribution. The returned value will lie within the 
/// defined range of the distribution.
///
/// __Semantics__:
/// 
/// Number is a real number, or a reference to a cell containing that number, 
/// which is a probability in the cumulative distribution function of the beta 
/// distribution function. Number lies in the range [0, 1].
/// 
/// Alpha is a positive real number, or a reference to a cell containing that 
/// number, which is the value of one of the two parameters that control the 
/// shape of the beta distribution.
/// 
/// Beta is a positive real number, or a reference to a cell containing that 
/// number, which is the value of the second of the two parameters that control 
/// the shape of the beta distribution.
/// 
/// Start is a real number, or a reference to a cell containing that number, 
/// which is the lower bound of the distribution. If omitted, the default value 
/// of 0.0 is used.
/// 
/// End is a real number, or a reference to a cell containing that number, 
/// which is the upper bound of the distribution. If omitted, the default value 
/// of 1.0 is used.
/// 
/// If any of Number, Alpha, Beta, Start, or End is non-numeric, then BETA.INV 
/// reports a #VALUE! error.
/// If either Alpha or Beta is less than or equal to 0.0, then BETA.INV reports 
/// an invalid argument error (Err:502).
/// If Number is less than 0.0 or greater than 1.0, then BETA.INV reports an 
/// invalid argument error (Err:502).
/// If Start is greater than or equal to End, then BETA.INV reports an invalid 
/// argument error (Err:502).
/// 
/// Info:
/// 
/// Calc's BETAINV and BETA.INV functions perform the same calculations. The 
/// requirements for BETAINV are specified in ODF 1.2; BETA.INV is provided for 
/// interoperability with Microsoft Excel.
///
/// __See also__: [crate::of::beta_inv()], [crate::of::beta_inv_()], 
#[inline]
pub fn beta_inv__<A: Number, B: Number, C: Number, D: Number, E: Number>(number: A, alpha: B, beta: C, start: D, end: E) -> FnNumber5<A, B, C, D, E> {
    FnNumber5("BETA.INV", number, alpha, beta, start, end)
}

/// Calculates binomial distribution probabilities from either the probability 
/// mass function or the cumulative distribution function. The binomial 
/// distribution is a discrete probability distribution that is used to analyze 
/// data in many domains.
///
/// [documentfoundation->BINOM.DIST](https://wiki.documentfoundation.org/Documentation/Calc_Functions/BINOM.DIST)
///
/// __Syntax__: 
/// ```ods
///     BINOM.DIST( X: Integer; Trials: Integer; SP: Number; C: Logical )
/// ```
///
/// __Info2__:
/// Returns a real number in the range [0, 1], which is the binomial 
/// distribution probability for the given arguments.
///
/// __Semantics__:
/// 
/// X is a non-negative integer, or a reference to a cell containing that 
/// integer, that is the number of trial successes for which the probability is 
/// required.
/// 
/// Trials is a non-negative integer, or a reference to a cell containing that 
/// integer, that is the total number of independent trials.
/// 
/// SP is a real number (expressed as a percentage, such as 2.5%, or a decimal 
/// fraction, such as 0.025), or a reference to a cell containing that number, 
/// that is the probability of a successful outcome on each trial. As a 
/// probability, SP lies in the range [0, 1] (or equivalently 0% ≤ SP ≤ 
/// 100%).
/// 
/// C is a logical value, or a reference to a cell containing that value, that 
/// determines whether the required probability is taken from the probability 
/// mass function or the cumulative distribution function. If C is set to 0 or 
/// FALSE, a value from the probability mass function is calculated. For any 
/// other values of C, a value from the cumulative distribution function is 
/// calculated.
/// 
/// If any of X, Trials, SP, and C is non-numeric, then BINOM.DIST reports a 
/// #VALUE! error.
/// If either of X or Trials is a non-integer value, then BINOM.DIST truncates 
/// it to an integer value.
/// If SP is less than 0.0 or greater than 1.0, then BINOM.DIST reports an 
/// invalid argument error (Err:502).
/// BINOM.DIST checks (after any truncation) that Trials ≥ 0, X ≥ 0, and 
/// Trials ≥ X. If any of these checks fail, then BINOM.DIST reports an 
/// invalid argument error (Err:502).
/// 
/// Info:
/// 
/// Calc's BINOM.DIST and BINOMDIST functions perform the same calculations. 
/// The requirements for BINOMDIST are specified in ODF 1.2; BINOM.DIST is 
/// provided for interoperability with Microsoft Excel.
///
/// __See also__: 
#[inline]
pub fn binom_dist<A: Number, B: Number, C: Number, D: Logical>(x: A, trials: B, s_p: C, c: D) -> FnNumber4<A, B, C, D> {
    FnNumber4("BINOM.DIST", x, trials, s_p, c)
}

/// Given the total number of independent trials and the probability of a 
/// successful outcome on each trial, BINOM.INV determines the minimum number 
/// of trial successes for which the binomial cumulative density function gives 
/// a probability of greater than or equal to a supplied criterion probability.
///
/// [documentfoundation->BINOM.INV](https://wiki.documentfoundation.org/Documentation/Calc_Functions/BINOM.INV)
///
/// __Syntax__: 
/// ```ods
///     BINOM.INV( Trials: Integer; SP: Number; Alpha: Number )
/// ```
///
/// __Info2__:
/// Returns a non-negative integer, which is the minimum number of trial 
/// successes for the given arguments.
///
/// __Semantics__:
/// 
/// Trials is a non-negative integer, or a reference to a cell containing that 
/// integer, that is the total number of independent trials.
/// 
/// SP is a real number (expressed as a percentage, such as 2.5%, or a decimal 
/// fraction, such as 0.025), or a reference to a cell containing that number, 
/// that is the probability of a successful outcome on each trial. As a 
/// probability, SP lies in the range [0, 1] (or equivalently 0% ≤ SP ≤ 
/// 100%).
/// 
/// Alpha is a real number (expressed as a percentage, such as 2.5%, or a 
/// decimal fraction, such as 0.025), or a reference to a cell containing that 
/// number, that is the criterion probability to be reached or exceeded. As a 
/// probability, Alpha lies in the range [0, 1] (or equivalently 0% ≤ Alpha 
/// ≤ 100%).
/// 
/// If any of Trials, SP, and Alpha is non-numeric, then BINOM.INV reports a 
/// #VALUE! error.
/// If Trials is a non-integer value, then BINOM.INV truncates it to an integer 
/// value.
/// If Trials is a less than 0, then BINOM.INV reports an invalid argument 
/// error (Err:502).
/// If either SP or Alpha is less than 0.0 or greater than 1.0, then BINOM.INV 
/// reports an invalid argument error (Err:502).
/// 
/// Info:
/// 
/// Calc's CRITBINOM and BINOM.INV functions perform the same calculations. The 
/// requirements for CRITBINOM are specified in ODF 1.2; BINOM.INV is provided 
/// for interoperability with Microsoft Excel.
///
/// __See also__: 
#[inline]
pub fn binom_inv<A: Number, B: Number, C: Number>(trials: A, s_p: B, alpha: C) -> FnNumber3<A, B, C> {
    FnNumber3("BINOM.INV", trials, s_p, alpha)
}
