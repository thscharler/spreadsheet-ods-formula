
use crate::*;
#[allow(unused_imports)]
use crate::text::*;

/// Converts full-width to half-width ASCII and katakana characters.
/// Syntax: ASC( T Text; )
///
/// Constraints:
/// None
///
/// Semantics:
/// Conversion is done for full-width ASCII and [UNICODE] katakana characters, 
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
/// [UAX11] and the Halfwidth and Fullwidth Code Chart of [UNICODE].
/// 
/// Note 3: For information about the mapping of JIS X 0201 and JIS X 0208 to 
/// Unicode characters see [JISX0201] and [JISX0208].
///
/// See also: "JIS", 
#[inline]
pub fn asc<A: Text>(t: A) -> FnText1<A> {
    FnText1("ASC", t)
}

/// Return character represented by the given numeric value
/// Syntax: CHAR( N Number; )
///
/// Constraints:
/// N ≤ 127; Evaluators may evaluate expressions where N ≥ 1, N ≤ 255.
///
/// Semantics:
/// 
/// Returns character represented by the given numeric value.
/// 
/// Evaluators should return an Error if N > 255.
/// 
/// Evaluators should implement CHAR such that CODE(CHAR(N)) returns N for any 
/// 1 ≤ N ≤ 255.
/// 
/// Note 1: Beyond 127, some evaluators return a character from a 
/// system-specific code page, while others return the [UNICODE] character. 
/// Most evaluators do not allow values greater than 255.
/// 
/// Note 2: Where interoperability is a concern, expressions should use the 
/// UNICHAR function. 6.20.25
///
/// See also: "CODE", "UNICHAR", "UNICODE", 
#[inline]
pub fn char<A: Number>(n: A) -> FnText1<A> {
    FnText1("CHAR", n)
}

/// Remove all non-printable characters from the string and return the result.
/// Syntax: CLEAN( T Text; )
///
/// Semantics:
/// 
/// Removes all non-printable characters from the string T and returns the 
/// resulting string. Evaluators should remove each particular character from 
/// the string, if and only if the character belongs to [UNICODE] class Cc 
/// (Other - Control), or to Unicode class Cn (Other - Not Assigned). The 
/// resulting string shall contain all printable characters from the original 
/// string, in the same order. The space character is considered a printable 
/// character.
#[inline]
pub fn clean<A: Text>(t: A) -> FnText1<A> {
    FnText1("CLEAN", t)
}

/// Return numeric value corresponding to the first character of the text 
/// value.
/// Syntax: CODE( T Text; )
///
/// Constraints:
/// code point ≤ 127. Evaluators may evaluate expressions where Length(T) > 
/// 0.
///
/// Semantics:
/// 
/// Returns a numeric value which represents the first letter of the given text 
/// T.
/// 
/// Behavior for code points ≥ 128 is implementation-defined. Evaluators may 
/// use the underlying system's code page. Evaluators should implement CODE 
/// such that CODE(CHAR(N)) returns N for 1 ≤ N ≤ 255.
///
/// Note:
/// Where interoperability is a concern, expressions should use the UNICODE 
/// function. 6.20.26
///
/// See also: "CHAR", "UNICHAR", "UNICODE", 
#[inline]
pub fn code<A: Text>(t: A) -> FnNumber1<A> {
    FnNumber1("CODE", t)
}

/// Concatenate the text strings
/// Syntax: CONCATENATE({ T Text}+ )
///
/// Constraints:
/// None
///
/// Semantics:
/// Concatenate each text value, in order, into a single text result.
///
/// See also: "Infix Operator \"&\"", 
#[inline]
pub fn concatenate<A: Sequence>(t: A) -> FnText1<A> {
    FnText1("CONCATENATE", t)
}

/// Convert the parameters to Text formatted as currency.
/// Syntax: DOLLAR( N Number; )
///
/// Constraints:
/// None
///
/// Semantics:
/// Returns the value formatted as a currency, using locale-specific data. D is 
/// the number of decimal places used in the result string, a negative D rounds 
/// number N. If D is omitted, locale information may be used to determine the 
/// currency's decimal places, or a value of 2 shall be assumed.
#[inline]
pub fn dollar<A: Number>(n: A) -> FnText1<A> {
    FnText1("DOLLAR", n)
}

