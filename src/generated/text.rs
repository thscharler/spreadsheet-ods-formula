
use crate::*;
#[allow(unused_imports)]
use crate::text::*;

/// Converts full-width to half-width ASCII and katakana characters.
///
/// [documentfoundation->ASC](https://wiki.documentfoundation.org/Documentation/Calc_Functions/ASC)
///
/// __Syntax__: 
/// ```ods
///     ASC( T: Text )
/// ```
///
/// __Constraints__:
/// None
///
/// __Semantics__:
/// Conversion is done for full-width ASCII and UNICODE katakana characters, 
/// some characters are converted in a special way, see table below. Other 
/// characters are copied from T to the result. This is the complementary 
/// function to JIS.
/// 
/// The percent sign % in the conversion table below denotes the modulo 
/// operation. A followed by means that a character is converted to two 
/// consecutive characters.
/// 
/// Note 1: The "\" (REVERSE SOLIDUS, U+005C) is a specialty that gets 
/// displayed as a Yen sign with some Japanese fonts, which is a legacy of 
/// code-page 932.
/// 
/// Note 2: For references regarding halfwidth and fullwidth characters see 
/// UAX11 and the Halfwidth and Fullwidth Code Chart of UNICODE.
/// 
/// Note 3: For information about the mapping of JIS X 0201 and JIS X 0208 to 
/// Unicode characters see JISX0201 and JISX0208.
///
/// __See also__: [crate::of::jis()], 
#[inline]
pub fn asc<A: Text>(t: A) -> FnText1<A> {
    FnText1("ASC", t)
}

/// Return character represented by the given numeric value
///
/// [documentfoundation->CHAR](https://wiki.documentfoundation.org/Documentation/Calc_Functions/CHAR)
///
/// __Syntax__: 
/// ```ods
///     CHAR( N: Number )
/// ```
///
/// __Constraints__:
/// N ≤ 127; Evaluators may evaluate expressions where N ≥ 1, N ≤ 255.
///
/// __Semantics__:
/// 
/// Returns character represented by the given numeric value.
/// 
/// Evaluators should return an Error if N > 255.
/// 
/// Evaluators should implement CHAR such that CODE(CHAR(N)) returns N for any 
/// 1 ≤ N ≤ 255.
/// 
/// Note 1: Beyond 127, some evaluators return a character from a 
/// system-specific code page, while others return the UNICODE character. Most 
/// evaluators do not allow values greater than 255.
/// 
/// Note 2: Where interoperability is a concern, expressions should use the 
/// UNICHAR function. 6.20.25
///
/// __See also__: [crate::of::code()], [crate::of::unichar()], [crate::of::unicode()], 
#[inline]
pub fn char<A: Number>(n: A) -> FnText1<A> {
    FnText1("CHAR", n)
}

/// Remove all non-printable characters from the string and return the result.
///
/// [documentfoundation->CLEAN](https://wiki.documentfoundation.org/Documentation/Calc_Functions/CLEAN)
///
/// __Syntax__: 
/// ```ods
///     CLEAN( T: Text )
/// ```
///
/// __Semantics__:
/// 
/// Removes all non-printable characters from the string T and returns the 
/// resulting string. Evaluators should remove each particular character from 
/// the string, if and only if the character belongs to UNICODE class Cc (Other 
/// - Control), or to Unicode class Cn (Other - Not Assigned). The resulting 
/// string shall contain all printable characters from the original string, in 
/// the same order. The space character is considered a printable character.
///
/// __See also__: 
#[inline]
pub fn clean<A: Text>(t: A) -> FnText1<A> {
    FnText1("CLEAN", t)
}

/// Return numeric value corresponding to the first character of the text 
/// value.
///
/// [documentfoundation->CODE](https://wiki.documentfoundation.org/Documentation/Calc_Functions/CODE)
///
/// __Syntax__: 
/// ```ods
///     CODE( T: Text )
/// ```
///
/// __Constraints__:
/// code point ≤ 127. Evaluators may evaluate expressions where Length(T) > 
/// 0.
///
/// __Semantics__:
/// 
/// Returns a numeric value which represents the first letter of the given text 
/// T.
/// 
/// Behavior for code points ≥ 128 is implementation-defined. Evaluators may 
/// use the underlying system's code page. Evaluators should implement CODE 
/// such that CODE(CHAR(N)) returns N for 1 ≤ N ≤ 255.
///
/// __Note__:
/// Where interoperability is a concern, expressions should use the UNICODE 
/// function. 6.20.26
///
/// __See also__: [crate::of::char()], [crate::of::unichar()], [crate::of::unicode()], 
#[inline]
pub fn code<A: Text>(t: A) -> FnNumber1<A> {
    FnNumber1("CODE", t)
}

