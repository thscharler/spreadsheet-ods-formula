//! 
//! The logical functions are: TRUE() and FALSE(); the functions that compute 
//! Logical values NOT(), AND(), and OR(); and the conditional function IF(). 
//! The OpenDocument specification mentions "logical operators"; these are 
//! another name for the logical functions.
//! 
//! Note that because of Error values, any logical function that accepts 
//! parameters can actually produce TRUE, FALSE, or an Error value instead of 
//! TRUE or FALSE.
//! 
//! These are not bitwise operations, e.g., AND(12;10) produces TRUE, not 8. 
//! See the bit operation functions for bitwise operations.

use crate::*;
#[allow(unused_imports)]
use crate::logic::*;

/// Compute logical AND of all parameters.
///
/// __Syntax__: 
/// ```ods
///     AND({ L Logical|NumberSequenceList}+ )
/// ```
///
/// __Constraints__:
/// Shall have 1 or more parameters
///
/// __Semantics__:
/// Computes the logical AND of the parameters. If all parameters are TRUE, 
/// returns TRUE; if any are FALSE, returns FALSE. When given one parameter, 
/// this has the effect of converting that one parameter into a Logical value. 
/// When given zero parameters, evaluators may return a Logical value or an 
/// Error.
/// 
/// Also in array context a logical AND of all arguments is computed, range or 
/// array parameters are not evaluated as a matrix and no array is returned. 
/// This behavior is consistent with functions like SUM. To compute a logical 
/// AND of arrays per element use the * operator in array context.
///
/// __See also__: "OR", "IF", 
#[inline]
pub fn and<A: Sequence>(l: A) -> FnLogical1<A> {
    FnLogical1("AND", l)
}

/// Returns constant FALSE.
///
/// __Syntax__: 
/// ```ods
///     FALSE( )
/// ```
///
/// __Constraints__:
/// Shall have 0 parameters
///
/// __Semantics__:
/// Returns logical constant FALSE. This may be a Number or a distinct type.
///
/// __See also__: "TRUE", "IF", 
#[inline]
pub fn false_() -> FnLogical0 {
    FnLogical0("FALSE", )
}

/// Return X unless it is an Error, in which case return an alternative value.
///
/// __Syntax__: 
/// ```ods
///     IFERROR( X Any; Alternative Any )
/// ```
///
/// __Constraints__:
/// None.
///
/// __Semantics__:
/// Computes X. If ISERROR(X) is TRUE, return Alternative, else return X.
///
/// __Note__:
/// This is semantically equivalent to IF(ISERROR(X); Alternative; X), except 
/// that X is only computed once.
///
/// __See also__: "IF", "ISERROR", 
#[inline]
pub fn iferror<A: Any, B: Any>(x: A, alternative: B) -> FnAny2<A, B> {
    FnAny2("IFERROR", x, alternative)
}

/// Return X unless it is #N/A, in which case return an alternative value.
///
/// __Syntax__: 
/// ```ods
///     IFNA( X Any; Alternative Any )
/// ```
///
/// __Constraints__:
/// None.
///
/// __Semantics__:
/// Computes X. If ISNA(X) is TRUE, return Alternative, else return X.
///
/// __Note__:
/// This is semantically equivalent to IF(ISNA(X); Alternative; X), except that 
/// X is only computed once.
///
/// __See also__: "IF", "ISNA", 
#[inline]
pub fn ifna<A: Any, B: Any>(x: A, alternative: B) -> FnAny2<A, B> {
    FnAny2("IFNA", x, alternative)
}

/// Compute logical NOT.
///
/// __Syntax__: 
/// ```ods
///     NOT( L Logical )
/// ```
///
/// __Constraints__:
/// Shall have 1 parameter.
///
/// __Semantics__:
/// Computes the logical NOT. If given TRUE, returns FALSE; if given FALSE, 
/// returns TRUE.
///
/// __See also__: "AND", "IF", 
#[inline]
pub fn not<A: Logical>(l: A) -> FnLogical1<A> {
    FnLogical1("NOT", l)
}

/// Compute logical OR of all parameters.
///
/// __Syntax__: 
/// ```ods
///     OR({ L Logical|NumberSequenceList}+ )
/// ```
///
/// __Constraints__:
/// Shall have 1 or more parameters
///
/// __Semantics__:
/// Computes the logical OR of the parameters. If all parameters are FALSE, it 
/// shall return FALSE; if any are TRUE, it shall returns TRUE. When given one 
/// parameter, this has the effect of converting that one parameter into a 
/// Logical value. When given zero parameters, evaluators may return a Logical 
/// value or an Error.
/// 
/// Also in array context a logical OR of all arguments is computed, range or 
/// array parameters are not evaluated as a matrix and no array is returned. 
/// This behavior is consistent with functions like SUM. To compute a logical 
/// OR of arrays per element use the + operator in array context.
///
/// __See also__: "AND", "IF", 
#[inline]
pub fn or<A: Sequence>(l: A) -> FnLogical1<A> {
    FnLogical1("OR", l)
}

/// Returns constant TRUE
///
/// __Syntax__: 
/// ```ods
///     TRUE( )
/// ```
///
/// __Constraints__:
/// Shall have 0 parameters
///
/// __Semantics__:
/// Returns logical constant TRUE. The result of this function may but need not 
/// be equal to 1 when compared using “=”. It always has the value of 1 if 
/// used in a context requiring Number (because of the automatic conversions), 
/// so if ISNUMBER(TRUE()) is TRUE, then it shall have the value 1.
///
/// __See also__: "FALSE", "IF", "ISNUMBER", 
#[inline]
pub fn true_() -> FnLogical0 {
    FnLogical0("TRUE", )
}

/// Compute a logical XOR of all parameters.
///
/// __Syntax__: 
/// ```ods
///     XOR({ L Logical}+ )
/// ```
///
/// __Constraints__:
/// Shall have 1 or more parameters.
///
/// __Semantics__:
/// Computes the logical XOR of the parameters such that the result is an 
/// addition modulo 2. If an even number of parameters is TRUE it returns 
/// FALSE, if an odd number of parameters is TRUE it returns TRUE. When given 
/// one parameter, this has the effect of converting that one parameter into a 
/// Logical value.
///
/// __See also__: "AND", "OR", 
#[inline]
pub fn xor<A: Sequence>(l: A) -> FnLogical1<A> {
    FnLogical1("XOR", l)
}
