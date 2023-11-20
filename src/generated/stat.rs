//! 
//! The following are statistical functions (functions that report information 
//! on a set of numbers). Some functions that could also be considered 
//! statistical functions, such as SUM, are listed elsewhere.

use crate::*;
#[allow(unused_imports)]
use crate::stat::*;

/// Calculates the average of the absolute deviations of the values in list.
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
/// __See also__: "SUM", "AVERAGE", 
#[inline]
pub fn avedev<A: Sequence>(n: A) -> FnNumber1<A> {
    FnNumber1("AVEDEV", n)
}

/// Average the set of numbers
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
/// __See also__: "SUM", "COUNT", 
#[inline]
pub fn average<A: Sequence>(n: A) -> FnNumber1<A> {
    FnNumber1("AVERAGE", n)
}

/// Average values, including values of type Text and Logical.
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
/// __See also__: "AVERAGE", 
#[inline]
pub fn averagea<A: Sequence>(n: A) -> FnNumber1<A> {
    FnNumber1("AVERAGEA", n)
}

/// Average the values of cells in a range that meet a criteria.
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
/// __See also__: "AVERAGEIFS", "COUNTIF", "SUMIF", "Infix Operator \"=\"", "Infix Operator \"<>\"", "Infix Operator Ordered Comparison (\"<\", \"<=\", \">\", \">=\")", 
#[inline]
pub fn averageif<A: Reference, B: Criterion>(r: A, c: B) -> FnNumber2<A, B> {
    FnNumber2("AVERAGEIF", r, c)
}

/// Average the values of cells in a range that meet a criteria.
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
/// __See also__: "AVERAGEIFS", "COUNTIF", "SUMIF", "Infix Operator \"=\"", "Infix Operator \"<>\"", "Infix Operator Ordered Comparison (\"<\", \"<=\", \">\", \">=\")", 
#[inline]
pub fn averageif_<A: Reference, B: Criterion, C: Reference>(r: A, c: B, a: C) -> FnNumber3<A, B, C> {
    FnNumber3("AVERAGEIF", r, c, a)
}

/// returns the value of the probability density function or the cumulative 
/// distribution function for the beta distribution.
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
/// __See also__: "BETAINV", 
#[inline]
pub fn betadist<A: Number, B: Number, C: Number>(x: A, alpha: B, beta: C) -> FnNumber3<A, B, C> {
    FnNumber3("BETADIST", x, alpha, beta)
}

/// returns the value of the probability density function or the cumulative 
/// distribution function for the beta distribution.
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
/// __See also__: "BETAINV", 
#[inline]
pub fn betadist_<A: Number, B: Number, C: Number, D: Number>(x: A, alpha: B, beta: C, a: D) -> FnNumber4<A, B, C, D> {
    FnNumber4("BETADIST", x, alpha, beta, a)
}

/// returns the value of the probability density function or the cumulative 
/// distribution function for the beta distribution.
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
/// __See also__: "BETAINV", 
#[inline]
pub fn betadist__<A: Number, B: Number, C: Number, D: Number, E: Number>(x: A, alpha: B, beta: C, a: D, b: E) -> FnNumber5<A, B, C, D, E> {
    FnNumber5("BETADIST", x, alpha, beta, a, b)
}

/// returns the value of the probability density function or the cumulative 
/// distribution function for the beta distribution.
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
/// __See also__: "BETAINV", 
#[inline]
pub fn betadist___<A: Number, B: Number, C: Number, D: Number, E: Number, F: Logical>(x: A, alpha: B, beta: C, a: D, b: E, cumulative: F) -> FnNumber6<A, B, C, D, E, F> {
    FnNumber6("BETADIST", x, alpha, beta, a, b, cumulative)
}

/// returns the inverse of BETADIST(x;α;β;A;B;TRUE()).
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
/// __See also__: "BETADIST", 
#[inline]
pub fn betainv<A: Number, B: Number, C: Number>(p: A, alpha: B, beta: C) -> FnNumber3<A, B, C> {
    FnNumber3("BETAINV", p, alpha, beta)
}

/// returns the inverse of BETADIST(x;α;β;A;B;TRUE()).
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
/// __See also__: "BETADIST", 
#[inline]
pub fn betainv_<A: Number, B: Number, C: Number, D: Number>(p: A, alpha: B, beta: C, a: D) -> FnNumber4<A, B, C, D> {
    FnNumber4("BETAINV", p, alpha, beta, a)
}

/// returns the inverse of BETADIST(x;α;β;A;B;TRUE()).
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
/// __See also__: "BETADIST", 
#[inline]
pub fn betainv__<A: Number, B: Number, C: Number, D: Number, E: Number>(p: A, alpha: B, beta: C, a: D, b: E) -> FnNumber5<A, B, C, D, E> {
    FnNumber5("BETAINV", p, alpha, beta, a, b)
}

/// Returns the probability of a trial result using binomial distribution.
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
/// __See also__: "BINOMDIST", 
#[inline]
pub fn binom_dist_range<A: Number, B: Number, C: Number>(n: A, p: B, s: C) -> FnNumber3<A, B, C> {
    FnNumber3("BINOM.DIST.RANGE", n, p, s)
}

/// Returns the probability of a trial result using binomial distribution.
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
/// __See also__: "BINOMDIST", 
#[inline]
pub fn binom_dist_range_<A: Number, B: Number, C: Number, D: Number>(n: A, p: B, s: C, s2: D) -> FnNumber4<A, B, C, D> {
    FnNumber4("BINOM.DIST.RANGE", n, p, s, s2)
}

/// Returns the binomial distribution.
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
/// __See also__: "BINOM.DIST.RANGE", 
#[inline]
pub fn binomdist<A: Number, B: Number, C: Number, D: Logical>(s: A, n: B, p: C, cumulative: D) -> FnNumber4<A, B, C, D> {
    FnNumber4("BINOMDIST", s, n, p, cumulative)
}