/// Concatenate the text strings
///
/// [documentfoundation->CONCATENATE](https://wiki.documentfoundation.org/Documentation/Calc_Functions/CONCATENATE)
///
/// __Syntax__: 
/// ```ods
///     CONCATENATE({ T: Text}+ )
/// ```
///
/// __Constraints__:
/// None
///
/// __Semantics__:
/// Concatenate each text value, in order, into a single text result.
///
/// __See also__: [crate::of::infix operator "&"()], 
#[inline]
pub fn concatenate<A: Sequence>(t: A) -> FnText1<A> {
    FnText1("CONCATENATE", t)
}

/// Convert the parameters to Text formatted as currency.
///
/// [documentfoundation->DOLLAR](https://wiki.documentfoundation.org/Documentation/Calc_Functions/DOLLAR)
///
/// __Syntax__: 
/// ```ods
///     DOLLAR( N: Number )
/// ```
///
/// __Constraints__:
/// None
///
/// __Semantics__:
/// Returns the value formatted as a currency, using locale-specific data. D is 
/// the number of decimal places used in the result string, a negative D rounds 
/// number N. If D is omitted, locale information may be used to determine the 
/// currency's decimal places, or a value of 2 shall be assumed.
///
/// __See also__: [crate::of::dollar_()], 
#[inline]
pub fn dollar<A: Number>(n: A) -> FnText1<A> {
    FnText1("DOLLAR", n)
}

/// Convert the parameters to Text formatted as currency.
///
/// [documentfoundation->DOLLAR](https://wiki.documentfoundation.org/Documentation/Calc_Functions/DOLLAR)
///
/// __Syntax__: 
/// ```ods
///     DOLLAR( N: Number; D: Integer )
/// ```
///
/// __Constraints__:
/// None
///
/// __Semantics__:
/// Returns the value formatted as a currency, using locale-specific data. D is 
/// the number of decimal places used in the result string, a negative D rounds 
/// number N. If D is omitted, locale information may be used to determine the 
/// currency's decimal places, or a value of 2 shall be assumed.
///
/// __See also__: [crate::of::dollar()], 
#[inline]
pub fn dollar_<A: Number, B: Number>(n: A, d: B) -> FnText2<A, B> {
    FnText2("DOLLAR", n, d)
}

/// Report if two text values are equal using a case-sensitive comparison .
///
/// [documentfoundation->EXACT](https://wiki.documentfoundation.org/Documentation/Calc_Functions/EXACT)
///
/// __Syntax__: 
/// ```ods
///     EXACT( T1: Text; T2: Text )
/// ```
///
/// __Constraints__:
/// None
///
/// __Semantics__:
/// Converts both sides to Text, and then returns TRUE if the two text values 
/// are equal, including case, otherwise it returns FALSE.
///
/// __See also__: [crate::of::find()], [crate::of::search()], [crate::of::infix operator "<>"()], [crate::of::infix operator "="()], 
#[inline]
pub fn exact<A: Text, B: Text>(t1: A, t2: B) -> FnLogical2<A, B> {
    FnLogical2("EXACT", t1, t2)
}

/// Return the starting position of a given text.
///
/// [documentfoundation->FIND](https://wiki.documentfoundation.org/Documentation/Calc_Functions/FIND)
///
/// __Syntax__: 
/// ```ods
///     FIND( Search: Text; T: Text )
/// ```
///
/// __Constraints__:
/// Start ≥ 1
///
/// __Semantics__:
/// Returns the character position where Search is first found in T, when the 
/// search is started from character position Start. The match is 
/// case-sensitive, and no wildcards or other instructions are considered in 
/// Search. Returns an Error if text not found.
///
/// __See also__: [crate::of::exact()], [crate::of::search()], [crate::of::find_()], 
#[inline]
pub fn find<A: Text, B: Text>(search: A, t: B) -> FnNumber2<A, B> {
    FnNumber2("FIND", search, t)
}

