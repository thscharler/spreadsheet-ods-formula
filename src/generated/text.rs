
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
/// Table 33 - ASC
/// 
/// From Unicode Character (c)
/// 
/// To Unicode Character
/// 
/// Comment
/// 
/// U+30a1 ≤ c ≤ U+30aa
/// if c%2==0
/// 
/// (c - 0x30a2) / 2 + 0xff71
/// 
/// katakana a-o
/// 
/// U+30a1 ≤ c ≤ U+30aa
/// if c%2==1
/// 
/// (c - 0x30a1) / 2 + 0xff67
/// 
/// katakana small a-o
/// 
/// U+30ab ≤ c ≤ U+30c2
/// if c%2==1
/// 
/// (c - 0x30ab) / 2 + 0xff76
/// 
/// katakana ka-chi
/// 
/// U+30ab ≤ c ≤ U+30c2
/// if c%2==0
/// 
/// (c - 0x30ac) / 2 + 0xff76
/// followed by 0xff9e
/// 
/// katakana ga-dhi
/// 
/// U+30c3
/// 
/// 0xff6f
/// 
/// katakana small tsu
/// 
/// U+30c4 ≤ c ≤ U+30c9
/// if c%2==0
/// 
/// (c - 0x30c4) / 2 + 0xff82
/// 
/// katakana tsu-to
/// 
/// U+30c4 ≤ c ≤ U+30c9
/// if c%2==1
/// 
/// (c - 0x30c5) / 2 + 0xff82
/// followed by 0xff9e
/// 
/// katakana du-do
/// 
/// U+30ca ≤ c ≤ U+30ce
/// 
/// c - 0x30ca + 0xff85
/// 
/// katakana na-no
/// 
/// U+30cf ≤ c ≤ U+30dd
/// if c%3==0
/// 
/// (c - 0x30cf) / 3 + 0xff8a
/// 
/// katakana ha-ho
/// 
/// U+30cf ≤ c ≤ U+30dd
/// if c%3==1
/// 
/// (c - 0x30d0) / 3 + 0xff8a
/// followed by 0xff9e
/// 
/// katakana ba-bo
/// 
/// U+30cf ≤ c ≤ U+30dd
/// if c%3==2
/// 
/// (c - 0x30d1) / 3 + 0xff8a
/// followed by 0xff9f
/// 
/// katakana pa-po
/// 
/// U+30de ≤ c ≤ U+30e2
/// 
/// c - 0x30de + 0xff8f
/// 
/// katakana ma-mo
/// 
/// U+30e3 ≤ c ≤ U+30e8
/// if c%2==0
/// 
/// (c - 0x30e4) / 2 + 0xff94)
/// 
/// katakana ya-yo
/// 
/// U+30e3 ≤ c ≤ U+30e8
/// if c%2==1
/// 
/// (c - 0x30e3) / 2 + 0xff6c
/// 
/// katakana small ya-yo
/// 
/// U+30e9 ≤ c ≤ U+30ed
/// 
/// c - 0x30e9 + 0xff97
/// 
/// katakana ra-ro
/// 
/// U+30ef
/// 
/// U+ff9c
/// 
/// katakana wa
/// 
/// U+30f2
/// 
/// U+ff66
/// 
/// katakana wo
/// 
/// U+30f3
/// 
/// U+ff9d
/// 
/// katakana nn
/// 
/// U+ff01 <= c <= U+ff5e
/// 
/// c - 0xff01 + 0x0021
/// 
/// ASCII characters
/// 
/// U+2015
/// 
/// U+ff70
/// 
/// HORIZONTAL BAR => HALFWIDTH KATAKANA-HIRAGANA PROLONGED SOUND MARK
/// 
/// U+2018
/// 
/// U+0060
/// 
/// LEFT SINGLE QUOTATION MARK => GRAVE ACCENT
/// 
/// U+2019
/// 
/// U+0027
/// 
/// RIGHT SINGLE QUOTATION MARK => APOSTROPHE
/// 
/// U+201d
/// 
/// U+0022
/// 
/// RIGHT DOUBLE QUOTATION MARK => QUOTATION MARK
/// 
/// U+3001
/// 
/// U+ff64
/// 
/// IDEOGRAPHIC COMMA
/// 
/// U+3002
/// 
/// U+ff61
/// 
/// IDEOGRAPHIC FULL STOP
/// 
/// U+300c
/// 
/// U+ff62
/// 
/// LEFT CORNER BRACKET
/// 
/// U+300d
/// 
/// U+ff63
/// 
/// RIGHT CORNER BRACKET
/// 
/// U+309b
/// 
/// U+ff9e
/// 
/// KATAKANA-HIRAGANA VOICED SOUND MARK
/// 
/// U+309c
/// 
/// U+ff9f
/// 
/// KATAKANA-HIRAGANA SEMI-VOICED SOUND MARK
/// 
/// U+30fb
/// 
/// U+ff65
/// 
/// KATAKANA MIDDLE DOT
/// 
/// U+30fc
/// 
/// U+ff70
/// 
/// KATAKANA-HIRAGANA PROLONGED SOUND MARK
/// 
/// U+ffe5
/// 
/// U+005c
/// 
/// FULLWIDTH YEN SIGN => REVERSE SOLIDUS "\"
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
/// Table 34 - JIS
/// 
/// From Unicode Character (c)
/// 
/// To Unicode Character
/// 
/// Comment
/// 
/// U+0022
/// 
/// 0x201d
/// 
/// QUOTATION MARK => RIGHT DOUBLE QUOTATION MARK
/// This is an exception to the ASCII range that follows below.
/// 
/// U+005c
/// 
/// 0xffe5
/// 
/// REVERSE SOLIDUS "\" => FULLWIDTH YEN SIGN
/// (code-page 932 legacy, for details see ASC function)
/// This is an exception to the ASCII range that follows below.
/// 
/// U+0060
/// 
/// 0x2018
/// 
/// GRAVE ACCENT => LEFT SINGLE QUOTATION MARK
/// This is an exception to the ASCII range that follows below.
/// 
/// U+0027
/// 
/// 0x2019
/// 
/// APOSTROPHE => RIGHT SINGLE QUOTATION MARK
/// This is an exception to the ASCII range that follows below.
/// 
/// U+0021 ≤ c ≤= U+007e
/// 
/// c - 0x0021 + 0xff01
/// 
/// ASCII characters
/// 
/// U+ff66
/// 
/// 0x30f2
/// 
/// katakana wo
/// 
/// U+ff67 ≤ c ≤ U+ff6b
/// 
/// (c - 0xff67) * 2 + 0x30a1
/// 
/// katakana small a-o
/// 
/// U+ff6c ≤ c ≤ U+ff6e
/// 
/// (c - 0xff6c) * 2 + 0x30e3
/// 
/// katakana small ya-yo
/// 
/// U+ff6f
/// 
/// 0x30c3
/// 
/// katakana small tsu
/// 
/// U+ff71 ≤ c ≤ U+ff75
/// 
/// (c - 0xff71) * 2 + 0x30a2
/// 
/// katakana a-o
/// 
/// U+ff76 ≤ c ≤ U+ff81
/// followed by U+ff9e
/// 
/// (c - 0xff76) * 2 + 0x30ac
/// 
/// katakana ga-dsu
/// 
/// U+ff76 ≤ c ≤ U+ff81
/// not followed by U+ff9e
/// 
/// (c - 0xff76) * 2 + 0x30ab
/// 
/// katakana ka-chi
/// 
/// U+ff82 ≤ c ≤ U+ff84
/// followed by U+ff9e
/// 
/// (c - 0xff82) * 2 + 0x30c5
/// 
/// katakana du-do
/// 
/// U+ff82 ≤ c ≤ U+ff84
/// not followed by U+ff9e
/// 
/// (c - 0xff82) * 2 + 0x30c4
/// 
/// katakana tsu-to
/// 
/// U+ff85 ≤ c ≤ U+ff89
/// 
/// c - 0xff85 + 0x30ca
/// 
/// katakana na-no
/// 
/// U+ff8a ≤ c ≤ U+ff8e
/// followed by U+ff9e
/// 
/// (c - 0xff8a) * 3 + 0x30d0
/// 
/// katakana ba-bo
/// 
/// U+ff8a ≤ c ≤ U+ff8e
/// followed by U+ff9f
/// 
/// (c - 0xff8a) * 3 + 0x30d1
/// 
/// katakana pa-po
/// 
/// U+ff8a ≤ c ≤ U+ff8e
/// neither followed by U+ff9e nor U+ff9f
/// 
/// (c - 0xff8a) * 3 + 0x30cf
/// 
/// katakana ha-ho
/// 
/// U+ff8f ≤ c ≤ U+ff93
/// 
/// c - 0xff8f + 0x30de
/// 
/// katakana ma-mo
/// 
/// U+ff94 ≤ c ≤ U+ff96
/// 
/// (c - 0xff94) * 2 + 0x30e4
/// 
/// katakana ya-yo
/// 
/// U+ff97 ≤ c ≤ U+ff9b
/// 
/// c - 0xff97 + 0x30e9
/// 
/// katakana ra-ro
/// 
/// U+ff9c
/// 
/// U+30ef
/// 
/// katakana wa
/// 
/// U+ff9d
/// 
/// U+30f3
/// 
/// katakana nn
/// 
/// U+ff9e
/// 
/// U+309b
/// 
/// HALFWIDTH KATAKANA VOICED SOUND MARK => FULLWIDTH
/// 
/// U+ff9f
/// 
/// U+309c
/// 
/// HALFWIDTH KATAKANA SEMI-VOICED SOUND MARK => FULLWIDTH
/// 
/// U+ff70
/// 
/// U+30fc
/// 
/// HALFWIDTH KATAKANA-HIRAGANA PROLONGED SOUND MARK => FULLWIDTH
/// 
/// U+ff61
/// 
/// U+3002
/// 
/// HALFWIDTH IDEOGRAPHIC FULL STOP => FULLWIDTH
/// 
/// U+ff62
/// 
/// U+300c
/// 
/// HALFWIDTH LEFT CORNER BRACKET => FULLWIDTH
/// 
/// U+ff63
/// 
/// U+300d
/// 
/// HALFWIDTH RIGHT CORNER BRACKET => FULLWIDTH
/// 
/// U+ff64
/// 
/// U+3001
/// 
/// HALFWIDTH IDEOGRAPHIC COMMA => FULLWIDTH
/// 
/// U+ff65
/// 
/// U+30fb
/// 
/// HALFWIDTH KATAKANA MIDDLE DOT => FULLWIDTH
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
