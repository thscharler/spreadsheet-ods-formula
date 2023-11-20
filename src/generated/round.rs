
use crate::*;
#[allow(unused_imports)]
use crate::round::*;

/// Round a number N up to the nearest multiple of the second parameter, 
/// significance.
/// Syntax: CEILING( N Number; )
///
/// Constraints:
/// Both N and Significance shall be numeric and have the same sign if not 0.
///
/// Semantics:
/// Rounds a number up to a multiple of the second number. If Significance is 
/// omitted or an empty parameter (two consecutive ;; semicolons) it is assumed 
/// to be -1 if N is negative and +1 if N is non-negative, making the function 
/// act like the normal mathematical ceiling function if Mode is not given or 
/// zero. If Mode is given and not equal to zero, the absolute value of N is 
/// rounded away from zero to a multiple of the absolute value of Significance 
/// and then the sign applied . If Mode is omitted or zero, rounding is toward 
/// positive infinity; the number is rounded to the smallest multiple of 
/// significance that is equal-to or greater than N. If any of the two 
/// parameters N or Significance is zero, the result is zero.
///
/// Note:
/// Many application user interfaces have a CEILING function with only two 
/// parameters, and somewhat different semantics than given here (e.g., they 
/// operate as if there was a non-zero Mode value). These CEILING functions are 
/// inconsistent with the standard mathematical definition of CEILING.
///
/// See also: "FLOOR", "INT", 
#[inline]
pub fn ceiling<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("CEILING", n)
}

/// Round a number N up to the nearest multiple of the second parameter, 
/// significance.
/// Syntax: CEILING( N Number;[; Significance Number] )
///
/// Constraints:
/// Both N and Significance shall be numeric and have the same sign if not 0.
///
/// Semantics:
/// Rounds a number up to a multiple of the second number. If Significance is 
/// omitted or an empty parameter (two consecutive ;; semicolons) it is assumed 
/// to be -1 if N is negative and +1 if N is non-negative, making the function 
/// act like the normal mathematical ceiling function if Mode is not given or 
/// zero. If Mode is given and not equal to zero, the absolute value of N is 
/// rounded away from zero to a multiple of the absolute value of Significance 
/// and then the sign applied . If Mode is omitted or zero, rounding is toward 
/// positive infinity; the number is rounded to the smallest multiple of 
/// significance that is equal-to or greater than N. If any of the two 
/// parameters N or Significance is zero, the result is zero.
///
/// Note:
/// Many application user interfaces have a CEILING function with only two 
/// parameters, and somewhat different semantics than given here (e.g., they 
/// operate as if there was a non-zero Mode value). These CEILING functions are 
/// inconsistent with the standard mathematical definition of CEILING.
///
/// See also: "FLOOR", "INT", 
#[inline]
pub fn ceiling_<A: Number, B: Number>(n: A, significance: B) -> FnNumber2<A, B> {
    FnNumber2("CEILING", n, significance)
}

/// Round a number N up to the nearest multiple of the second parameter, 
/// significance.
/// Syntax: CEILING( N Number;[; Significance Number][; Mode Number] )
///
/// Constraints:
/// Both N and Significance shall be numeric and have the same sign if not 0.
///
/// Semantics:
/// Rounds a number up to a multiple of the second number. If Significance is 
/// omitted or an empty parameter (two consecutive ;; semicolons) it is assumed 
/// to be -1 if N is negative and +1 if N is non-negative, making the function 
/// act like the normal mathematical ceiling function if Mode is not given or 
/// zero. If Mode is given and not equal to zero, the absolute value of N is 
/// rounded away from zero to a multiple of the absolute value of Significance 
/// and then the sign applied . If Mode is omitted or zero, rounding is toward 
/// positive infinity; the number is rounded to the smallest multiple of 
/// significance that is equal-to or greater than N. If any of the two 
/// parameters N or Significance is zero, the result is zero.
///
/// Note:
/// Many application user interfaces have a CEILING function with only two 
/// parameters, and somewhat different semantics than given here (e.g., they 
/// operate as if there was a non-zero Mode value). These CEILING functions are 
/// inconsistent with the standard mathematical definition of CEILING.
///
/// See also: "FLOOR", "INT", 
#[inline]
pub fn ceiling__<A: Number, B: Number>(n: A, significance: B, mode: RoundingMode) -> FnNumber3<A, B, RoundingMode> {
    FnNumber3("CEILING", n, significance, mode)
}

