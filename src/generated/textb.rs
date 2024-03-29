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
///
/// [documentfoundation->FINDB](https://wiki.documentfoundation.org/Documentation/Calc_Functions/FINDB)
///
/// __Syntax__: 
/// ```ods
///     FINDB( Search: Text; T: Text )
/// ```
///
/// __Semantics__:
/// The same as FIND, but using byte positions.
///
/// __See also__: [crate::of::find()], [crate::of::leftb()], [crate::of::rightb()], [crate::of::findb_()], 
#[inline]
pub fn findb<A: Text, B: Text>(search: A, t: B) -> FnNumber2<A, B> {
    FnNumber2("FINDB", search, t)
}

/// Returns the starting position of a given text, using byte positions.
///
/// [documentfoundation->FINDB](https://wiki.documentfoundation.org/Documentation/Calc_Functions/FINDB)
///
/// __Syntax__: 
/// ```ods
///     FINDB( Search: Text; T: Text; Start: BytePosition )
/// ```
///
/// __Semantics__:
/// The same as FIND, but using byte positions.
///
/// __See also__: [crate::of::find()], [crate::of::leftb()], [crate::of::rightb()], [crate::of::findb()], 
#[inline]
pub fn findb_<A: Text, B: Text, C: Number>(search: A, t: B, start: C) -> FnNumber3<A, B, C> {
    FnNumber3("FINDB", search, t, start)
}

/// Returns a selected number of text characters from the left, using a byte 
/// position.
///
/// [documentfoundation->LEFTB](https://wiki.documentfoundation.org/Documentation/Calc_Functions/LEFTB)
///
/// __Syntax__: 
/// ```ods
///     LEFTB( T: Text )
/// ```
///
/// __Semantics__:
/// As LEFT, but using a byte position.
///
/// __See also__: [crate::of::left()], [crate::of::right()], [crate::of::rightb()], [crate::of::leftb_()], 
#[inline]
pub fn leftb<A: Text>(t: A) -> FnText1<A> {
    FnText1("LEFTB", t)
}

/// Returns a selected number of text characters from the left, using a byte 
/// position.
///
/// [documentfoundation->LEFTB](https://wiki.documentfoundation.org/Documentation/Calc_Functions/LEFTB)
///
/// __Syntax__: 
/// ```ods
///     LEFTB( T: Text; Length: ByteLength )
/// ```
///
/// __Semantics__:
/// As LEFT, but using a byte position.
///
/// __See also__: [crate::of::left()], [crate::of::right()], [crate::of::rightb()], [crate::of::leftb()], 
#[inline]
pub fn leftb_<A: Text, B: Number>(t: A, length: B) -> FnText2<A, B> {
    FnText2("LEFTB", t, length)
}

/// Returns the length of given text in units compatible with byte positions
///
/// [documentfoundation->LENB](https://wiki.documentfoundation.org/Documentation/Calc_Functions/LENB)
///
/// __Syntax__: 
/// ```ods
///     LENB( T: Text )
/// ```
///
/// __Constraints__:
/// None.
///
/// __Semantics__:
/// As LEN, but compatible with byte position values.
///
/// __See also__: [crate::of::len()], [crate::of::leftb()], [crate::of::rightb()], 
#[inline]
pub fn lenb<A: Text>(t: A) -> FnNumber1<A> {
    FnNumber1("LENB", t)
}

/// Returns extracted text, given an original text, starting position using a 
/// byte position, and length.
///
/// [documentfoundation->MIDB](https://wiki.documentfoundation.org/Documentation/Calc_Functions/MIDB)
///
/// __Syntax__: 
/// ```ods
///     MIDB( T: Text; Start: BytePosition; Length: ByteLength )
/// ```
///
/// __Constraints__:
/// Length ≥ 0.
///
/// __Semantics__:
/// As MID, but using byte positions.
///
/// __See also__: [crate::of::mid()], [crate::of::leftb()], [crate::of::rightb()], [crate::of::replaceb()], 
#[inline]
pub fn midb<A: Text, B: Number, C: Number>(t: A, start: B, length: C) -> FnText3<A, B, C> {
    FnText3("MIDB", t, start, length)
}