/// Convert the parameters to Text formatted as currency.
/// Syntax: DOLLAR( N Number;[; D Integer] )
///
/// Constraints:
/// None
///
/// Semantics:
/// Returns the value formatted as a currency, using locale-specific data. D is 
/// the number of decimal places used in the result string, a negative D rounds 
/// number N. If D is omitted, locale information may be used to determine the 
/// currency's decimal places, or a value of 2 shall be assumed.
#[inline]
pub fn dollar_<A: Number, B: Number>(n: A, d: B) -> FnText2<A, B> {
    FnText2("DOLLAR", n, d)
}

/// Report if two text values are equal using a case-sensitive comparison .
/// Syntax: EXACT( T1 Text;; T2 Text; )
///
/// Constraints:
/// None
///
/// Semantics:
/// Converts both sides to Text, and then returns TRUE if the two text values 
/// are equal, including case, otherwise it returns FALSE.
///
/// See also: "FIND", "SEARCH", "Infix Operator \"<>\"", "Infix Operator \"=\"", 
#[inline]
pub fn exact<A: Text, B: Text>(t1: A, t2: B) -> FnLogical2<A, B> {
    FnLogical2("EXACT", t1, t2)
}

/// Return the starting position of a given text.
/// Syntax: FIND( Search Text;; T Text; )
///
/// Constraints:
/// Start ≥ 1
///
/// Semantics:
/// Returns the character position where Search is first found in T, when the 
/// search is started from character position Start. The match is 
/// case-sensitive, and no wildcards or other instructions are considered in 
/// Search. Returns an Error if text not found.
///
/// See also: "EXACT", "SEARCH", 
#[inline]
pub fn find<A: Text, B: Text>(search: A, t: B) -> FnNumber2<A, B> {
    FnNumber2("FIND", search, t)
}

/// Return the starting position of a given text.
/// Syntax: FIND( Search Text;; T Text;[; Start Integer] )
///
/// Constraints:
/// Start ≥ 1
///
/// Semantics:
/// Returns the character position where Search is first found in T, when the 
/// search is started from character position Start. The match is 
/// case-sensitive, and no wildcards or other instructions are considered in 
/// Search. Returns an Error if text not found.
///
/// See also: "EXACT", "SEARCH", 
#[inline]
pub fn find_<A: Text, B: Text, C: Number>(search: A, t: B, start: C) -> FnNumber3<A, B, C> {
    FnNumber3("FIND", search, t, start)
}

/// Round the number to a specified number of decimals and format the result as 
/// a text.
/// Syntax: FIXED( N Number; )
///
/// Constraints:
/// None
///
/// Semantics:
/// Rounds value N to D decimal places (after the decimal point) and returns 
/// the result formatted as text, using locale-specific settings. If D is 
/// negative, the number is rounded to ABS(D) places to the left from the 
/// decimal point. If the optional parameter OmitSeparators is TRUE, then group 
/// separators are omitted from the resulting string. Group separators are 
/// included in the absence of this parameter. If D is a fraction, it is 
/// rounded towards 0 as an integer (ignoring what is the closest integer).
///
/// See also: "ABS", 
#[inline]
pub fn fixed<A: Number>(n: A) -> FnText1<A> {
    FnText1("FIXED", n)
}

/// Round the number to a specified number of decimals and format the result as 
/// a text.
/// Syntax: FIXED( N Number;[; D Integer] )
///
/// Constraints:
/// None
///
/// Semantics:
/// Rounds value N to D decimal places (after the decimal point) and returns 
/// the result formatted as text, using locale-specific settings. If D is 
/// negative, the number is rounded to ABS(D) places to the left from the 
/// decimal point. If the optional parameter OmitSeparators is TRUE, then group 
/// separators are omitted from the resulting string. Group separators are 
/// included in the absence of this parameter. If D is a fraction, it is 
/// rounded towards 0 as an integer (ignoring what is the closest integer).
///
/// See also: "ABS", 
#[inline]
pub fn fixed_<A: Number, B: Number>(n: A, d: B) -> FnText2<A, B> {
    FnText2("FIXED", n, d)
}