/// Return the starting position of a given text.
///
/// [documentfoundation->FIND](https://wiki.documentfoundation.org/Documentation/Calc_Functions/FIND)
///
/// __Syntax__: 
/// ```ods
///     FIND( Search: Text; T: Text; Start: Integer )
/// ```
///
/// __Constraints__:
/// Start ≥ 1
///
/// __Semantics__:
/// Returns the character position where Search is first found in T, when the 
/// search is started from character position Start. The match is 
/// case-sensitive, and no wildcards or other instructions are considered in 
/// Search. Returns an Error if text not found.
///
/// __See also__: [crate::of::exact()], [crate::of::search()], [crate::of::find()], 
#[inline]
pub fn find_<A: Text, B: Text, C: Number>(search: A, t: B, start: C) -> FnNumber3<A, B, C> {
    FnNumber3("FIND", search, t, start)
}

/// Round the number to a specified number of decimals and format the result as 
/// a text.
///
/// [documentfoundation->FIXED](https://wiki.documentfoundation.org/Documentation/Calc_Functions/FIXED)
///
/// __Syntax__: 
/// ```ods
///     FIXED( N: Number )
/// ```
///
/// __Constraints__:
/// None
///
/// __Semantics__:
/// Rounds value N to D decimal places (after the decimal point) and returns 
/// the result formatted as text, using locale-specific settings. If D is 
/// negative, the number is rounded to ABS(D) places to the left from the 
/// decimal point. If the optional parameter OmitSeparators is TRUE, then group 
/// separators are omitted from the resulting string. Group separators are 
/// included in the absence of this parameter. If D is a fraction, it is 
/// rounded towards 0 as an integer (ignoring what is the closest integer).
///
/// __See also__: [crate::of::abs()], [crate::of::fixed_()], [crate::of::fixed__()], 
#[inline]
pub fn fixed<A: Number>(n: A) -> FnText1<A> {
    FnText1("FIXED", n)
}

/// Round the number to a specified number of decimals and format the result as 
/// a text.
///
/// [documentfoundation->FIXED](https://wiki.documentfoundation.org/Documentation/Calc_Functions/FIXED)
///
/// __Syntax__: 
/// ```ods
///     FIXED( N: Number; D: Integer )
/// ```
///
/// __Constraints__:
/// None
///
/// __Semantics__:
/// Rounds value N to D decimal places (after the decimal point) and returns 
/// the result formatted as text, using locale-specific settings. If D is 
/// negative, the number is rounded to ABS(D) places to the left from the 
/// decimal point. If the optional parameter OmitSeparators is TRUE, then group 
/// separators are omitted from the resulting string. Group separators are 
/// included in the absence of this parameter. If D is a fraction, it is 
/// rounded towards 0 as an integer (ignoring what is the closest integer).
///
/// __See also__: [crate::of::abs()], [crate::of::fixed()], [crate::of::fixed__()], 
#[inline]
pub fn fixed_<A: Number, B: Number>(n: A, d: B) -> FnText2<A, B> {
    FnText2("FIXED", n, d)
}

/// Round the number to a specified number of decimals and format the result as 
/// a text.
///
/// [documentfoundation->FIXED](https://wiki.documentfoundation.org/Documentation/Calc_Functions/FIXED)
///
/// __Syntax__: 
/// ```ods
///     FIXED( N: Number; D: Integer; OmitSeparators: Logical )
/// ```
///
/// __Constraints__:
/// None
///
/// __Semantics__:
/// Rounds value N to D decimal places (after the decimal point) and returns 
/// the result formatted as text, using locale-specific settings. If D is 
/// negative, the number is rounded to ABS(D) places to the left from the 
/// decimal point. If the optional parameter OmitSeparators is TRUE, then group 
/// separators are omitted from the resulting string. Group separators are 
/// included in the absence of this parameter. If D is a fraction, it is 
/// rounded towards 0 as an integer (ignoring what is the closest integer).
///
/// __See also__: [crate::of::abs()], [crate::of::fixed()], [crate::of::fixed_()], 
#[inline]
pub fn fixed__<A: Number, B: Number, C: Logical>(n: A, d: B, omit_separators: C) -> FnText3<A, B, C> {
    FnText3("FIXED", n, d, omit_separators)
}

