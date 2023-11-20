//!
//! Basic operators as functions.
//!

use crate::{Number, OpNumber, OpReference, OpText, Reference, Text};

/// Adds two numbers.
///  
/// __Syntax__:
/// ```ods
///     Number Left + Number Right
/// ```
///
/// __Returns__: Number
///
/// __Constraints__: None
///
/// __Semantics__: Adds numbers together.
///
/// __See also__ postfix add() and as operator +.
#[inline]
pub fn add<A: Number, B: Number>(a: A, b: B) -> OpNumber<A, B> {
    OpNumber(a, "+", b)
}

/// Subtracts two numbers.  
///
/// __Syntax__:
/// ```ods
///     Number Left - Number Right
/// ```
///
/// __Returns__: Number
///
/// __Constraints__: None
///
/// __Semantics__: Subtracts two numbers .
///
/// __See also__ postfix sub() and as operator -.
#[inline]
pub fn sub<A: Number, B: Number>(a: A, b: B) -> OpNumber<A, B> {
    OpNumber(a, "-", b)
}

/// Multiplies to numbers.
///
/// Syntax:
/// ```ods
///     Number Left * Number Right
/// ```
///
/// Returns: Number
///
/// Constraints: None
///
/// Semantics: Multiplies numbers together.
///
/// See also postfix mul() and as operator *;
#[inline]
pub fn mul<A: Number, B: Number>(a: A, b: B) -> OpNumber<A, B> {
    OpNumber(a, "*", b)
}

/// Divides to numbers. Also available as postfix div() and as operator /.
///
/// Syntax:
/// ```ods
///     Number Left / Number Right
/// ```
///
/// Returns: Number
///
/// Constraints: None
///
/// Semantics: Divides numbers. Dividing by zero returns an Error.
///
/// See also postfix div() and as operator /.
#[inline]
pub fn div<A: Number, B: Number>(a: A, b: B) -> OpNumber<A, B> {
    OpNumber(a, "/", b)
}

/// Exponential function.  
///
/// Syntax:
/// ```ods
///     Number Left ^ Number Right
/// ```
///
/// Returns: Number
///
/// Constraints: NOT(AND(Left=0; Right=0)); Evaluators may evaluate expressions where
/// OR(Left != 0; Right != 0) evaluates to a non-Error value.
///
/// Semantics: Returns POWER(Left, Right).
///
/// See also postfix pow() and as operator ^.
#[inline]
pub fn pow<A: Number, B: Number>(a: A, b: B) -> OpNumber<A, B> {
    OpNumber(a, "^", b)
}

/// Negates as number. Also available as prefix operator -.
#[inline]
pub fn neg<A: Number>(a: A) -> OpNumber<(), A> {
    OpNumber((), "-", a)
}

/// percentage. Also available as postfix percent()
#[inline]
pub fn percent<A: Number>(a: A) -> OpNumber<A, ()> {
    OpNumber(a, "%", ())
}

/// Concatenate two strings.
///
/// Syntax:
/// ```ods
///     Text Left & Text Right
/// ```
///
/// Returns: Text
///
/// Constraints: None
///
/// Semantics: Concatenates two text (string) values.
///
/// Note: The infix operator “&” is equivalent to CONCATENATE(Left,Right).
///
/// See also postfix concat() and as operator &.
#[inline]
pub fn concat<A: Text, B: Text>(a: A, b: B) -> OpText<A, B> {
    OpText(a, "&", b)
}

/// Compute the intersection of two references
///
/// __Syntax__:
/// ```ods
///     Reference Left ! Reference Right
/// ```
///
/// __Returns__: Reference
///
/// __Constraints__: None
///
/// __Semantics__: Takes two references and computes the intersection - a reference to the intersection
/// of cells in both Left and Right. If there are no cells in common, returns an Error.
/// If Left or Right are not of type Reference or ReferenceList, an Error shall be returned.
/// If Left and/or Right are reference lists (result of infix operator reference concatenation), the
/// intersection is computed for each combination of Left and Right, producing a reference list of
/// intersections.
///
/// __Note 1__: For example (a,b,c,d denoting one reference each):
/// (a~b)!(c~d) will compute (a!c)~(a!d)~(b!c)~(b!d)
///
/// If for a resulting intersection there are no cells in common, the element is ignored and omitted
/// from the result list. If for all intersections there are no cells in common and the result list is empty,
/// an Error is returned.
///
/// __Note 2__: Intersection is notated as "!" in OpenFormula format, but as a space character in some
/// user interfaces.
///
/// __See also__ Infix Operator Reference Union 6.4.13
#[inline]
pub fn intersect<A: Reference, B: Reference>(a: A, b: B) -> OpReference<A, B> {
    OpReference(a, "!", b)
}

/// Concatenate two references
///
/// Syntax:
/// ```ods
///     Reference Left ~ Reference Right
/// ```
///
/// Returns: ReferenceList
///
/// Constraints: None
///
/// Semantics: Takes two references and computes the "cell union", which is a concatenation of the
/// reference Left followed by the reference Right. This is not the same as a union in set theory;
/// duplicate references to cells are not removed. The resulting reference will have the number of
/// areas, as reported by AREAS, as AREAS(Left)+AREAS(Right).
///
/// Note: Concatenation is notated as "~" in OpenFormula format, but as a comma or “+” in some
/// user interfaces.
///
/// If Left or Right are not of type Reference or ReferenceList, an Error shall be returned.
///
/// See also
#[inline]
pub fn refcat<A: Reference, B: Reference>(a: A, b: B) -> OpReference<A, B> {
    OpReference(a, "~", b)
}
