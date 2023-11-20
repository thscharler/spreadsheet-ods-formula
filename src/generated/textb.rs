//! 
//! Byte-position text functions are like their equivalent ordinary text 
//! functions, but manipulate byte positions rather than a count of the number 
//! of characters. Byte positions are integers that may depend on the specific 
//! text representation used by the implementation. Byte positions are by 
//! definition implementation-dependent and reliance upon them reduces 
//! interoperability.
//! 
//! The pseudotypes ByteLength and BytePosition are Integers, but their exact 
//! meanings and values are not further defined by this specification.

use crate::*;
#[allow(unused_imports)]
use crate::textb::*;

/// Returns the starting position of a given text, using byte positions.
/// Syntax: FINDB( Search Text;; T Text; )
///
/// Semantics:
/// The same as FIND, but using byte positions.
///
/// See also: "FIND", "LEFTB", "RIGHTB", 
#[inline]
pub fn findb<A: Text, B: Text>(search: A, t: B) -> FnNumber2<A, B> {
    FnNumber2("FINDB", search, t)
}

/// Returns the starting position of a given text, using byte positions.
/// Syntax: FINDB( Search Text;; T Text;[; Start BytePosition] )
///
/// Semantics:
/// The same as FIND, but using byte positions.
///
/// See also: "FIND", "LEFTB", "RIGHTB", 
#[inline]
pub fn findb_<A: Text, B: Text, C: Number>(search: A, t: B, start: C) -> FnNumber3<A, B, C> {
    FnNumber3("FINDB", search, t, start)
}

/// Returns a selected number of text characters from the left, using a byte 
/// position.
/// Syntax: LEFTB( T Text; )
///
/// Semantics:
/// As LEFT, but using a byte position.
///
/// See also: "LEFT", "RIGHT", "RIGHTB", 
#[inline]
pub fn leftb<A: Text>(t: A) -> FnText1<A> {
    FnText1("LEFTB", t)
}

/// Returns a selected number of text characters from the left, using a byte 
/// position.
/// Syntax: LEFTB( T Text;[; Length ByteLength] )
///
/// Semantics:
/// As LEFT, but using a byte position.
///
/// See also: "LEFT", "RIGHT", "RIGHTB", 
#[inline]
pub fn leftb_<A: Text, B: Number>(t: A, length: B) -> FnText2<A, B> {
    FnText2("LEFTB", t, length)
}

/// Returns the length of given text in units compatible with byte positions
/// Syntax: LENB( T Text; )
///
/// Constraints:
/// None.
///
/// Semantics:
/// As LEN, but compatible with byte position values.
///
/// See also: "LEN", "LEFTB", "RIGHTB", 
#[inline]
pub fn lenb<A: Text>(t: A) -> FnNumber1<A> {
    FnNumber1("LENB", t)
}

/// Returns extracted text, given an original text, starting position using a 
/// byte position, and length.
/// Syntax: MIDB( T Text;; Start BytePosition;; Length ByteLength; )
///
/// Constraints:
/// Length ≥ 0.
///
/// Semantics:
/// As MID, but using byte positions.
///
/// See also: "MID", "LEFTB", "RIGHTB", "REPLACEB", 
#[inline]
pub fn midb<A: Text, B: Number, C: Number>(t: A, start: B, length: C) -> FnText3<A, B, C> {
    FnText3("MIDB", t, start, length)
}

/// Returns text where an old text is replaced with a new text, using byte 
/// positions.
/// Syntax: REPLACEB( T Text;; Start BytePosition;; Len ByteLength;; New Text; )
///
/// Semantics:
/// As REPLACE, but using byte positions.
///
/// See also: "REPLACE", "LEFTB", "RIGHTB", "MIDB", "SUBSTITUTE", 
#[inline]
pub fn replaceb<A: Text, B: Number, C: Number, D: Text>(t: A, start: B, len: C, new: D) -> FnText4<A, B, C, D> {
    FnText4("REPLACEB", t, start, len, new)
}

/// Returns a selected number of text characters from the right, using byte 
/// position.
/// Syntax: RIGHTB( T Text; )
///
/// Semantics:
/// As RIGHT, but using byte positions.
///
/// See also: "RIGHT", "LEFTB", 
#[inline]
pub fn rightb<A: Text>(t: A) -> FnText1<A> {
    FnText1("RIGHTB", t)
}

/// Returns a selected number of text characters from the right, using byte 
/// position.
/// Syntax: RIGHTB( T Text;[; Length ByteLength] )
///
/// Semantics:
/// As RIGHT, but using byte positions.
///
/// See also: "RIGHT", "LEFTB", 
#[inline]
pub fn rightb_<A: Text, B: Number>(t: A, length: B) -> FnText2<A, B> {
    FnText2("RIGHTB", t, length)
}

/// Returns the starting position of a given text, using byte positions.
/// Syntax: SEARCHB( Search Text;; T Text; )
///
/// Semantics:
/// As SEARCH, but using byte positions.
///
/// See also: "SEARCH", "EXACT", "FIND", "FINDB", 
#[inline]
pub fn searchb<A: Text, B: Text>(search: A, t: B) -> FnNumber2<A, B> {
    FnNumber2("SEARCHB", search, t)
}

/// Returns the starting position of a given text, using byte positions.
/// Syntax: SEARCHB( Search Text;; T Text;[; Start BytePosition] )
///
/// Semantics:
/// As SEARCH, but using byte positions.
///
/// See also: "SEARCH", "EXACT", "FIND", "FINDB", 
#[inline]
pub fn searchb_<A: Text, B: Text, C: Number>(search: A, t: B, start: C) -> FnNumber3<A, B, C> {
    FnNumber3("SEARCHB", search, t, start)
}