/// returns the right-tail probability for the χ2-distribution.
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
/// __See also__: "CHISQDIST", "LEGACY.CHITEST", 
#[inline]
pub fn legacy_chidist<A: Number, B: Number>(x: A, degrees_of_freedom: B) -> FnNumber2<A, B> {
    FnNumber2("LEGACY.CHIDIST", x, degrees_of_freedom)
}

/// returns the value of the probability density function or the cumulative 
/// distribution function for the χ2-distribution.
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
/// __See also__: "LEGACY.CHIDIST", 
#[inline]
pub fn chisqdist<A: Number, B: Number>(x: A, degrees_of_freedom: B) -> FnNumber2<A, B> {
    FnNumber2("CHISQDIST", x, degrees_of_freedom)
}

/// returns the value of the probability density function or the cumulative 
/// distribution function for the χ2-distribution.
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
/// __See also__: "LEGACY.CHIDIST", 
#[inline]
pub fn chisqdist_<A: Number, B: Number, C: Logical>(x: A, degrees_of_freedom: B, cumulative: C) -> FnNumber3<A, B, C> {
    FnNumber3("CHISQDIST", x, degrees_of_freedom, cumulative)
}

/// returns the inverse of LEGACY.CHIDIST(x; DegreesOfFreedom).
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
/// __See also__: "LEGACY.CHIDIST", 
#[inline]
pub fn legacy_chiinv<A: Number, B: Number>(p: A, degrees_of_freedom: B) -> FnNumber2<A, B> {
    FnNumber2("LEGACY.CHIINV", p, degrees_of_freedom)
}

/// returns the inverse of CHISQDIST(x; DegreesOfFreedom; TRUE()).
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
/// __See also__: "CHISQDIST", 
#[inline]
pub fn chisqinv<A: Number, B: Number>(p: A, degrees_of_freedom: B) -> FnNumber2<A, B> {
    FnNumber2("CHISQINV", p, degrees_of_freedom)
}

/// Returns some Chi square goodness-for-fit test.
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
/// __See also__: "COLUMNS", "ROWS", "LEGACY.CHIDIST", 
#[inline]
pub fn legacy_chitest<A: Array, B: Array>(a: A, e: B) -> FnNumber2<A, B> {
    FnNumber2("LEGACY.CHITEST", a, e)
}

/// Returns the confidence interval for a population mean.
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
/// __See also__: "NORMINV", "SQRT", 
#[inline]
pub fn confidence<A: Number, B: Number, C: Number>(alpha: A, stddev: B, size: C) -> FnNumber3<A, B, C> {
    FnNumber3("CONFIDENCE", alpha, stddev, size)
}

/// Calculates the correlation coefficient of values in N1 and N2.
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
/// __See also__: "COLUMNS", "ROWS", "COVAR", "STDEVP", "PEARSON", 
#[inline]
pub fn correl<A: Array, B: Array>(n1: A, n2: B) -> FnNumber2<A, B> {
    FnNumber2("CORREL", n1, n2)
}

/// Calculates covariance of two cell ranges.
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
/// __See also__: "COLUMNS", "ROWS", "AVERAGE", 
#[inline]
pub fn covar<A: Array, B: Array>(n1: A, n2: B) -> FnNumber2<A, B> {
    FnNumber2("COVAR", n1, n2)
}

/// Returns the smallest value for which the cumulative binomial distribution 
/// is greater than or equal to a criterion value.
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
#[inline]
pub fn critbinom<A: Number, B: Number, C: Number>(trials: A, s_p: B, alpha: C) -> FnNumber3<A, B, C> {
    FnNumber3("CRITBINOM", trials, s_p, alpha)
}

/// Calculates sum of squares of deviations.
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
#[inline]
pub fn devsq<A: Sequence>(n: A) -> FnNumber1<A> {
    FnNumber1("DEVSQ", n)
}

/// returns the value of the probability density function or the cumulative 
/// distribution function for the exponential distribution.
///
/// __Syntax__: 
/// ```ods
///     EXPONDIST( X: Number; λ: Number )
/// ```
///
/// __Constraints__:
/// λ > 0
///
/// __Semantics__:
/// If Cumulative is FALSE, EXPONDIST returns 0 if X < 0 and the value
/// 
/// otherwise.
/// 
/// If Cumulative is TRUE, EXPONDIST returns 0 if X < 0 and the value
/// 
/// otherwise.
#[inline]
pub fn expondist<A: Number, B: Number>(x: A, λ: B) -> FnNumber2<A, B> {
    FnNumber2("EXPONDIST", x, λ)
}

/// returns the value of the probability density function or the cumulative 
/// distribution function for the exponential distribution.
///
/// __Syntax__: 
/// ```ods
///     EXPONDIST( X: Number; λ: Number; Cumulative: Logical )
/// ```
///
/// __Constraints__:
/// λ > 0
///
/// __Semantics__:
/// If Cumulative is FALSE, EXPONDIST returns 0 if X < 0 and the value
/// 
/// otherwise.
/// 
/// If Cumulative is TRUE, EXPONDIST returns 0 if X < 0 and the value
/// 
/// otherwise.
#[inline]
pub fn expondist_<A: Number, B: Number, C: Logical>(x: A, λ: B, cumulative: C) -> FnNumber3<A, B, C> {
    FnNumber3("EXPONDIST", x, λ, cumulative)
}

/// returns the value of the probability density function or the cumulative 
/// distribution function for the F-distribution.
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
/// __See also__: "LEGACY.FDIST", 
#[inline]
pub fn fdist<A: Number, B: Number, C: Number>(x: A, r1: B, r2: C) -> FnNumber3<A, B, C> {
    FnNumber3("FDIST", x, r1, r2)
}