/// Converts half-width to full-width ASCII and katakana characters.
///
/// [documentfoundation->JIS](https://wiki.documentfoundation.org/Documentation/Calc_Functions/JIS)
///
/// __Syntax__: 
/// ```ods
///     JIS( T: Text )
/// ```
///
/// __Constraints__:
/// None
///
/// __Semantics__:
/// Conversion is done for half-width ASCII and UNICODE katakana characters, 
/// some characters are converted in a special way, see table below. Other 
/// characters are copied from T to the result. This is the complementary 
/// function to ASC.
/// 
/// A followed by means that there are two consecutive characters to convert 
/// from.
/// 
/// Note 1: For references regarding halfwidth and fullwidth characters see 
/// UAX11 and the Halfwidth and Fullwidth Code Chart of UNICODE.
/// 
/// Note 2: For information about the mapping of JIS X 0201 and JIS X 0208 to 
/// Unicode characters see JISX0201 and JISX0208.
///
/// __See also__: [crate::of::asc()], 
#[inline]
pub fn jis<A: Text>(t: A) -> FnText1<A> {
    FnText1("JIS", t)
}

/// Return a selected number of text characters from the left.
///
/// [documentfoundation->LEFT](https://wiki.documentfoundation.org/Documentation/Calc_Functions/LEFT)
///
/// __Syntax__: 
/// ```ods
///     LEFT( T: Text )
/// ```
///
/// __Constraints__:
/// Length ≥ 0
///
/// __Semantics__:
/// Returns the INT(Length) number of characters of text T, starting from the 
/// left. If Length is omitted, it defaults to 1; otherwise, it computes Length 
/// = INT(Length). If T has fewer than Length characters, it returns T. This 
/// means that if T is an empty string (which has length 0) or the parameter 
/// Length is 0, LEFT() will always return an empty string. Note that if Length 
/// < 0, an Error is returned. This function shall return the same string as 
/// MID(T; 1; Length).
/// 
/// The results of this function may be normalization-sensitive. 4.2
///
/// __See also__: [crate::of::int()], [crate::of::len()], [crate::of::mid()], [crate::of::right()], [crate::of::left_()], 
#[inline]
pub fn left<A: Text>(t: A) -> FnText1<A> {
    FnText1("LEFT", t)
}

/// Return a selected number of text characters from the left.
///
/// [documentfoundation->LEFT](https://wiki.documentfoundation.org/Documentation/Calc_Functions/LEFT)
///
/// __Syntax__: 
/// ```ods
///     LEFT( T: Text; Length: Integer )
/// ```
///
/// __Constraints__:
/// Length ≥ 0
///
/// __Semantics__:
/// Returns the INT(Length) number of characters of text T, starting from the 
/// left. If Length is omitted, it defaults to 1; otherwise, it computes Length 
/// = INT(Length). If T has fewer than Length characters, it returns T. This 
/// means that if T is an empty string (which has length 0) or the parameter 
/// Length is 0, LEFT() will always return an empty string. Note that if Length 
/// < 0, an Error is returned. This function shall return the same string as 
/// MID(T; 1; Length).
/// 
/// The results of this function may be normalization-sensitive. 4.2
///
/// __See also__: [crate::of::int()], [crate::of::len()], [crate::of::mid()], [crate::of::right()], [crate::of::left()], 
#[inline]
pub fn left_<A: Text, B: Number>(t: A, length: B) -> FnText2<A, B> {
    FnText2("LEFT", t, length)
}

/// Return the length, in characters, of given text
///
/// [documentfoundation->LEN](https://wiki.documentfoundation.org/Documentation/Calc_Functions/LEN)
///
/// __Syntax__: 
/// ```ods
///     LEN( T: Text )
/// ```
///
/// __Constraints__:
/// None.
///
/// __Semantics__:
/// Computes number of characters (not the number of bytes) in T. If T is of 
/// type Number, it is automatically converted to Text, including a fractional 
/// part and decimal separator if necessary.
/// 
/// The results of this function may be normalization-sensitive. 4.2
///
/// __See also__: [crate::of::text()], [crate::of::istext()], [crate::of::left()], [crate::of::mid()], [crate::of::right()], 
#[inline]
pub fn len<A: Text>(t: A) -> FnNumber1<A> {
    FnNumber1("LEN", t)
}

