//!
//! Basic operators as functions.
//!

use crate::{Number, OpNumber, OpReference, OpText, Reference, Text};

/// Adds two numbers. Also available as postfix add() and as operator +.
#[inline]
pub fn add<'a, A: Number, B: Number>(a: A, b: B) -> OpNumber<A, B> {
    OpNumber(a, "+", b)
}

/// Subtracts two numbers. Also available as postfix sub() and as operator -.
#[inline]
pub fn sub<'a, A: Number, B: Number>(a: A, b: B) -> OpNumber<A, B> {
    OpNumber(a, "-", b)
}

/// Multiplies to numbers. Also available as postfix mul() and as operator *;
#[inline]
pub fn mul<'a, A: Number, B: Number>(a: A, b: B) -> OpNumber<A, B> {
    OpNumber(a, "*", b)
}

/// Divides to numbers. Also available as postfix div() and as operator /.
#[inline]
pub fn div<'a, A: Number, B: Number>(a: A, b: B) -> OpNumber<A, B> {
    OpNumber(a, "/", b)
}

/// Exponential function. Also available as postfix pow() and as operator ^.
#[inline]
pub fn pow<'a, A: Number, B: Number>(a: A, b: B) -> OpNumber<A, B> {
    OpNumber(a, "^", b)
}

/// Negates as number. Also available as prefix operator -.
#[inline]
pub fn neg<'a, A: Number>(a: A) -> OpNumber<(), A> {
    OpNumber((), "-", a)
}

/// percentage. Also available as postfix percent()
#[inline]
pub fn percent<'a, A: Number>(a: A) -> OpNumber<A, ()> {
    OpNumber(a, "%", ())
}

/// concatenates two strings. Also available as postfix concat() and as operator &.
#[inline]
pub fn concat<'a, A: Text, B: Text>(a: A, b: B) -> OpText<A, B> {
    OpText(a, "&", b)
}

#[inline]
pub fn intersect<'a, A: Reference, B: Reference>(a: A, b: B) -> OpReference<A, B> {
    OpReference(a, "!", b)
}

#[inline]
pub fn refcat<'a, A: Reference, B: Reference>(a: A, b: B) -> OpReference<A, B> {
    OpReference(a, "~", b)
}