/// Rounds a number down to the nearest integer.
/// Syntax: INT( N Number; )
///
/// Constraints:
/// None
///
/// Semantics:
/// Returns the nearest integer whose value is less than or equal to N. 
/// Rounding is towards negative infinity.
///
/// See also: "ROUND", "TRUNC", 
#[inline]
pub fn int<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("INT", n)
}

/// Round a number N down to the nearest multiple of the second parameter, 
/// significance.
/// Syntax: FLOOR( N Number; )
///
/// Constraints:
/// Both N and Significance shall be numeric and have the same sign.
///
/// Semantics:
/// Rounds a number down to a multiple of the second number. If Significance is 
/// omitted or an empty parameter (two consecutive ;; semicolons) it is assumed 
/// to be -1 if N is negative and +1 if N is non-negative, making the function 
/// act like the normal mathematical floor function if Mode is not given or 
/// zero. If Mode is given and not equal to zero, the absolute value of N is 
/// rounded toward zero to a multiple of the absolute value of Significance and 
/// then the sign applied . Otherwise, it rounds toward negative infinity, and 
/// the result is the largest multiple of Significance that is less than or 
/// equal to N. If any of the two parameters N or Significance is zero, the 
/// result is zero.
///
/// Note:
/// Many application user interfaces have a FLOOR function with only two 
/// parameters, and somewhat different semantics than given here (e.g., they 
/// operate as if there was a non-zero Mode value). These FLOOR functions are 
/// inconsistent with the standard mathematical definition of FLOOR.
///
/// See also: "CEILING", "INT", 
#[inline]
pub fn floor<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("FLOOR", n)
}

/// Round a number N down to the nearest multiple of the second parameter, 
/// significance.
/// Syntax: FLOOR( N Number;[; Significance Number] )
///
/// Constraints:
/// Both N and Significance shall be numeric and have the same sign.
///
/// Semantics:
/// Rounds a number down to a multiple of the second number. If Significance is 
/// omitted or an empty parameter (two consecutive ;; semicolons) it is assumed 
/// to be -1 if N is negative and +1 if N is non-negative, making the function 
/// act like the normal mathematical floor function if Mode is not given or 
/// zero. If Mode is given and not equal to zero, the absolute value of N is 
/// rounded toward zero to a multiple of the absolute value of Significance and 
/// then the sign applied . Otherwise, it rounds toward negative infinity, and 
/// the result is the largest multiple of Significance that is less than or 
/// equal to N. If any of the two parameters N or Significance is zero, the 
/// result is zero.
///
/// Note:
/// Many application user interfaces have a FLOOR function with only two 
/// parameters, and somewhat different semantics than given here (e.g., they 
/// operate as if there was a non-zero Mode value). These FLOOR functions are 
/// inconsistent with the standard mathematical definition of FLOOR.
///
/// See also: "CEILING", "INT", 
#[inline]
pub fn floor_<A: Number, B: Number>(n: A, significance: B) -> FnNumber2<A, B> {
    FnNumber2("FLOOR", n, significance)
}