/// returns the value of the probability density function or the cumulative 
/// distribution function for the F-distribution.
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
/// __See also__: "LEGACY.FDIST", 
#[inline]
pub fn fdist_<A: Number, B: Number, C: Number, D: Logical>(x: A, r1: B, r2: C, cumulative: D) -> FnNumber4<A, B, C, D> {
    FnNumber4("FDIST", x, r1, r2, cumulative)
}

/// returns the area of the right tail of the probability density function for 
/// the F-distribution.
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
/// __See also__: "FDIST", 
#[inline]
pub fn legacy_fdist<A: Number, B: Number, C: Number>(x: A, r1: B, r2: C) -> FnNumber3<A, B, C> {
    FnNumber3("LEGACY.FDIST", x, r1, r2)
}

/// returns the inverse of FDIST(x;R1;R2;TRUE()).
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
/// __See also__: "FDIST", "LEGACY.FDIST", "LEGACY.FINV", 
#[inline]
pub fn finv<A: Number, B: Number, C: Number>(p: A, r1: B, r2: C) -> FnNumber3<A, B, C> {
    FnNumber3("FINV", p, r1, r2)
}

/// returns the inverse of LEGACY.FDIST(x;R1;R2).
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
/// __See also__: "FDIST", "LEGACY.FDIST", "FINV", 
#[inline]
pub fn legacy_finv<A: Number, B: Number, C: Number>(p: A, r1: B, r2: C) -> FnNumber3<A, B, C> {
    FnNumber3("LEGACY.FINV", p, r1, r2)
}

/// returns the Fisher transformation.
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
/// __See also__: "ATANH", 
#[inline]
pub fn fisher<A: Number>(r: A) -> FnNumber1<A> {
    FnNumber1("FISHER", r)
}

/// returns the inverse Fisher transformation.
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
/// __See also__: "TANH", 
#[inline]
pub fn fisherinv<A: Number>(r: A) -> FnNumber1<A> {
    FnNumber1("FISHERINV", r)
}

/// Extrapolates future values based on existing x and y values.
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
/// __See also__: "COLUMNS", "ROWS", 
#[inline]
pub fn forecast<A: Number, B: Array, C: Array>(value: A, data_y: B, data_x: C) -> FnNumber3<A, B, C> {
    FnNumber3("FORECAST", value, data_y, data_x)
}

/// Categorizes values into intervals and counts the number of values in each 
/// interval.
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
#[inline]
pub fn frequency<A: Sequence, B: Sequence>(data: A, bins: B) -> FnArray2<A, B> {
    FnArray2("FREQUENCY", data, bins)
}

/// Calculates the probability of an F-test.
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
#[inline]
pub fn ftest<A: Sequence, B: Sequence>(data_1: A, data_2: B) -> FnNumber2<A, B> {
    FnNumber2("FTEST", data_1, data_2)
}

/// returns the value of the probability density function or the cumulative 
/// distribution function for the Gamma distribution.
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
/// __See also__: "GAMMA", "GAMMAINV", 
#[inline]
pub fn gammadist<A: Number, B: Number, C: Number>(x: A, alpha: B, beta: C) -> FnNumber3<A, B, C> {
    FnNumber3("GAMMADIST", x, alpha, beta)
}

/// returns the value of the probability density function or the cumulative 
/// distribution function for the Gamma distribution.
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
/// __See also__: "GAMMA", "GAMMAINV", 
#[inline]
pub fn gammadist_<A: Number, B: Number, C: Number, D: Logical>(x: A, alpha: B, beta: C, cumulative: D) -> FnNumber4<A, B, C, D> {
    FnNumber4("GAMMADIST", x, alpha, beta, cumulative)
}

/// returns the inverse of GAMMADIST(X;α;β;TRUE).
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
/// __See also__: "GAMMADIST", 
#[inline]
pub fn gammainv<A: Number, B: Number, C: Number>(p: A, alpha: B, beta: C) -> FnNumber3<A, B, C> {
    FnNumber3("GAMMAINV", p, alpha, beta)
}

/// Returns 0.5 less than the standard normal cumulative distribution
///
/// __Syntax__: 
/// ```ods
///     GAUSS( X: Number )
/// ```
///
/// __Semantics__:
/// Returns NORMDIST(X;0;1;TRUE())-0.5
///
/// __See also__: "NORMDIST", 
#[inline]
pub fn gauss<A: Number>(x: A) -> FnNumber1<A> {
    FnNumber1("GAUSS", x)
}

/// returns the geometric mean of a sequence
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
/// __See also__: "COUNT", 
#[inline]
pub fn geomean<A: Sequence>(n: A) -> FnNumber1<A> {
    FnNumber1("GEOMEAN", n)
}

/// Returns predicted values based on an exponential regression.
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
/// __See also__: "COLUMNS", "ROWS", "LOGEST", "TREND", 
#[inline]
pub fn growth<A: Array>(known_y: A) -> FnArray1<A> {
    FnArray1("GROWTH", known_y)
}

/// Returns predicted values based on an exponential regression.
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
/// __See also__: "COLUMNS", "ROWS", "LOGEST", "TREND", 
#[inline]
pub fn growth_<A: Array, B: Array>(known_y: A, known_x: B) -> FnArray2<A, B> {
    FnArray2("GROWTH", known_y, known_x)
}

/// Returns predicted values based on an exponential regression.
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
/// __See also__: "COLUMNS", "ROWS", "LOGEST", "TREND", 
#[inline]
pub fn growth__<A: Array, B: Array, C: Array>(known_y: A, known_x: B, new_x: C) -> FnArray3<A, B, C> {
    FnArray3("GROWTH", known_y, known_x, new_x)
}