/// Return input string, but with all uppercase letters converted to lowercase 
/// letters.
///
/// [documentfoundation->LOWER](https://wiki.documentfoundation.org/Documentation/Calc_Functions/LOWER)
///
/// __Syntax__: 
/// ```ods
///     LOWER( T: Text )
/// ```
///
/// __Constraints__:
/// None
///
/// __Semantics__:
/// Return input string, but with all uppercase letters converted to lowercase 
/// letters, as defined by §3.13 Default Case Algorithms, §4.2 Case-Normative 
/// and §5.18 Case Mappings of UNICODE. As with most functions, it is 
/// side-effect free (it does not modify the source values). All Evaluators 
/// shall convert A-Z to a-z.
///
/// __Note__:
/// As this function can be locale aware, results may be unexpected in certain 
/// cases. For example in a Turkish locale an upper case "I without dot" (LATIN 
/// CAPITAL LETTER I, U+0049) is converted to a lower case "i without dot" 
/// (LATIN SMALL LETTER DOTLESS I, U+0131).
///
/// __See also__: [crate::of::upper()], [crate::of::proper()], 
#[inline]
pub fn lower<A: Text>(t: A) -> FnText1<A> {
    FnText1("LOWER", t)
}

/// Returns extracted text, given an original text, starting position, and 
/// length.
///
/// [documentfoundation->MID](https://wiki.documentfoundation.org/Documentation/Calc_Functions/MID)
///
/// __Syntax__: 
/// ```ods
///     MID( T: Text; Start: Integer; Length: Integer )
/// ```
///
/// __Constraints__:
/// Start ≥ 1, Length ≥ 0.
///
/// __Semantics__:
/// Returns the characters from T, starting at character position Start, for up 
/// to Length characters. For the integer conversions, Start = INT(Start), and 
/// Length = INT(Length). If there are less than Length characters starting at 
/// start, it returns as many characters as it can beginning with Start. In 
/// particular, if Start > LEN(T), it returns the empty string (""). If Start < 
/// 0, it returns an Error. If Start ≥ 0, and Length = 0, it returns the 
/// empty string. Note that MID(T;1;Length) produces the same results as 
/// LEFT(T;Length).
/// 
/// The results of this function may be normalization-sensitive. 4.2
///
/// __See also__: [crate::of::int()], [crate::of::left()], [crate::of::len()], [crate::of::right()], [crate::of::replace()], [crate::of::substitute()], 
#[inline]
pub fn mid<A: Text, B: Number, C: Number>(t: A, start: B, length: C) -> FnText3<A, B, C> {
    FnText3("MID", t, start, length)
}

/// Return the input string with the first letter of each word converted to an 
/// uppercase letter and the rest of the letters in the word converted to 
/// lowercase.
///
/// [documentfoundation->PROPER](https://wiki.documentfoundation.org/Documentation/Calc_Functions/PROPER)
///
/// __Syntax__: 
/// ```ods
///     PROPER( T: Text )
/// ```
///
/// __Constraints__:
/// None
///
/// __Semantics__:
/// Return input string, but modified as follows:
/// 
/// ●If the first character is a letter, it is converted to its uppercase 
/// equivalent; otherwise, the original character is returned
/// 
/// ●If a letter is preceded by a non-letter, it is converted to its 
/// uppercase equivalent
/// 
/// ●If a letter is preceded by a letter, it is converted to its lowercase 
/// equivalent.
/// 
/// Evaluators shall implement this for at least the Latin letters A-Z and a-z.
/// 
/// As with most functions, it is side-effect free, that is, it does not modify 
/// the source values.
///
/// __See also__: [crate::of::lower()], [crate::of::upper()], 
#[inline]
pub fn proper<A: Text>(t: A) -> FnText1<A> {
    FnText1("PROPER", t)
}

/// Returns text where an old text is substituted with a new text.
///
/// [documentfoundation->REPLACE](https://wiki.documentfoundation.org/Documentation/Calc_Functions/REPLACE)
///
/// __Syntax__: 
/// ```ods
///     REPLACE( T: Text; Start: Number; Count: Number; New: Text )
/// ```
///
/// __Constraints__:
/// Start ≥ 1.
///
/// __Semantics__:
/// Returns text T, but remove the characters starting at character position 
/// Start for Count characters, and instead replace them with New. Character 
/// positions defined by Start begin at 1 (for the leftmost character). If 
/// Count=0, the text New is inserted before character position Start, and all 
/// the text before and after Start is retained. If Start > length of text T 
/// (TLen) then Start is set to TLen. If Count > TLen - Start then Count is set 
/// to TLen - Start.
/// 
/// REPLACE(T;Start;Len;New) is the same as LEFT(T;Start - 1) & New & MID(T; 
/// Start + Len; LEN(T)))
///
/// __See also__: [crate::of::left()], [crate::of::len()], [crate::of::mid()], [crate::of::right()], [crate::of::substitute()], 
#[inline]
pub fn replace<A: Text, B: Number, C: Number, D: Text>(t: A, start: B, count: C, new: D) -> FnText4<A, B, C, D> {
    FnText4("REPLACE", t, start, count, new)
}