/// Round a number N down to the nearest multiple of the second parameter, 
/// significance.
/// Syntax: FLOOR( N Number;[; Significance Number][; Mode Number] )
///
/// Constraints:
/// Both N and Significance shall be numeric and have the same sign.
///
/// Semantics:
/// Rounds a number down to a multiple of the second number. If Significance is 
/// omitted or an empty parameter (two consecutive ;; semicolons) it is assumed 
/// to be -1 if N is negative and +1 if N is non-negative, making the function 
/// act like the normal mathematical floor function if Mode is not given or 
/// zero. If Mode is given and not equal to zero, the absolute value of N is 
/// rounded toward zero to a multiple of the absolute value of Significance and 
/// then the sign applied . Otherwise, it rounds toward negative infinity, and 
/// the result is the largest multiple of Significance that is less than or 
/// equal to N. If any of the two parameters N or Significance is zero, the 
/// result is zero.
///
/// Note:
/// Many application user interfaces have a FLOOR function with only two 
/// parameters, and somewhat different semantics than given here (e.g., they 
/// operate as if there was a non-zero Mode value). These FLOOR functions are 
/// inconsistent with the standard mathematical definition of FLOOR.
///
/// See also: "CEILING", "INT", 
#[inline]
pub fn floor__<A: Number, B: Number>(n: A, significance: B, mode: RoundingMode) -> FnNumber3<A, B, RoundingMode> {
    FnNumber3("FLOOR", n, significance, mode)
}

/// Rounds the number to given multiple.
/// Syntax: MROUND( A Number;; B Number; )
///
/// Constraints:
/// None
///
/// Semantics:
/// Returns the number X, for which the following holds: X/B = INT(X / B) (B 
/// divides X), and for any other Y with the same property, ABS(Y – A) ≥ 
/// ABS(X - A). In case that two such X exist, the greater one is the result. 
/// In less formal language, this function rounds the number A to multiples of 
/// B.
///
/// See also: "ABS", "INT", "ROUND", 
#[inline]
pub fn mround<A: Number, B: Number>(a: A, b: B) -> FnNumber2<A, B> {
    FnNumber2("MROUND", a, b)
}

/// Rounds the value X to the nearest multiple of the power of 10 specified by 
/// Digits.
/// Syntax: ROUND( X Number; )
///
/// Constraints:
/// None
///
/// Semantics:
/// Round number X to the precision specified by Digits. The number X is 
/// rounded to the nearest power of 10 given by 10 −Digits. If Digits is 
/// zero, or absent, round to the nearest decimal integer. If Digits is 
/// non-negative, round to the specified number of decimal places. If Digits is 
/// negative, round to the left of the decimal point by -Digits places. If X is 
/// halfway between the two nearest values, the result shall round away from 
/// zero. Note that if X is a Number, and Digits ≤ 0, the results will always 
/// be an integer (without a fractional component).
///
/// See also: "TRUNC", "INT", 
#[inline]
pub fn round<A: Number>(x: A) -> FnNumber1<A> {
    FnNumber1("ROUND", x)
}

/// Rounds the value X to the nearest multiple of the power of 10 specified by 
/// Digits.
/// Syntax: ROUND( X Number;[; Digits Number] )
///
/// Constraints:
/// None
///
/// Semantics:
/// Round number X to the precision specified by Digits. The number X is 
/// rounded to the nearest power of 10 given by 10 −Digits. If Digits is 
/// zero, or absent, round to the nearest decimal integer. If Digits is 
/// non-negative, round to the specified number of decimal places. If Digits is 
/// negative, round to the left of the decimal point by -Digits places. If X is 
/// halfway between the two nearest values, the result shall round away from 
/// zero. Note that if X is a Number, and Digits ≤ 0, the results will always 
/// be an integer (without a fractional component).
///
/// See also: "TRUNC", "INT", 
#[inline]
pub fn round_<A: Number, B: Number>(x: A, digits: B) -> FnNumber2<A, B> {
    FnNumber2("ROUND", x, digits)
}