/// Round the number to a specified number of decimals and format the result as 
/// a text.
/// Syntax: FIXED( N Number;[; D Integer][; OmitSeparators Logical] )
///
/// Constraints:
/// None
///
/// Semantics:
/// Rounds value N to D decimal places (after the decimal point) and returns 
/// the result formatted as text, using locale-specific settings. If D is 
/// negative, the number is rounded to ABS(D) places to the left from the 
/// decimal point. If the optional parameter OmitSeparators is TRUE, then group 
/// separators are omitted from the resulting string. Group separators are 
/// included in the absence of this parameter. If D is a fraction, it is 
/// rounded towards 0 as an integer (ignoring what is the closest integer).
///
/// See also: "ABS", 
#[inline]
pub fn fixed__<A: Number, B: Number, C: Logical>(n: A, d: B, omit_separators: C) -> FnText3<A, B, C> {
    FnText3("FIXED", n, d, omit_separators)
}

/// Converts half-width to full-width ASCII and katakana characters.
/// Syntax: JIS( T Text; )
///
/// Constraints:
/// None
///
/// Semantics:
/// Conversion is done for half-width ASCII and [UNICODE] katakana characters, 
/// some characters are converted in a special way, see table below. Other 
/// characters are copied from T to the result. This is the complementary 
/// function to ASC.
/// 
/// A followed by means that there are two consecutive characters to convert 
/// from.
/// 
/// Note 1: For references regarding halfwidth and fullwidth characters see 
/// [UAX11] and the Halfwidth and Fullwidth Code Chart of [UNICODE].
/// 
/// Note 2: For information about the mapping of JIS X 0201 and JIS X 0208 to 
/// Unicode characters see [JISX0201] and [JISX0208].
///
/// See also: "ASC", 
#[inline]
pub fn jis<A: Text>(t: A) -> FnText1<A> {
    FnText1("JIS", t)
}

/// Return a selected number of text characters from the left.
/// Syntax: LEFT( T Text; )
///
/// Constraints:
/// Length ≥ 0
///
/// Semantics:
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
/// See also: "INT", "LEN", "MID", "RIGHT", 
#[inline]
pub fn left<A: Text>(t: A) -> FnText1<A> {
    FnText1("LEFT", t)
}

/// Return a selected number of text characters from the left.
/// Syntax: LEFT( T Text;[; Length Integer] )
///
/// Constraints:
/// Length ≥ 0
///
/// Semantics:
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
/// See also: "INT", "LEN", "MID", "RIGHT", 
#[inline]
pub fn left_<A: Text, B: Number>(t: A, length: B) -> FnText2<A, B> {
    FnText2("LEFT", t, length)
}

/// Return the length, in characters, of given text
/// Syntax: LEN( T Text; )
///
/// Constraints:
/// None.
///
/// Semantics:
/// Computes number of characters (not the number of bytes) in T. If T is of 
/// type Number, it is automatically converted to Text, including a fractional 
/// part and decimal separator if necessary.
/// 
/// The results of this function may be normalization-sensitive. 4.2
///
/// See also: "TEXT", "ISTEXT", "LEFT", "MID", "RIGHT", 
#[inline]
pub fn len<A: Text>(t: A) -> FnNumber1<A> {
    FnNumber1("LEN", t)
}