/// Returns predicted values based on an exponential regression.
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
/// __See also__: "COLUMNS", "ROWS", "LOGEST", "TREND", 
#[inline]
pub fn growth___<A: Array, B: Array, C: Array, D: Logical>(known_y: A, known_x: B, new_x: C, const_: D) -> FnArray4<A, B, C, D> {
    FnArray4("GROWTH", known_y, known_x, new_x, const_)
}

/// returns the harmonic mean of a sequence
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
/// __See also__: "COUNT", 
#[inline]
pub fn harmean<A: Sequence>(n: A) -> FnNumber1<A> {
    FnNumber1("HARMEAN", n)
}

/// The hypergeometric distribution returns the number of successes in a 
/// sequence of n draws from a finite population without replacement.
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
#[inline]
pub fn hypgeomdist<A: Number, B: Number, C: Number, D: Number>(x: A, t: B, m: C, n: D) -> FnNumber4<A, B, C, D> {
    FnNumber4("HYPGEOMDIST", x, t, m, n)
}

/// The hypergeometric distribution returns the number of successes in a 
/// sequence of n draws from a finite population without replacement.
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
#[inline]
pub fn hypgeomdist_<A: Number, B: Number, C: Number, D: Number, E: Logical>(x: A, t: B, m: C, n: D, cumulative: E) -> FnNumber5<A, B, C, D, E> {
    FnNumber5("HYPGEOMDIST", x, t, m, n, cumulative)
}

/// Returns the y-intercept of the linear regression line for the given data.
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
/// __See also__: "COLUMNS", "ROWS", 
#[inline]
pub fn intercept<A: Array, B: Array>(data_y: A, data_x: B) -> FnNumber2<A, B> {
    FnNumber2("INTERCEPT", data_y, data_x)
}

/// Return the kurtosis (“peakedness”) of a data set.
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
/// __See also__: "STDEV", 
#[inline]
pub fn kurt<A: Sequence>(x: A) -> FnNumber1<A> {
    FnNumber1("KURT", x)
}

/// Finds the nth largest value in a list.
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
/// __See also__: "SMALL", "ROUNDUP", 
#[inline]
pub fn large<A: Sequence, B: NumberOrArray>(list: A, n: B) -> FnArray2<A, B> {
    FnArray2("LARGE", list, n)
}

/// Returns the parameters of the (simple or multiple) linear regression 
/// equation for the given data and, optionally, statistics on this regression.
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
/// __See also__: "COLUMNS", "ROWS", 
#[inline]
pub fn linest<A: Array>(known_y: A) -> FnArray1<A> {
    FnArray1("LINEST", known_y)
}

/// Returns the parameters of the (simple or multiple) linear regression 
/// equation for the given data and, optionally, statistics on this regression.
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
/// __See also__: "COLUMNS", "ROWS", 
#[inline]
pub fn linest_<A: Array, B: Array>(known_y: A, known_x: B) -> FnArray2<A, B> {
    FnArray2("LINEST", known_y, known_x)
}

/// Returns the parameters of the (simple or multiple) linear regression 
/// equation for the given data and, optionally, statistics on this regression.
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
/// __See also__: "COLUMNS", "ROWS", 
#[inline]
pub fn linest__<A: Array, B: Array, C: Logical>(known_y: A, known_x: B, const_: C) -> FnArray3<A, B, C> {
    FnArray3("LINEST", known_y, known_x, const_)
}

/// Returns the parameters of the (simple or multiple) linear regression 
/// equation for the given data and, optionally, statistics on this regression.
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
/// __See also__: "COLUMNS", "ROWS", 
#[inline]
pub fn linest___<A: Array, B: Array, C: Logical, D: Logical>(known_y: A, known_x: B, const_: C, stats: D) -> FnArray4<A, B, C, D> {
    FnArray4("LINEST", known_y, known_x, const_, stats)
}

/// Returns the parameters of an exponential regression equation for the given 
/// data obtained by linearizing this intrinsically linear response function 
/// and returns, optionally, statistics on this regression.
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
/// __See also__: "COLUMNS", "ROWS", 
#[inline]
pub fn logest<A: Array>(known_y: A) -> FnArray1<A> {
    FnArray1("LOGEST", known_y)
}

/// Returns the parameters of an exponential regression equation for the given 
/// data obtained by linearizing this intrinsically linear response function 
/// and returns, optionally, statistics on this regression.
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
/// __See also__: "COLUMNS", "ROWS", 
#[inline]
pub fn logest_<A: Array, B: Array>(known_y: A, known_x: B) -> FnArray2<A, B> {
    FnArray2("LOGEST", known_y, known_x)
}

/// Returns the parameters of an exponential regression equation for the given 
/// data obtained by linearizing this intrinsically linear response function 
/// and returns, optionally, statistics on this regression.
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
/// __See also__: "COLUMNS", "ROWS", 
#[inline]
pub fn logest__<A: Array, B: Array, C: Logical>(known_y: A, known_x: B, const_: C) -> FnArray3<A, B, C> {
    FnArray3("LOGEST", known_y, known_x, const_)
}

/// Returns the parameters of an exponential regression equation for the given 
/// data obtained by linearizing this intrinsically linear response function 
/// and returns, optionally, statistics on this regression.
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
/// __See also__: "COLUMNS", "ROWS", 
#[inline]
pub fn logest___<A: Array, B: Array, C: Logical, D: Logical>(known_y: A, known_x: B, const_: C, stats: D) -> FnArray4<A, B, C, D> {
    FnArray4("LOGEST", known_y, known_x, const_, stats)
}

/// returns the inverse of LOGNORMDIST(x;Mean;StandardDeviation,TRUE()).
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
/// __See also__: "LOGNORMDIST", 
#[inline]
pub fn loginv<A: Number>(p: A) -> FnNumber1<A> {
    FnNumber1("LOGINV", p)
}