/// Return text repeated Count times.
///
/// [documentfoundation->REPT](https://wiki.documentfoundation.org/Documentation/Calc_Functions/REPT)
///
/// __Syntax__: 
/// ```ods
///     REPT( T: Text; Count: Integer )
/// ```
///
/// __Constraints__:
/// Count ≥ 0
///
/// __Semantics__:
/// Returns text T repeated Count number of times; if Count is zero, an empty 
/// string is returned. If Count < 0, the result is Error.
///
/// __See also__: [crate::of::left()], [crate::of::mid()], [crate::of::right()], [crate::of::substitute()], 
#[inline]
pub fn rept<A: Text, B: Number>(t: A, count: B) -> FnText2<A, B> {
    FnText2("REPT", t, count)
}

/// Return a selected number of text characters from the right.
///
/// [documentfoundation->RIGHT](https://wiki.documentfoundation.org/Documentation/Calc_Functions/RIGHT)
///
/// __Syntax__: 
/// ```ods
///     RIGHT( T: Text )
/// ```
///
/// __Constraints__:
/// Length ≥ 0
///
/// __Semantics__:
/// Returns the Length number of characters of text T, starting from the right. 
/// If Length is omitted, it defaults to 1; otherwise, it computes Length = 
/// INT(Length). If T has fewer than Length characters, it returns T 
/// (unchanged). This means that if T is an empty string (which has length 0) 
/// or the parameter Length is 0, RIGHT() will always return an empty string. 
/// Note that if Length < 0, an Error is returned.
/// 
/// The results of this function may be normalization-sensitive. 4.2
///
/// __See also__: [crate::of::int()], [crate::of::left()], [crate::of::len()], [crate::of::mid()], [crate::of::right_()], 
#[inline]
pub fn right<A: Text>(t: A) -> FnText1<A> {
    FnText1("RIGHT", t)
}

/// Return a selected number of text characters from the right.
///
/// [documentfoundation->RIGHT](https://wiki.documentfoundation.org/Documentation/Calc_Functions/RIGHT)
///
/// __Syntax__: 
/// ```ods
///     RIGHT( T: Text; Length: Integer )
/// ```
///
/// __Constraints__:
/// Length ≥ 0
///
/// __Semantics__:
/// Returns the Length number of characters of text T, starting from the right. 
/// If Length is omitted, it defaults to 1; otherwise, it computes Length = 
/// INT(Length). If T has fewer than Length characters, it returns T 
/// (unchanged). This means that if T is an empty string (which has length 0) 
/// or the parameter Length is 0, RIGHT() will always return an empty string. 
/// Note that if Length < 0, an Error is returned.
/// 
/// The results of this function may be normalization-sensitive. 4.2
///
/// __See also__: [crate::of::int()], [crate::of::left()], [crate::of::len()], [crate::of::mid()], [crate::of::right()], 
#[inline]
pub fn right_<A: Text, B: Number>(t: A, length: B) -> FnText2<A, B> {
    FnText2("RIGHT", t, length)
}

/// Return the starting position of a given text.
///
/// [documentfoundation->SEARCH](https://wiki.documentfoundation.org/Documentation/Calc_Functions/SEARCH)
///
/// __Syntax__: 
/// ```ods
///     SEARCH( Search: Text; T: Text )
/// ```
///
/// __Constraints__:
/// Start ≥ 1
///
/// __Semantics__:
/// Returns the character position where Search is first found in T, when the 
/// search is started from character position Start. The match is not 
/// case-sensitive. Start is 1 if omitted. Returns an Error if text not found.
/// 
/// The values returned may vary depending upon the 
/// HOST-USE-REGULAR-EXPRESSIONS or HOST-USE-WILDCARDS properties. 3.4
///
/// __See also__: [crate::of::exact()], [crate::of::find()], [crate::of::search_()], 
#[inline]
pub fn search<A: Text, B: Text>(search: A, t: B) -> FnNumber2<A, B> {
    FnNumber2("SEARCH", search, t)
}