/// Return input string, but with all uppercase letters converted to lowercase 
/// letters.
/// Syntax: LOWER( T Text; )
///
/// Constraints:
/// None
///
/// Semantics:
/// Return input string, but with all uppercase letters converted to lowercase 
/// letters, as defined by §3.13 Default Case Algorithms, §4.2 Case-Normative 
/// and §5.18 Case Mappings of [UNICODE]. As with most functions, it is 
/// side-effect free (it does not modify the source values). All Evaluators 
/// shall convert A-Z to a-z.
///
/// Note:
/// As this function can be locale aware, results may be unexpected in certain 
/// cases. For example in a Turkish locale an upper case "I without dot" (LATIN 
/// CAPITAL LETTER I, U+0049) is converted to a lower case "i without dot" 
/// (LATIN SMALL LETTER DOTLESS I, U+0131).
///
/// See also: "UPPER", "PROPER", 
#[inline]
pub fn lower<A: Text>(t: A) -> FnText1<A> {
    FnText1("LOWER", t)
}

/// Returns extracted text, given an original text, starting position, and 
/// length.
/// Syntax: MID( T Text;; Start Integer;; Length Integer; )
///
/// Constraints:
/// Start ≥ 1, Length ≥ 0.
///
/// Semantics:
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
/// See also: "INT", "LEFT", "LEN", "RIGHT", "REPLACE", "SUBSTITUTE", 
#[inline]
pub fn mid<A: Text, B: Number, C: Number>(t: A, start: B, length: C) -> FnText3<A, B, C> {
    FnText3("MID", t, start, length)
}

/// Return the input string with the first letter of each word converted to an 
/// uppercase letter and the rest of the letters in the word converted to 
/// lowercase.
/// Syntax: PROPER( T Text; )
///
/// Constraints:
/// None
///
/// Semantics:
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
/// See also: "LOWER", "UPPER", 
#[inline]
pub fn proper<A: Text>(t: A) -> FnText1<A> {
    FnText1("PROPER", t)
}

/// Returns text where an old text is substituted with a new text.
/// Syntax: REPLACE( T Text;; Start Number;; Count Number;; New Text; )
///
/// Constraints:
/// Start ≥ 1.
///
/// Semantics:
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
/// See also: "LEFT", "LEN", "MID", "RIGHT", "SUBSTITUTE", 
#[inline]
pub fn replace<A: Text, B: Number, C: Number, D: Text>(t: A, start: B, count: C, new: D) -> FnText4<A, B, C, D> {
    FnText4("REPLACE", t, start, count, new)
}

/// Return text repeated Count times.
/// Syntax: REPT( T Text;; Count Integer; )
///
/// Constraints:
/// Count ≥ 0
///
/// Semantics:
/// Returns text T repeated Count number of times; if Count is zero, an empty 
/// string is returned. If Count < 0, the result is Error.
///
/// See also: "LEFT", "MID", "RIGHT", "SUBSTITUTE", 
#[inline]
pub fn rept<A: Text, B: Number>(t: A, count: B) -> FnText2<A, B> {
    FnText2("REPT", t, count)
}

/// Return a selected number of text characters from the right.
/// Syntax: RIGHT( T Text; )
///
/// Constraints:
/// Length ≥ 0
///
/// Semantics:
/// Returns the Length number of characters of text T, starting from the right. 
/// If Length is omitted, it defaults to 1; otherwise, it computes Length = 
/// INT(Length). If T has fewer than Length characters, it returns T 
/// (unchanged). This means that if T is an empty string (which has length 0) 
/// or the parameter Length is 0, RIGHT() will always return an empty string. 
/// Note that if Length < 0, an Error is returned.
/// 
/// The results of this function may be normalization-sensitive. 4.2
///
/// See also: "INT", "LEFT", "LEN", "MID", 
#[inline]
pub fn right<A: Text>(t: A) -> FnText1<A> {
    FnText1("RIGHT", t)
}

/// Return a selected number of text characters from the right.
/// Syntax: RIGHT( T Text;[; Length Integer] )
///
/// Constraints:
/// Length ≥ 0
///
/// Semantics:
/// Returns the Length number of characters of text T, starting from the right. 
/// If Length is omitted, it defaults to 1; otherwise, it computes Length = 
/// INT(Length). If T has fewer than Length characters, it returns T 
/// (unchanged). This means that if T is an empty string (which has length 0) 
/// or the parameter Length is 0, RIGHT() will always return an empty string. 
/// Note that if Length < 0, an Error is returned.
/// 
/// The results of this function may be normalization-sensitive. 4.2
///
/// See also: "INT", "LEFT", "LEN", "MID", 
#[inline]
pub fn right_<A: Text, B: Number>(t: A, length: B) -> FnText2<A, B> {
    FnText2("RIGHT", t, length)
}