/// returns the inverse of LOGNORMDIST(x;Mean;StandardDeviation,TRUE()).
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
/// __See also__: "LOGNORMDIST", 
#[inline]
pub fn loginv_<A: Number, B: Number>(p: A, mean: B) -> FnNumber2<A, B> {
    FnNumber2("LOGINV", p, mean)
}

/// returns the inverse of LOGNORMDIST(x;Mean;StandardDeviation,TRUE()).
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
/// __See also__: "LOGNORMDIST", 
#[inline]
pub fn loginv__<A: Number, B: Number, C: Number>(p: A, mean: B, standard_deviation: C) -> FnNumber3<A, B, C> {
    FnNumber3("LOGINV", p, mean, standard_deviation)
}

/// returns the value of the probability density function or the cumulative 
/// distribution function for the lognormal distribution with the mean and 
/// standard deviation given.
///
/// __Syntax__: 
/// ```ods
///     LOGNORMDIST( X: Number )
/// ```
///
/// __Constraints__:
/// σ > 0; X > 0 if Cumulative is FALSE
///
/// __Semantics__:
/// If Cumulative is FALSE, LOGNORMDIST returns the value
/// 
/// If Cumulative is TRUE, LOGNORMDIST returns the value
/// 
/// if X > 0 and 0 otherwise.
#[inline]
pub fn lognormdist<A: Number>(x: A) -> FnNumber1<A> {
    FnNumber1("LOGNORMDIST", x)
}

/// returns the value of the probability density function or the cumulative 
/// distribution function for the lognormal distribution with the mean and 
/// standard deviation given.
///
/// __Syntax__: 
/// ```ods
///     LOGNORMDIST( X: Number; μ: Number )
/// ```
///
/// __Constraints__:
/// σ > 0; X > 0 if Cumulative is FALSE
///
/// __Semantics__:
/// If Cumulative is FALSE, LOGNORMDIST returns the value
/// 
/// If Cumulative is TRUE, LOGNORMDIST returns the value
/// 
/// if X > 0 and 0 otherwise.
#[inline]
pub fn lognormdist_<A: Number, B: Number>(x: A, μ: B) -> FnNumber2<A, B> {
    FnNumber2("LOGNORMDIST", x, μ)
}

/// returns the value of the probability density function or the cumulative 
/// distribution function for the lognormal distribution with the mean and 
/// standard deviation given.
///
/// __Syntax__: 
/// ```ods
///     LOGNORMDIST( X: Number; μ: Number; σ: Number )
/// ```
///
/// __Constraints__:
/// σ > 0; X > 0 if Cumulative is FALSE
///
/// __Semantics__:
/// If Cumulative is FALSE, LOGNORMDIST returns the value
/// 
/// If Cumulative is TRUE, LOGNORMDIST returns the value
/// 
/// if X > 0 and 0 otherwise.
#[inline]
pub fn lognormdist__<A: Number, B: Number, C: Number>(x: A, μ: B, σ: C) -> FnNumber3<A, B, C> {
    FnNumber3("LOGNORMDIST", x, μ, σ)
}

/// returns the value of the probability density function or the cumulative 
/// distribution function for the lognormal distribution with the mean and 
/// standard deviation given.
///
/// __Syntax__: 
/// ```ods
///     LOGNORMDIST( X: Number; μ: Number; σ: Number; Cumulative: Logical )
/// ```
///
/// __Constraints__:
/// σ > 0; X > 0 if Cumulative is FALSE
///
/// __Semantics__:
/// If Cumulative is FALSE, LOGNORMDIST returns the value
/// 
/// If Cumulative is TRUE, LOGNORMDIST returns the value
/// 
/// if X > 0 and 0 otherwise.
#[inline]
pub fn lognormdist___<A: Number, B: Number, C: Number, D: Logical>(x: A, μ: B, σ: C, cumulative: D) -> FnNumber4<A, B, C, D> {
    FnNumber4("LOGNORMDIST", x, μ, σ, cumulative)
}

/// Return the maximum from a set of numbers.
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
/// __See also__: "MAXA", "MIN", 
#[inline]
pub fn max<A: Sequence>(n: A) -> FnNumber1<A> {
    FnNumber1("MAX", n)
}

/// Return the maximum from a set of values, including values of type Text and 
/// Logical.
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
/// __See also__: "MAX", "MIN", "MINA", 
#[inline]
pub fn maxa<A: Sequence>(n: A) -> FnNumber1<A> {
    FnNumber1("MAXA", n)
}

/// Returns the median (middle) value in the list.
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
#[inline]
pub fn median<A: Sequence>(x: A) -> FnNumber1<A> {
    FnNumber1("MEDIAN", x)
}

/// Return the minimum from a set of numbers.
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
/// __See also__: "MAX", "MINA", 
#[inline]
pub fn min<A: Sequence>(n: A) -> FnNumber1<A> {
    FnNumber1("MIN", n)
}

/// Return the minimum from a set of values, including values of type Text and 
/// Logical.
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
/// __See also__: "MIN", "MAXA", 
#[inline]
pub fn mina<A: Sequence>(n: A) -> FnNumber1<A> {
    FnNumber1("MINA", n)
}

/// Returns the most common value in a data set.
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
#[inline]
pub fn mode<A: Sequence>(n: A) -> FnNumber1<A> {
    FnNumber1("MODE", n)
}

/// Returns the negative binomial distribution.
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
#[inline]
pub fn negbinomdist<A: Number, B: Number, C: Number>(x: A, r: B, prob: C) -> FnNumber3<A, B, C> {
    FnNumber3("NEGBINOMDIST", x, r, prob)
}