/// Return the starting position of a given text.
///
/// [documentfoundation->SEARCH](https://wiki.documentfoundation.org/Documentation/Calc_Functions/SEARCH)
///
/// __Syntax__: 
/// ```ods
///     SEARCH( Search: Text; T: Text; Start: Integer )
/// ```
///
/// __Constraints__:
/// Start ≥ 1
///
/// __Semantics__:
/// Returns the character position where Search is first found in T, when the 
/// search is started from character position Start. The match is not 
/// case-sensitive. Start is 1 if omitted. Returns an Error if text not found.
/// 
/// The values returned may vary depending upon the 
/// HOST-USE-REGULAR-EXPRESSIONS or HOST-USE-WILDCARDS properties. 3.4
///
/// __See also__: [crate::of::exact()], [crate::of::find()], [crate::of::search()], 
#[inline]
pub fn search_<A: Text, B: Text, C: Number>(search: A, t: B, start: C) -> FnNumber3<A, B, C> {
    FnNumber3("SEARCH", search, t, start)
}

/// Returns text where an old text is substituted with a new text.
///
/// [documentfoundation->SUBSTITUTE](https://wiki.documentfoundation.org/Documentation/Calc_Functions/SUBSTITUTE)
///
/// __Syntax__: 
/// ```ods
///     SUBSTITUTE( T: Text; Old: Text; New: Text )
/// ```
///
/// __Constraints__:
/// Which ≥ 1 (when provided)
///
/// __Semantics__:
/// Returns text T, but with text Old replaced by text New (when searching from 
/// the left). If Which is omitted, every occurrence of Old is replaced with 
/// New; if Which is provided, only that occurrence of Old is replaced by New 
/// (starting the count from 1). If there is no match, or if Old has length 0, 
/// the value of T is returned. Note that Old and New may have different 
/// lengths. If Which is present and Which < 1, returns Error.
///
/// __See also__: [crate::of::left()], [crate::of::len()], [crate::of::mid()], [crate::of::replace()], [crate::of::right()], [crate::of::substitute_()], 
#[inline]
pub fn substitute<A: Text, B: Text, C: Text>(t: A, old: B, new: C) -> FnText3<A, B, C> {
    FnText3("SUBSTITUTE", t, old, new)
}

/// Returns text where an old text is substituted with a new text.
///
/// [documentfoundation->SUBSTITUTE](https://wiki.documentfoundation.org/Documentation/Calc_Functions/SUBSTITUTE)
///
/// __Syntax__: 
/// ```ods
///     SUBSTITUTE( T: Text; Old: Text; New: Text; Which: Integer )
/// ```
///
/// __Constraints__:
/// Which ≥ 1 (when provided)
///
/// __Semantics__:
/// Returns text T, but with text Old replaced by text New (when searching from 
/// the left). If Which is omitted, every occurrence of Old is replaced with 
/// New; if Which is provided, only that occurrence of Old is replaced by New 
/// (starting the count from 1). If there is no match, or if Old has length 0, 
/// the value of T is returned. Note that Old and New may have different 
/// lengths. If Which is present and Which < 1, returns Error.
///
/// __See also__: [crate::of::left()], [crate::of::len()], [crate::of::mid()], [crate::of::replace()], [crate::of::right()], [crate::of::substitute()], 
#[inline]
pub fn substitute_<A: Text, B: Text, C: Text, D: Number>(t: A, old: B, new: C, which: D) -> FnText4<A, B, C, D> {
    FnText4("SUBSTITUTE", t, old, new, which)
}

/// Return the text (if Text), else return 0-length Text value
///
/// [documentfoundation->T](https://wiki.documentfoundation.org/Documentation/Calc_Functions/T)
///
/// __Syntax__: 
/// ```ods
///     T( X: Any )
/// ```
///
/// __Constraints__:
/// None
///
/// __Semantics__:
/// The type of (a dereferenced) X is examined; if it is of type Text, it is 
/// returned, else an empty string (Text value of zero length) is returned. 
/// This is not a type-conversion function; T(5) produces an empty string, not 
/// "5".
///
/// __See also__: [crate::of::n()], 
#[inline]
pub fn t<A: Any>(x: A) -> FnText1<A> {
    FnText1("T", x)
}