/// Rounds the value X towards zero to the number of digits specified by 
/// Digits.
/// Syntax: ROUNDDOWN( X Number; )
///
/// Constraints:
/// None
///
/// Semantics:
/// Round X towards zero, to the precision specified by Digits. The number 
/// returned is a multiple of 10−Digits. If Digits is zero, or absent, round 
/// to the largest decimal integer whose absolute value is smaller or equal to 
/// the absolute value of X. If Digits is positive, round towards zero to the 
/// specified number of decimal places. If Digits is negative, round towards 
/// zero to the left of the decimal point by -Digits places.
///
/// See also: "TRUNC", "INT", "ROUND", "ROUNDUP", 
#[inline]
pub fn rounddown<A: Number>(x: A) -> FnNumber1<A> {
    FnNumber1("ROUNDDOWN", x)
}

/// Rounds the value X towards zero to the number of digits specified by 
/// Digits.
/// Syntax: ROUNDDOWN( X Number;[; Digits Integer] )
///
/// Constraints:
/// None
///
/// Semantics:
/// Round X towards zero, to the precision specified by Digits. The number 
/// returned is a multiple of 10−Digits. If Digits is zero, or absent, round 
/// to the largest decimal integer whose absolute value is smaller or equal to 
/// the absolute value of X. If Digits is positive, round towards zero to the 
/// specified number of decimal places. If Digits is negative, round towards 
/// zero to the left of the decimal point by -Digits places.
///
/// See also: "TRUNC", "INT", "ROUND", "ROUNDUP", 
#[inline]
pub fn rounddown_<A: Number, B: Number>(x: A, digits: B) -> FnNumber2<A, B> {
    FnNumber2("ROUNDDOWN", x, digits)
}

/// Rounds the value X away from zero to the number of digits specified by 
/// Digits
/// Syntax: ROUNDUP( X Number; )
///
/// Constraints:
/// None
///
/// Semantics:
/// Round X away from zero, to the precision specified by Digits. The number 
/// returned is a multiple of 10−Digits. If Digits is zero, or absent, round 
/// to the smallest decimal integer whose absolute value is larger or equal to 
/// the absolute value of X. If Digits is positive, round away from zero to the 
/// specified number of decimal places. If Digits is negative, round away from 
/// zero to the left of the decimal point by -Digits places.
///
/// See also: "TRUNC", "INT", "ROUND", "ROUNDDOWN", 
#[inline]
pub fn roundup<A: Number>(x: A) -> FnNumber1<A> {
    FnNumber1("ROUNDUP", x)
}

/// Rounds the value X away from zero to the number of digits specified by 
/// Digits
/// Syntax: ROUNDUP( X Number;[; Digits Integer] )
///
/// Constraints:
/// None
///
/// Semantics:
/// Round X away from zero, to the precision specified by Digits. The number 
/// returned is a multiple of 10−Digits. If Digits is zero, or absent, round 
/// to the smallest decimal integer whose absolute value is larger or equal to 
/// the absolute value of X. If Digits is positive, round away from zero to the 
/// specified number of decimal places. If Digits is negative, round away from 
/// zero to the left of the decimal point by -Digits places.
///
/// See also: "TRUNC", "INT", "ROUND", "ROUNDDOWN", 
#[inline]
pub fn roundup_<A: Number, B: Number>(x: A, digits: B) -> FnNumber2<A, B> {
    FnNumber2("ROUNDUP", x, digits)
}

/// Truncate a number to a specified number of digits.
/// Syntax: TRUNC( A Number;; B Integer; )
///
/// Constraints:
/// None
///
/// Semantics:
/// Truncate number A to the number of digits specified by B. If B is zero, or 
/// absent, truncate to an integer. If B is positive, truncate to the specified 
/// number of decimal places. If B is negative, truncate to the left of the 
/// decimal point.
///
/// See also: "ROUND", "INT", 
#[inline]
pub fn trunc<A: Number, B: Number>(a: A, b: B) -> FnNumber2<A, B> {
    FnNumber2("TRUNC", a, b)
}