/// returns the value of the probability density function or the cumulative 
/// distribution function for the normal distribution with the mean and 
/// standard deviation given.
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
/// In the following μ is Mean and σ is StandardDeviation.
/// 
/// If Cumulative is FALSE, NORMDIST returns the value
/// 
/// If Cumulative is TRUE, NORMDIST returns the value
///
/// __See also__: "LEGACY.NORMSDIST", 
#[inline]
pub fn normdist<A: Number, B: Number, C: Number>(x: A, mean: B, standard_deviation: C) -> FnNumber3<A, B, C> {
    FnNumber3("NORMDIST", x, mean, standard_deviation)
}

/// returns the value of the probability density function or the cumulative 
/// distribution function for the normal distribution with the mean and 
/// standard deviation given.
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
/// In the following μ is Mean and σ is StandardDeviation.
/// 
/// If Cumulative is FALSE, NORMDIST returns the value
/// 
/// If Cumulative is TRUE, NORMDIST returns the value
///
/// __See also__: "LEGACY.NORMSDIST", 
#[inline]
pub fn normdist_<A: Number, B: Number, C: Number, D: Logical>(x: A, mean: B, standard_deviation: C, cumulative: D) -> FnNumber4<A, B, C, D> {
    FnNumber4("NORMDIST", x, mean, standard_deviation, cumulative)
}

/// returns the inverse of NORMDIST(x;Mean;StandardDeviation,TRUE()).
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
/// __See also__: "NORMDIST", 
#[inline]
pub fn norminv<A: Number, B: Number, C: Number>(p: A, mean: B, standard_deviation: C) -> FnNumber3<A, B, C> {
    FnNumber3("NORMINV", p, mean, standard_deviation)
}

/// returns the value of the cumulative distribution function for the standard 
/// normal distribution.
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
/// __See also__: "NORMDIST", "LEGACY.NORMSINV", 
#[inline]
pub fn legacy_normsdist<A: Number>(x: A) -> FnNumber1<A> {
    FnNumber1("LEGACY.NORMSDIST", x)
}

/// returns the inverse of LEGACY.NORMSDIST(X).
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
/// __See also__: "NORMINV", "LEGACY.NORMSDIST", 
#[inline]
pub fn legacy_normsinv<A: Number>(p: A) -> FnNumber1<A> {
    FnNumber1("LEGACY.NORMSINV", p)
}

/// PEARSON returns the Pearson correlation coefficient of two data sets
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
/// __See also__: "COLUMNS", "ROWS", 
#[inline]
pub fn pearson<A: Array, B: Array>(independent_values: A, dependent_values: B) -> FnNumber2<A, B> {
    FnNumber2("PEARSON", independent_values, dependent_values)
}

/// Calculates the x-th sample percentile among the values in range.
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
/// __See also__: "COUNT", "MAX", "MAX", "MEDIAN", "MIN", "PERCENTRANK", "QUARTILE", "RANK", 
#[inline]
pub fn percentile<A: Sequence, B: Number>(data: A, x: B) -> FnNumber2<A, B> {
    FnNumber2("PERCENTILE", data, x)
}

/// Returns the percentage rank of a value in a sample.
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
/// __See also__: "COUNT", "INT", "MAX", "MIN", "PERCENTILE", "RANK", 
#[inline]
pub fn percentrank<A: Sequence, B: Number>(data: A, x: B) -> FnNumber2<A, B> {
    FnNumber2("PERCENTRANK", data, x)
}

/// Returns the percentage rank of a value in a sample.
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
/// __See also__: "COUNT", "INT", "MAX", "MIN", "PERCENTILE", "RANK", 
#[inline]
pub fn percentrank_<A: Sequence, B: Number, C: Number>(data: A, x: B, significance: C) -> FnNumber3<A, B, C> {
    FnNumber3("PERCENTRANK", data, x, significance)
}

/// returns the number of permutations of k objects taken from n objects.
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
#[inline]
pub fn permut<A: Number, B: Number>(n: A, k: B) -> FnNumber2<A, B> {
    FnNumber2("PERMUT", n, k)
}

/// Returns the number of permutations for a given number of objects 
/// (repetition allowed).
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
#[inline]
pub fn permutationa<A: Number, B: Number>(total: A, chosen: B) -> FnNumber2<A, B> {
    FnNumber2("PERMUTATIONA", total, chosen)
}

/// Returns the values of the density function for a standard normal 
/// distribution.
///
/// __Syntax__: 
/// ```ods
///     PHI( N: Number )
/// ```
///
/// __Semantics__:
/// PHI(N) is a synonym for NORMDIST(N,0,1,FALSE()).
///
/// __See also__: "NORMDIST", 
#[inline]
pub fn phi<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("PHI", n)
}

/// returns the probability or the cumulative distribution function for the 
/// Poisson distribution
///
/// __Syntax__: 
/// ```ods
///     POISSON( X: Integer; λ: Number )
/// ```
///
/// __Constraints__:
/// λ > 0, X ≥ 0
///
/// __Semantics__:
/// If Cumulative is FALSE, POISSON returns the value
/// 
/// If Cumulative is TRUE, POISSON returns the value
#[inline]
pub fn poisson<A: Number, B: Number>(x: A, λ: B) -> FnNumber2<A, B> {
    FnNumber2("POISSON", x, λ)
}

/// returns the probability or the cumulative distribution function for the 
/// Poisson distribution
///
/// __Syntax__: 
/// ```ods
///     POISSON( X: Integer; λ: Number; Cumulative: Logical )
/// ```
///
/// __Constraints__:
/// λ > 0, X ≥ 0
///
/// __Semantics__:
/// If Cumulative is FALSE, POISSON returns the value
/// 
/// If Cumulative is TRUE, POISSON returns the value
#[inline]
pub fn poisson_<A: Number, B: Number, C: Logical>(x: A, λ: B, cumulative: C) -> FnNumber3<A, B, C> {
    FnNumber3("POISSON", x, λ, cumulative)
}