/// Return the value converted to a text.
///
/// [documentfoundation->TEXT](https://wiki.documentfoundation.org/Documentation/Calc_Functions/TEXT)
///
/// __Syntax__: 
/// ```ods
///     TEXT( X: Scalar; FormatCode: Text )
/// ```
///
/// __Constraints__:
/// The FormatCode is a sequence of characters with an implementation-defined 
/// meaning.
///
/// __Semantics__:
/// Converts the value X to a Text according to the rules of a number format 
/// code passed as FormatCode and returns it.
///
/// __See also__: [crate::of::n()], [crate::of::t()], 
#[inline]
pub fn text<A: Scalar, B: Text>(x: A, format_code: B) -> FnText2<A, B> {
    FnText2("TEXT", x, format_code)
}

/// Remove leading and trailing spaces, and replace all internal multiple 
/// spaces with a single space.
///
/// [documentfoundation->TRIM](https://wiki.documentfoundation.org/Documentation/Calc_Functions/TRIM)
///
/// __Syntax__: 
/// ```ods
///     TRIM( T: Text )
/// ```
///
/// __Constraints__:
/// None.
///
/// __Semantics__:
/// Takes T and removes all leading and trailing space. Any other sequence of 2 
/// or more spaces is replaced with a single space.
/// 
/// A space is one or more, HORIZONTAL TABULATION (U+0009), LINE FEED (U+000A), 
/// CARRIAGE RETURN (U+000D) or SPACE (U+0020) characters.
///
/// __See also__: [crate::of::left()], [crate::of::right()], 
#[inline]
pub fn trim<A: Text>(t: A) -> FnText1<A> {
    FnText1("TRIM", t)
}

/// Return the character represented by the given numeric value according to 
/// the UNICODE Standard.
///
/// [documentfoundation->UNICHAR](https://wiki.documentfoundation.org/Documentation/Calc_Functions/UNICHAR)
///
/// __Syntax__: 
/// ```ods
///     UNICHAR( N: Integer )
/// ```
///
/// __Constraints__:
/// N ≥ 0, N ≤ 1114111 (U+10FFFF)
///
/// __Semantics__:
/// Returns the character having the given numeric value as UNICODE code point.
/// Evaluators shall support values between 1 and 0xFFFF. Evaluators should 
/// allow N to be any UNICODE code point of type Graphic, Format or Control. 
/// Evaluators should implement UNICHAR such that UNICODE(UNICHAR(N)) returns N 
/// for any UNICODE code point N of type Graphic, Format or Control.
///
/// __See also__: [crate::of::unicode()], 
#[inline]
pub fn unichar<A: Number>(n: A) -> FnText1<A> {
    FnText1("UNICHAR", n)
}

/// Return the UNICODE code point corresponding to the first character of the 
/// text value.
///
/// [documentfoundation->UNICODE](https://wiki.documentfoundation.org/Documentation/Calc_Functions/UNICODE)
///
/// __Syntax__: 
/// ```ods
///     UNICODE( T: Text )
/// ```
///
/// __Constraints__:
/// Length(T) > 0.
///
/// __Semantics__:
/// Returns the numeric value of the UNICODE code point of the first character 
/// of the given text T.
/// 
/// The results of this function may be normalization-sensitive. 4.2
///
/// __See also__: [crate::of::unichar()], 
#[inline]
pub fn unicode<A: Text>(t: A) -> FnNumber1<A> {
    FnNumber1("UNICODE", t)
}

/// Return input string, but with all lowercase letters converted to uppercase 
/// letters.
///
/// [documentfoundation->UPPER](https://wiki.documentfoundation.org/Documentation/Calc_Functions/UPPER)
///
/// __Syntax__: 
/// ```ods
///     UPPER( T: Text )
/// ```
///
/// __Constraints__:
/// None
///
/// __Semantics__:
/// Return input string, but with all lowercase letters converted to uppercase 
/// letters, as defined by §3.13 Default Case Algorithms, §4.2 Case-Normative 
/// and §5.18 Case Mappings of UNICODE. As with most functions, it is 
/// side-effect free (it does not modify the source values). All Evaluators 
/// shall convert a-z to A-Z.
///
/// __Note__:
/// As this function can be locale aware, results may be unexpected in certain 
/// cases, for example in a Turkish locale a lower case "i with dot" (LATIN 
/// SMALL LETTER I) U+0069 is converted to an upper case "I with dot" (LATIN 
/// CAPITAL LETTER I WITH DOT ABOVE, U+0130).
///
/// __See also__: [crate::of::lower()], [crate::of::proper()], 
#[inline]
pub fn upper<A: Text>(t: A) -> FnText1<A> {
    FnText1("UPPER", t)
}