/// Return the starting position of a given text.
/// Syntax: SEARCH( Search Text;; T Text; )
///
/// Constraints:
/// Start ≥ 1
///
/// Semantics:
/// Returns the character position where Search is first found in T, when the 
/// search is started from character position Start. The match is not 
/// case-sensitive. Start is 1 if omitted. Returns an Error if text not found.
/// 
/// The values returned may vary depending upon the 
/// HOST-USE-REGULAR-EXPRESSIONS or HOST-USE-WILDCARDS properties. 3.4
///
/// See also: "EXACT", "FIND", 
#[inline]
pub fn search<A: Text, B: Text>(search: A, t: B) -> FnNumber2<A, B> {
    FnNumber2("SEARCH", search, t)
}

/// Return the starting position of a given text.
/// Syntax: SEARCH( Search Text;; T Text;[; Start Integer] )
///
/// Constraints:
/// Start ≥ 1
///
/// Semantics:
/// Returns the character position where Search is first found in T, when the 
/// search is started from character position Start. The match is not 
/// case-sensitive. Start is 1 if omitted. Returns an Error if text not found.
/// 
/// The values returned may vary depending upon the 
/// HOST-USE-REGULAR-EXPRESSIONS or HOST-USE-WILDCARDS properties. 3.4
///
/// See also: "EXACT", "FIND", 
#[inline]
pub fn search_<A: Text, B: Text, C: Number>(search: A, t: B, start: C) -> FnNumber3<A, B, C> {
    FnNumber3("SEARCH", search, t, start)
}

/// Returns text where an old text is substituted with a new text.
/// Syntax: SUBSTITUTE( T Text;; Old Text;; New Text; )
///
/// Constraints:
/// Which ≥ 1 (when provided)
///
/// Semantics:
/// Returns text T, but with text Old replaced by text New (when searching from 
/// the left). If Which is omitted, every occurrence of Old is replaced with 
/// New; if Which is provided, only that occurrence of Old is replaced by New 
/// (starting the count from 1). If there is no match, or if Old has length 0, 
/// the value of T is returned. Note that Old and New may have different 
/// lengths. If Which is present and Which < 1, returns Error.
///
/// See also: "LEFT", "LEN", "MID", "REPLACE", "RIGHT", 
#[inline]
pub fn substitute<A: Text, B: Text, C: Text>(t: A, old: B, new: C) -> FnText3<A, B, C> {
    FnText3("SUBSTITUTE", t, old, new)
}

/// Returns text where an old text is substituted with a new text.
/// Syntax: SUBSTITUTE( T Text;; Old Text;; New Text;[; Which Integer] )
///
/// Constraints:
/// Which ≥ 1 (when provided)
///
/// Semantics:
/// Returns text T, but with text Old replaced by text New (when searching from 
/// the left). If Which is omitted, every occurrence of Old is replaced with 
/// New; if Which is provided, only that occurrence of Old is replaced by New 
/// (starting the count from 1). If there is no match, or if Old has length 0, 
/// the value of T is returned. Note that Old and New may have different 
/// lengths. If Which is present and Which < 1, returns Error.
///
/// See also: "LEFT", "LEN", "MID", "REPLACE", "RIGHT", 
#[inline]
pub fn substitute_<A: Text, B: Text, C: Text, D: Number>(t: A, old: B, new: C, which: D) -> FnText4<A, B, C, D> {
    FnText4("SUBSTITUTE", t, old, new, which)
}