/// Returns the probability that a discrete random variable lies between two 
/// limits.
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
/// __See also__: "COUNT", 
#[inline]
pub fn prob<A: Array, B: Array, C: Number>(data: A, probability: B, start: C) -> FnNumber3<A, B, C> {
    FnNumber3("PROB", data, probability, start)
}

/// Returns the probability that a discrete random variable lies between two 
/// limits.
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
/// __See also__: "COUNT", 
#[inline]
pub fn prob_<A: Array, B: Array, C: Number, D: Number>(data: A, probability: B, start: C, end: D) -> FnNumber4<A, B, C, D> {
    FnNumber4("PROB", data, probability, start, end)
}

/// Returns a quartile of a set of data points.
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
/// __See also__: "COUNT", "MAX", "MEDIAN", "MIN", "PERCENTILE", "PERCENTRANK", "RANK", 
#[inline]
pub fn quartile<A: Sequence, B: Number>(data: A, quart: B) -> FnNumber2<A, B> {
    FnNumber2("QUARTILE", data, quart)
}

/// Returns the rank of a number in a list of numbers.
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
#[inline]
pub fn rank<A: Number, B: Sequence>(value: A, data: B) -> FnNumber2<A, B> {
    FnNumber2("RANK", value, data)
}

/// Returns the rank of a number in a list of numbers.
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
#[inline]
pub fn rank_<A: Number, B: Sequence, C: Number>(value: A, data: B, order: C) -> FnNumber3<A, B, C> {
    FnNumber3("RANK", value, data, order)
}

/// Returns the square of the Pearson product moment correlation coefficient.
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
/// __See also__: "COLUMNS", "ROWS", "PEARSON", 
#[inline]
pub fn rsq<A: Array, B: Array>(array_y: A, array_x: B) -> FnNumber2<A, B> {
    FnNumber2("RSQ", array_y, array_x)
}

/// Estimates the skewness of a distribution using a sample set of numbers.
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
/// __See also__: "SKEWP", 
#[inline]
pub fn skew<A: Sequence>(sample: A) -> FnNumber1<A> {
    FnNumber1("SKEW", sample)
}

/// Calculates the skewness of a distribution using the population of a random 
/// variable.
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
/// and the standard deviation σ,the skewness becomes
///
/// __See also__: "SKEW", 
#[inline]
pub fn skewp<A: Sequence>(population: A) -> FnNumber1<A> {
    FnNumber1("SKEWP", population)
}

/// Calculates the slope of the linear regression line.
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
/// __See also__: "COLUMNS", "ROWS", "INTERCEPT", "STEYX", 
#[inline]
pub fn slope<A: Array, B: Array>(y: A, x: B) -> FnNumber2<A, B> {
    FnNumber2("SLOPE", y, x)
}

/// Finds the nth smallest value in a list.
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
/// __See also__: "INT", "LARGE", "ROUNDDOWN", 
#[inline]
pub fn small<A: Sequence, B: NumberOrArray>(list: A, n: B) -> FnArray2<A, B> {
    FnArray2("SMALL", list, n)
}

/// Calculates a normalized value of a random variable.
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
/// __See also__: "GAUSS", 
#[inline]
pub fn standardize<A: Number, B: Number, C: Number>(value: A, mean: B, sigma: C) -> FnNumber3<A, B, C> {
    FnNumber3("STANDARDIZE", value, mean, sigma)
}

/// Compute the sample standard deviation of a set of numbers.
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
/// Note that s is not the same as the standard deviation of the set, σ, which 
/// uses n rather than n − 1.
///
/// __See also__: "STDEVP", "AVERAGE", 
#[inline]
pub fn stdev<A: Sequence>(n: A) -> FnNumber1<A> {
    FnNumber1("STDEV", n)
}

/// Calculate the standard deviation using a sample set of values, including 
/// values of type Text and Logical.
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
/// __See also__: "COUNTA", "STDEV", 
#[inline]
pub fn stdeva<A: Sequence>(sample: A) -> FnNumber1<A> {
    FnNumber1("STDEVA", sample)
}

/// Calculates the standard deviation using the population of a random 
/// variable, including values of type Text and Logical.
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
/// Computes the standard deviation of the set σ, where
/// 
/// Note that σ is not the same as the sample standard deviation, s, which 
/// uses n − 1 rather than n.
///
/// __See also__: "COUNT", "STDEV", "AVERAGE", 
#[inline]
pub fn stdevp<A: Sequence>(n: A) -> FnNumber1<A> {
    FnNumber1("STDEVP", n)
}

/// Calculates the standard deviation using the population of a random 
/// variable, including values of type Text and Logical.
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
/// __See also__: "COUNTA", "STDEVP", 
#[inline]
pub fn stdevpa<A: Sequence>(sample: A) -> FnNumber1<A> {
    FnNumber1("STDEVPA", sample)
}

/// Calculates the standard error of the predicted y value for each x in the 
/// regression.
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
/// __See also__: "COLUMNS", "ROWS", "INTERCEPT", "SLOPE", 
#[inline]
pub fn steyx<A: Array, B: Array>(measured_y: A, x: B) -> FnNumber2<A, B> {
    FnNumber2("STEYX", measured_y, x)
}

/// Returns the area to the tail or tails of the probability density function 
/// of the t-distribution.
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
/// __See also__: "GAMMA", "BETADIST", "BINOMDIST", "CHISQDIST", "EXPONDIST", "FDIST", "GAMMADIST", "GAUSS", "HYPGEOMDIST", "LOGNORMDIST", "NEGBINOMDIST", "NORMDIST", "POISSON", "WEIBULL", 
#[inline]
pub fn legacy_tdist<A: Number, B: Number, C: Number>(x: A, df: B, tails: C) -> FnNumber3<A, B, C> {
    FnNumber3("LEGACY.TDIST", x, df, tails)
}