/// Returns text where an old text is replaced with a new text, using byte 
/// positions.
///
/// [documentfoundation->REPLACEB](https://wiki.documentfoundation.org/Documentation/Calc_Functions/REPLACEB)
///
/// __Syntax__: 
/// ```ods
///     REPLACEB( T: Text; Start: BytePosition; Len: ByteLength; New: Text )
/// ```
///
/// __Semantics__:
/// As REPLACE, but using byte positions.
///
/// __See also__: [crate::of::replace()], [crate::of::leftb()], [crate::of::rightb()], [crate::of::midb()], [crate::of::substitute()], 
#[inline]
pub fn replaceb<A: Text, B: Number, C: Number, D: Text>(t: A, start: B, len: C, new: D) -> FnText4<A, B, C, D> {
    FnText4("REPLACEB", t, start, len, new)
}

/// Returns a selected number of text characters from the right, using byte 
/// position.
///
/// [documentfoundation->RIGHTB](https://wiki.documentfoundation.org/Documentation/Calc_Functions/RIGHTB)
///
/// __Syntax__: 
/// ```ods
///     RIGHTB( T: Text )
/// ```
///
/// __Semantics__:
/// As RIGHT, but using byte positions.
///
/// __See also__: [crate::of::right()], [crate::of::leftb()], [crate::of::rightb_()], 
#[inline]
pub fn rightb<A: Text>(t: A) -> FnText1<A> {
    FnText1("RIGHTB", t)
}

/// Returns a selected number of text characters from the right, using byte 
/// position.
///
/// [documentfoundation->RIGHTB](https://wiki.documentfoundation.org/Documentation/Calc_Functions/RIGHTB)
///
/// __Syntax__: 
/// ```ods
///     RIGHTB( T: Text; Length: ByteLength )
/// ```
///
/// __Semantics__:
/// As RIGHT, but using byte positions.
///
/// __See also__: [crate::of::right()], [crate::of::leftb()], [crate::of::rightb()], 
#[inline]
pub fn rightb_<A: Text, B: Number>(t: A, length: B) -> FnText2<A, B> {
    FnText2("RIGHTB", t, length)
}

/// Returns the starting position of a given text, using byte positions.
///
/// [documentfoundation->SEARCHB](https://wiki.documentfoundation.org/Documentation/Calc_Functions/SEARCHB)
///
/// __Syntax__: 
/// ```ods
///     SEARCHB( Search: Text; T: Text )
/// ```
///
/// __Semantics__:
/// As SEARCH, but using byte positions.
///
/// __See also__: [crate::of::search()], [crate::of::exact()], [crate::of::find()], [crate::of::findb()], [crate::of::searchb_()], 
#[inline]
pub fn searchb<A: Text, B: Text>(search: A, t: B) -> FnNumber2<A, B> {
    FnNumber2("SEARCHB", search, t)
}

/// Returns the starting position of a given text, using byte positions.
///
/// [documentfoundation->SEARCHB](https://wiki.documentfoundation.org/Documentation/Calc_Functions/SEARCHB)
///
/// __Syntax__: 
/// ```ods
///     SEARCHB( Search: Text; T: Text; Start: BytePosition )
/// ```
///
/// __Semantics__:
/// As SEARCH, but using byte positions.
///
/// __See also__: [crate::of::search()], [crate::of::exact()], [crate::of::find()], [crate::of::findb()], [crate::of::searchb()], 
#[inline]
pub fn searchb_<A: Text, B: Text, C: Number>(search: A, t: B, start: C) -> FnNumber3<A, B, C> {
    FnNumber3("SEARCHB", search, t, start)
}