/// Return the text (if Text), else return 0-length Text value
/// Syntax: T( X Any; )
///
/// Constraints:
/// None
///
/// Semantics:
/// The type of (a dereferenced) X is examined; if it is of type Text, it is 
/// returned, else an empty string (Text value of zero length) is returned. 
/// This is not a type-conversion function; T(5) produces an empty string, not 
/// "5".
///
/// See also: "N", 
#[inline]
pub fn t<A: Any>(x: A) -> FnText1<A> {
    FnText1("T", x)
}

/// Return the value converted to a text.
/// Syntax: TEXT( X Scalar;; FormatCode Text; )
///
/// Constraints:
/// The FormatCode is a sequence of characters with an implementation-defined 
/// meaning.
///
/// Semantics:
/// Converts the value X to a Text according to the rules of a number format 
/// code passed as FormatCode and returns it.
///
/// See also: "N", "T", 
#[inline]
pub fn text<A: Scalar, B: Text>(x: A, format_code: B) -> FnText2<A, B> {
    FnText2("TEXT", x, format_code)
}

/// Remove leading and trailing spaces, and replace all internal multiple 
/// spaces with a single space.
/// Syntax: TRIM( T Text; )
///
/// Constraints:
/// None.
///
/// Semantics:
/// Takes T and removes all leading and trailing space. Any other sequence of 2 
/// or more spaces is replaced with a single space.
/// 
/// A space is one or more, HORIZONTAL TABULATION (U+0009), LINE FEED (U+000A), 
/// CARRIAGE RETURN (U+000D) or SPACE (U+0020) characters.
///
/// See also: "LEFT", "RIGHT", 
#[inline]
pub fn trim<A: Text>(t: A) -> FnText1<A> {
    FnText1("TRIM", t)
}

/// Return the character represented by the given numeric value according to 
/// the [UNICODE] Standard.
/// Syntax: UNICHAR( N Integer; )
///
/// Constraints:
/// N ≥ 0, N ≤ 1114111 (U+10FFFF)
///
/// Semantics:
/// Returns the character having the given numeric value as [UNICODE] code 
/// point.
/// Evaluators shall support values between 1 and 0xFFFF. Evaluators should 
/// allow N to be any [UNICODE] code point of type Graphic, Format or Control. 
/// Evaluators should implement UNICHAR such that UNICODE(UNICHAR(N)) returns N 
/// for any [UNICODE] code point N of type Graphic, Format or Control.
///
/// See also: "UNICODE", 
#[inline]
pub fn unichar<A: Number>(n: A) -> FnText1<A> {
    FnText1("UNICHAR", n)
}

/// Return the [UNICODE] code point corresponding to the first character of the 
/// text value.
/// Syntax: UNICODE( T Text; )
///
/// Constraints:
/// Length(T) > 0.
///
/// Semantics:
/// Returns the numeric value of the [UNICODE] code point of the first 
/// character of the given text T.
/// 
/// The results of this function may be normalization-sensitive. 4.2
///
/// See also: "UNICHAR", 
#[inline]
pub fn unicode<A: Text>(t: A) -> FnNumber1<A> {
    FnNumber1("UNICODE", t)
}

/// Return input string, but with all lowercase letters converted to uppercase 
/// letters.
/// Syntax: UPPER( T Text; )
///
/// Constraints:
/// None
///
/// Semantics:
/// Return input string, but with all lowercase letters converted to uppercase 
/// letters, as defined by §3.13 Default Case Algorithms, §4.2 Case-Normative 
/// and §5.18 Case Mappings of [UNICODE]. As with most functions, it is 
/// side-effect free (it does not modify the source values). All Evaluators 
/// shall convert a-z to A-Z.
///
/// Note:
/// As this function can be locale aware, results may be unexpected in certain 
/// cases, for example in a Turkish locale a lower case "i with dot" (LATIN 
/// SMALL LETTER I) U+0069 is converted to an upper case "I with dot" (LATIN 
/// CAPITAL LETTER I WITH DOT ABOVE, U+0130).
///
/// See also: "LOWER", "PROPER", 
#[inline]
pub fn upper<A: Text>(t: A) -> FnText1<A> {
    FnText1("UPPER", t)
}