/// Calculates the inverse of the two-tailed t-distribution.
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
/// __See also__: "LEGACY.TDIST", 
#[inline]
pub fn tinv<A: Number, B: Number>(probability: A, degree_of_freedom: B) -> FnNumber2<A, B> {
    FnNumber2("TINV", probability, degree_of_freedom)
}

/// Returns predicted values based on a simple or multiple linear regression.
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
/// __See also__: "COLUMNS", "ROWS", "INTERCEPT", "LINEST", "SLOPE", "STEYX", 
#[inline]
pub fn trend<A: Array>(known_y: A) -> FnArray1<A> {
    FnArray1("TREND", known_y)
}

/// Returns predicted values based on a simple or multiple linear regression.
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
/// __See also__: "COLUMNS", "ROWS", "INTERCEPT", "LINEST", "SLOPE", "STEYX", 
#[inline]
pub fn trend_<A: Array, B: Array>(known_y: A, known_x: B) -> FnArray2<A, B> {
    FnArray2("TREND", known_y, known_x)
}

/// Returns predicted values based on a simple or multiple linear regression.
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
/// __See also__: "COLUMNS", "ROWS", "INTERCEPT", "LINEST", "SLOPE", "STEYX", 
#[inline]
pub fn trend__<A: Array, B: Array, C: Array>(known_y: A, known_x: B, new_x: C) -> FnArray3<A, B, C> {
    FnArray3("TREND", known_y, known_x, new_x)
}

/// Returns predicted values based on a simple or multiple linear regression.
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
/// __See also__: "COLUMNS", "ROWS", "INTERCEPT", "LINEST", "SLOPE", "STEYX", 
#[inline]
pub fn trend___<A: Array, B: Array, C: Array, D: Logical>(known_y: A, known_x: B, new_x: C, const_: D) -> FnArray4<A, B, C, D> {
    FnArray4("TREND", known_y, known_x, new_x, const_)
}

/// Returns the mean of a data set, ignoring a proportion of high and low 
/// values.
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
/// __See also__: "AVERAGE", "GEOMEAN", "HARMEAN", 
#[inline]
pub fn trimmean<A: Sequence, B: Number>(data_set: A, cut_off_fraction: B) -> FnNumber2<A, B> {
    FnNumber2("TRIMMEAN", data_set, cut_off_fraction)
}

/// Calculates the p-value of a 2-sample t-test.
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
/// __See also__: "COLUMNS", "COUNT", "ROWS", "FTEST", "LEGACY.TDIST", "ZTEST", 
#[inline]
pub fn ttest<A: Array, B: Array, C: Number, D: Number>(x: A, y: B, tails: C, type_: D) -> FnNumber4<A, B, C, D> {
    FnNumber4("TTEST", x, y, tails, type_)
}

/// Compute the sample variance of a set of numbers.
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
/// Note that s2 is not the same as the variance of the set, σ2, which uses n 
/// rather than n − 1.
///
/// __See also__: "VARP", "STDEV", "AVERAGE", 
#[inline]
pub fn var<A: Sequence>(n: A) -> FnNumber1<A> {
    FnNumber1("VAR", n)
}

/// Estimates the variance using a sample set of values, including values of 
/// type Text and Logical.
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
/// __See also__: "VAR", 
#[inline]
pub fn vara<A: Sequence>(sample: A) -> FnNumber1<A> {
    FnNumber1("VARA", sample)
}

/// Compute the variance of the set for a set of numbers.
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
/// Computes the variance of the set σ2, where
/// 
/// Note that σ2 is not the same as the sample variance, s2, which uses n − 
/// 1 rather than n.
/// 
/// If only one number is provided, returns 0.
///
/// __See also__: "COUNT", "VAR", "STDEVP", "AVERAGE", 
#[inline]
pub fn varp<A: Sequence>(n: A) -> FnNumber1<A> {
    FnNumber1("VARP", n)
}

/// Calculates the variance using the population of the distribution, including 
/// values of type Text and Logical.
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
/// __See also__: "COUNTA", "VARP", 
#[inline]
pub fn varpa<A: Sequence>(sample: A) -> FnNumber1<A> {
    FnNumber1("VARPA", sample)
}

/// Calculates the Weibull distribution.
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
/// __See also__: "BETADIST", "BINOMDIST", "CHISQDIST", "EXPONDIST", "FDIST", "GAMMADIST", "GAUSS", "HYPGEOMDIST", "LOGNORMDIST", "NEGBINOMDIST", "NORMDIST", "POISSON", "LEGACY.TDIST", 
#[inline]
pub fn weibull<A: Number, B: Number, C: Number, D: Logical>(value: A, shape: B, scale: C, cumulative: D) -> FnNumber4<A, B, C, D> {
    FnNumber4("WEIBULL", value, shape, scale, cumulative)
}

/// Calculates the probability of observing a sample mean as large or larger 
/// than the mean of the given sample for samples drawn from a normal 
/// distribution.
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
/// Sigma. If Sigma is omitted, it is estimated from Sample, using STDEV. With 
/// Sample being the mean of Sample and
/// 
/// ZTEST returns
///
/// __See also__: "FTEST", "TTEST", 
#[inline]
pub fn ztest<A: Sequence, B: Number>(sample: A, mean: B) -> FnNumber2<A, B> {
    FnNumber2("ZTEST", sample, mean)
}

/// Calculates the probability of observing a sample mean as large or larger 
/// than the mean of the given sample for samples drawn from a normal 
/// distribution.
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
/// Sigma. If Sigma is omitted, it is estimated from Sample, using STDEV. With 
/// Sample being the mean of Sample and
/// 
/// ZTEST returns
///
/// __See also__: "FTEST", "TTEST", 
#[inline]
pub fn ztest_<A: Sequence, B: Number, C: Number>(sample: A, mean: B, sigma: C) -> FnNumber3<A, B, C> {
    FnNumber3("ZTEST", sample, mean, sigma)
}
