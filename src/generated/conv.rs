//! 
//! These functions convert between different representations of numbers, such 
//! as between different bases and Roman numerals.
//! 
//! The base conversion functions xxx2BIN (such as DEC2BIN), xxx2OCT, and 
//! xxx2HEX functions return Text, while the xxx2DEC functions return Number. 
//! All of the xxx2yyy functions accept either Text or Number, though a Number 
//! is interpreted as the digits when printed in base 10. These are intended to 
//! support relatively small numbers, and have a somewhat convoluted interface 
//! and semantics, as described in their specifications. General base 
//! conversion capabilities are provided by BASE and DECIMAL.
//! 
//! As an argument for the HEX2xxx functions, a hexadecimal number is any 
//! string consisting solely of the characters "0","1" to "9", "a" to "f" and 
//! "A" to "F". The hexadecimal output of an xxx2HEX function shall be a string 
//! consisting solely of the characters "0","1" to "9" (U+0030 through U+0039), 
//! "a" to "f" (U+0061 through U+0066) and "A" to "F" (U+0041 through U+0046), 
//! and should be a string consisting solely of the characters "0","1" to "9" 
//! and "A" to "F". In both cases, the 40th bit (from the right) is considered 
//! a sign bit.

use crate::*;
#[allow(unused_imports)]
use crate::conv::*;

/// Convert Roman numerals to Number.
///
/// [documentfoundation->ARABIC](https://wiki.documentfoundation.org/Documentation/Calc_Functions/ARABIC)
///
/// __Syntax__: 
/// ```ods
///     ARABIC( X: Text )
/// ```
///
/// __Constraints__:
/// X shall contain Roman numerals, or an empty string.
///
/// __Semantics__:
/// Converts the Roman numeral to Number. This is the reverse of ROMAN; see 
/// ROMAN for the values of individual Roman numeral symbols. A Roman symbol to 
/// the left of a larger symbol (directly or indirectly) reduces the final 
/// value by the symbol amount, otherwise, it increases the final amount by the 
/// symbol's amount. Case is ignored.
/// 
/// The characters accepted are U+004D "M", U+0044 "D", U+0043 "C", U+004C "L", 
/// U+0058 "X", U+0056 "V", U+0049 "I", U+006D "m", U+0064 "d", U+0063 "c", 
/// U+006C "l", U+0078 "x", U+0076 "v", U+0069 "i" .
/// 
/// The following identity shall hold: ARABIC(ROMAN(x; any)) = x, when ROMAN(x; 
/// any) is not an Error.
/// 
/// If X is an empty string, 0 is returned.
///
/// __See also__: [crate::of::infix operator "&"()], [crate::of::roman()], 
#[inline]
pub fn arabic<A: Text>(x: A) -> FnNumber1<A> {
    FnNumber1("ARABIC", x)
}

/// Converts a number into a text representation with the given base.
///
/// [documentfoundation->BASE](https://wiki.documentfoundation.org/Documentation/Calc_Functions/BASE)
///
/// __Syntax__: 
/// ```ods
///     BASE( X: Integer; Radix: Integer )
/// ```
///
/// __Constraints__:
/// X ≥ 0, 2 ≤ Radix ≤ 36, MinimumLength ≥ 0
///
/// __Semantics__:
/// Converts number X into text that represents the value of X in base Radix. 
/// The symbols 0-9 (U+0030 through U+0039), then upper case A-Z (U+0041 
/// through U+005A) are used as digits. Thus, BASE(45745;36) returns “ZAP”.
/// 
/// If MinimumLength is not supplied, the generated text uses the smallest 
/// number of characters (i.e., it does not add leading 0s). If MinimumLength 
/// is supplied, and the resulting text would normally be smaller than 
/// MinimumLength, leading 0s are added to produce text exactly MinimumLength 
/// characters long. If the text is longer than the MinimumLength argument, the 
/// MinimumLength parameter is ignored.
///
/// __See also__: [crate::of::decimal()], [crate::of::base_()], 
#[inline]
pub fn base<A: Number, B: Number>(x: A, radix: B) -> FnText2<A, B> {
    FnText2("BASE", x, radix)
}

/// Converts a number into a text representation with the given base.
///
/// [documentfoundation->BASE](https://wiki.documentfoundation.org/Documentation/Calc_Functions/BASE)
///
/// __Syntax__: 
/// ```ods
///     BASE( X: Integer; Radix: Integer; MinimumLength: Integer )
/// ```
///
/// __Constraints__:
/// X ≥ 0, 2 ≤ Radix ≤ 36, MinimumLength ≥ 0
///
/// __Semantics__:
/// Converts number X into text that represents the value of X in base Radix. 
/// The symbols 0-9 (U+0030 through U+0039), then upper case A-Z (U+0041 
/// through U+005A) are used as digits. Thus, BASE(45745;36) returns “ZAP”.
/// 
/// If MinimumLength is not supplied, the generated text uses the smallest 
/// number of characters (i.e., it does not add leading 0s). If MinimumLength 
/// is supplied, and the resulting text would normally be smaller than 
/// MinimumLength, leading 0s are added to produce text exactly MinimumLength 
/// characters long. If the text is longer than the MinimumLength argument, the 
/// MinimumLength parameter is ignored.
///
/// __See also__: [crate::of::decimal()], [crate::of::base()], 
#[inline]
pub fn base_<A: Number, B: Number, C: Number>(x: A, radix: B, minimum_length: C) -> FnText3<A, B, C> {
    FnText3("BASE", x, radix, minimum_length)
}

/// Converts a binary (base 2) number (up to 10 digits) to its decimal 
/// equivalent
///
/// [documentfoundation->BIN2DEC](https://wiki.documentfoundation.org/Documentation/Calc_Functions/BIN2DEC)
///
/// __Syntax__: 
/// ```ods
///     BIN2DEC( X: TextOrNumber )
/// ```
///
/// __Constraints__:
/// X shall contain only binary digits (no space or other characters), and 
/// shall contain at least one binary digit. When considered as a Number, 
/// INT(X) = X. Evaluators may evaluate expressions where the digits in X are 
/// only 0 or 1, no more than 10 digits.
///
/// __Semantics__:
/// Converts given binary number into decimal equivalent, with the topmost 10th 
/// digit being the sign bit (using a two's-complement representation). If 
/// given Text, the text is considered a binary number representation. If given 
/// a Number, the digits of the number when printed as base 10 are considered 
/// the digits of the equivalently-represented binary number. It is 
/// implementation-defined what happens if given a Logical value; an evaluator 
/// may produce an Error, or it may convert the Logical to Number (per 
/// “Convert to Number”) and then process as a Number.
/// 
/// If any digits are 2 through 9, an evaluator shall return an Error. It is 
/// implementation-defined what happens if an evaluator is given an empty 
/// string; evaluators may return an Error or 0 in such cases.
///
/// __See also__: [crate::of::int()], 
#[inline]
pub fn bin2dec<A: TextOrNumber>(x: A) -> FnNumber1<A> {
    FnNumber1("BIN2DEC", x)
}

/// Converts a binary (base 2) number (10th bit is sign) to its hexadecimal 
/// equivalent
///
/// [documentfoundation->BIN2HEX](https://wiki.documentfoundation.org/Documentation/Calc_Functions/BIN2HEX)
///
/// __Syntax__: 
/// ```ods
///     BIN2HEX( X: TextOrNumber )
/// ```
///
/// __Constraints__:
/// X shall contain only binary digits (no space or other characters), and 
/// shall contain at least one binary digit. When considered as a Number, 
/// INT(X) = X. Evaluators may evaluate expressions where the digits in X are 
/// only 0 or 1, no more than 10 digits.
///
/// __Semantics__:
/// Converts given binary number into hexadecimal (base 16) equivalent. For 
/// input value X, the topmost 10th digit is considered the sign bit (using a 
/// two's-complement representation). If given Text, the text is considered a 
/// binary number representation. If given a Number, the digits of the number 
/// when printed as base 10 are considered the digits of the 
/// equivalently-represented binary number. It is implementation-defined what 
/// happens if given a Logical value; an evaluator may produce an Error, or it 
/// may convert the Logical to Number (per “Convert to Number”) and then 
/// process as a Number.
/// 
/// If any digits in X are 2 through 9, an evaluator shall return an Error. It 
/// is implementation-defined what happens if an evaluator is given an empty 
/// string; evaluators may return an Error or 0 in such cases.
/// 
/// The resulting value is a hexadecimal value, up to 10 hexadecimal digits, 
/// with the topmost bit (40th bit) being the sign bit and in two's-complement 
/// form. The digits A through F are in uppercase. If the input has its 10th 
/// bit on, the Digits argument is ignored; otherwise, the Digits indicates the 
/// number of digits in the output, with leading 0 digits added as necessary to 
/// bring it up to that number of digits. If there are more digits required 
/// than the Digits parameter specifies, the results are 
/// implementation-defined.
///
/// __See also__: [crate::of::int()], [crate::of::bin2hex_()], 
#[inline]
pub fn bin2hex<A: TextOrNumber>(x: A) -> FnText1<A> {
    FnText1("BIN2HEX", x)
}

/// Converts a binary (base 2) number (10th bit is sign) to its hexadecimal 
/// equivalent
///
/// [documentfoundation->BIN2HEX](https://wiki.documentfoundation.org/Documentation/Calc_Functions/BIN2HEX)
///
/// __Syntax__: 
/// ```ods
///     BIN2HEX( X: TextOrNumber; Digits: Number )
/// ```
///
/// __Constraints__:
/// X shall contain only binary digits (no space or other characters), and 
/// shall contain at least one binary digit. When considered as a Number, 
/// INT(X) = X. Evaluators may evaluate expressions where the digits in X are 
/// only 0 or 1, no more than 10 digits.
///
/// __Semantics__:
/// Converts given binary number into hexadecimal (base 16) equivalent. For 
/// input value X, the topmost 10th digit is considered the sign bit (using a 
/// two's-complement representation). If given Text, the text is considered a 
/// binary number representation. If given a Number, the digits of the number 
/// when printed as base 10 are considered the digits of the 
/// equivalently-represented binary number. It is implementation-defined what 
/// happens if given a Logical value; an evaluator may produce an Error, or it 
/// may convert the Logical to Number (per “Convert to Number”) and then 
/// process as a Number.
/// 
/// If any digits in X are 2 through 9, an evaluator shall return an Error. It 
/// is implementation-defined what happens if an evaluator is given an empty 
/// string; evaluators may return an Error or 0 in such cases.
/// 
/// The resulting value is a hexadecimal value, up to 10 hexadecimal digits, 
/// with the topmost bit (40th bit) being the sign bit and in two's-complement 
/// form. The digits A through F are in uppercase. If the input has its 10th 
/// bit on, the Digits argument is ignored; otherwise, the Digits indicates the 
/// number of digits in the output, with leading 0 digits added as necessary to 
/// bring it up to that number of digits. If there are more digits required 
/// than the Digits parameter specifies, the results are 
/// implementation-defined.
///
/// __See also__: [crate::of::int()], [crate::of::bin2hex()], 
#[inline]
pub fn bin2hex_<A: TextOrNumber, B: Number>(x: A, digits: B) -> FnText2<A, B> {
    FnText2("BIN2HEX", x, digits)
}

/// Converts a binary (base 2) number (10th bit is sign) to its octal (base 8) 
/// equivalent
///
/// [documentfoundation->BIN2OCT](https://wiki.documentfoundation.org/Documentation/Calc_Functions/BIN2OCT)
///
/// __Syntax__: 
/// ```ods
///     BIN2OCT( X: TextOrNumber )
/// ```
///
/// __Constraints__:
/// X shall contain only binary digits (no space or other characters), and 
/// shall contain at least one binary digit. When considered as a Number, 
/// INT(X) = X. Evaluators may evaluate expressions where the digits in X are 
/// only 0 or 1, no more than 10 digits.
///
/// __Semantics__:
/// Converts given binary number into octal (base 8) equivalent. For input 
/// value X, the topmost 10th digit is considered the sign bit (using a 
/// two's-complement representation). If given Text, the text is considered a 
/// binary number representation. If given a Number, the digits of the number 
/// when printed as base 10 are considered the digits of the 
/// equivalently-represented binary number. It is implementation-defined what 
/// happens if given a Logical value; an evaluator may produce an Error, or it 
/// may convert the Logical to Number (per “Convert to Number”) and then 
/// process as a Number.
/// 
/// If any digits in X are 2 through 9, an evaluator shall return an Error. It 
/// is implementation-defined what happens if an evaluator is given an empty 
/// string; evaluators may return an Error or 0 in such cases.
/// 
/// The resulting value is an octal value, up to 10 octal digits, with the 
/// topmost bit (30th bit) being the sign bit and in two's-complement form. If 
/// the input has its 10th bit on, the Digits argument is ignored; otherwise, 
/// the Digits indicates the number of digits in the output, with leading 0 
/// digits added as necessary to bring it up to that number of digits. If there 
/// are more digits than specified by the Digits parameter, its results are 
/// implementation-defined.
///
/// __See also__: [crate::of::int()], [crate::of::bin2oct_()], 
#[inline]
pub fn bin2oct<A: TextOrNumber>(x: A) -> FnText1<A> {
    FnText1("BIN2OCT", x)
}

/// Converts a binary (base 2) number (10th bit is sign) to its octal (base 8) 
/// equivalent
///
/// [documentfoundation->BIN2OCT](https://wiki.documentfoundation.org/Documentation/Calc_Functions/BIN2OCT)
///
/// __Syntax__: 
/// ```ods
///     BIN2OCT( X: TextOrNumber; Digits: Number )
/// ```
///
/// __Constraints__:
/// X shall contain only binary digits (no space or other characters), and 
/// shall contain at least one binary digit. When considered as a Number, 
/// INT(X) = X. Evaluators may evaluate expressions where the digits in X are 
/// only 0 or 1, no more than 10 digits.
///
/// __Semantics__:
/// Converts given binary number into octal (base 8) equivalent. For input 
/// value X, the topmost 10th digit is considered the sign bit (using a 
/// two's-complement representation). If given Text, the text is considered a 
/// binary number representation. If given a Number, the digits of the number 
/// when printed as base 10 are considered the digits of the 
/// equivalently-represented binary number. It is implementation-defined what 
/// happens if given a Logical value; an evaluator may produce an Error, or it 
/// may convert the Logical to Number (per “Convert to Number”) and then 
/// process as a Number.
/// 
/// If any digits in X are 2 through 9, an evaluator shall return an Error. It 
/// is implementation-defined what happens if an evaluator is given an empty 
/// string; evaluators may return an Error or 0 in such cases.
/// 
/// The resulting value is an octal value, up to 10 octal digits, with the 
/// topmost bit (30th bit) being the sign bit and in two's-complement form. If 
/// the input has its 10th bit on, the Digits argument is ignored; otherwise, 
/// the Digits indicates the number of digits in the output, with leading 0 
/// digits added as necessary to bring it up to that number of digits. If there 
/// are more digits than specified by the Digits parameter, its results are 
/// implementation-defined.
///
/// __See also__: [crate::of::int()], [crate::of::bin2oct()], 
#[inline]
pub fn bin2oct_<A: TextOrNumber, B: Number>(x: A, digits: B) -> FnText2<A, B> {
    FnText2("BIN2OCT", x, digits)
}

/// Converts a decimal number to base 2 (whose 10th bit is sign)
///
/// [documentfoundation->DEC2BIN](https://wiki.documentfoundation.org/Documentation/Calc_Functions/DEC2BIN)
///
/// __Syntax__: 
/// ```ods
///     DEC2BIN( X: TextOrNumber )
/// ```
///
/// __Constraints__:
/// X shall contain only decimal digits (no space or other characters), and 
/// shall contain at least one decimal digit. When considered as a Number, 
/// INT(X) = X. Evaluators may evaluate expressions where -512 ≤ X ≤ 511.
///
/// __Semantics__:
/// Converts given number into binary (base 2) equivalent. If given Text, the 
/// text is considered a decimal number representation, and may have a leading 
/// minus sign. It is implementation-defined what happens if given a Logical 
/// value; an evaluator may produce an Error, or it may convert the Logical to 
/// Number (per “Convert to Number”) and then process as a Number.
/// 
/// The resulting value is a binary value, up to 10 digits, with the topmost 
/// bit (10th bit) being the sign bit and in two's-complement form. If the 
/// input is negative, the Digits argument is ignored; otherwise, the Digits 
/// indicates the number of digits in the output, with leading 0 digits added 
/// as necessary to bring it up to that number of digits. If there are more 
/// digits than specified by the Digits parameter, the results are 
/// implementation-defined.
///
/// __See also__: [crate::of::int()], [crate::of::dec2bin_()], 
#[inline]
pub fn dec2bin<A: TextOrNumber>(x: A) -> FnText1<A> {
    FnText1("DEC2BIN", x)
}

/// Converts a decimal number to base 2 (whose 10th bit is sign)
///
/// [documentfoundation->DEC2BIN](https://wiki.documentfoundation.org/Documentation/Calc_Functions/DEC2BIN)
///
/// __Syntax__: 
/// ```ods
///     DEC2BIN( X: TextOrNumber; Digits: Number )
/// ```
///
/// __Constraints__:
/// X shall contain only decimal digits (no space or other characters), and 
/// shall contain at least one decimal digit. When considered as a Number, 
/// INT(X) = X. Evaluators may evaluate expressions where -512 ≤ X ≤ 511.
///
/// __Semantics__:
/// Converts given number into binary (base 2) equivalent. If given Text, the 
/// text is considered a decimal number representation, and may have a leading 
/// minus sign. It is implementation-defined what happens if given a Logical 
/// value; an evaluator may produce an Error, or it may convert the Logical to 
/// Number (per “Convert to Number”) and then process as a Number.
/// 
/// The resulting value is a binary value, up to 10 digits, with the topmost 
/// bit (10th bit) being the sign bit and in two's-complement form. If the 
/// input is negative, the Digits argument is ignored; otherwise, the Digits 
/// indicates the number of digits in the output, with leading 0 digits added 
/// as necessary to bring it up to that number of digits. If there are more 
/// digits than specified by the Digits parameter, the results are 
/// implementation-defined.
///
/// __See also__: [crate::of::int()], [crate::of::dec2bin()], 
#[inline]
pub fn dec2bin_<A: TextOrNumber, B: Number>(x: A, digits: B) -> FnText2<A, B> {
    FnText2("DEC2BIN", x, digits)
}

/// Converts a decimal number to base 16 (whose 40th bit is sign)
///
/// [documentfoundation->DEC2HEX](https://wiki.documentfoundation.org/Documentation/Calc_Functions/DEC2HEX)
///
/// __Syntax__: 
/// ```ods
///     DEC2HEX( X: TextOrNumber )
/// ```
///
/// __Constraints__:
/// X shall contain only decimal digits (no space or other characters), and 
/// shall contain at least one decimal digit. When considered as a Number, 
/// INT(X) = X. Evaluators may evaluate expressions where -239≤ X ≤ 239-1.
///
/// __Semantics__:
/// Converts given number into hexadecimal (base 16) equivalent. If given Text, 
/// the text is considered a decimal number representation, and may have a 
/// leading minus sign. It is implementation-defined what happens if given a 
/// Logical value; an evaluator may produce an Error, or it may convert the 
/// Logical to Number (per “Convert to Number”) and then process as a 
/// Number.
/// 
/// The resulting value is a hexadecimal value, up to 10 digits, with the 
/// topmost bit (40th bit) being the sign bit and in two's-complement form. If 
/// the input is negative, the Digits argument is ignored; otherwise, the 
/// Digits indicates the number of digits in the output, with leading 0 digits 
/// added as necessary to bring it up to that number of digits. If there are 
/// more digits than specified by the Digits parameter, the results are 
/// implementation-defined.
///
/// __See also__: [crate::of::int()], [crate::of::dec2hex_()], 
#[inline]
pub fn dec2hex<A: TextOrNumber>(x: A) -> FnText1<A> {
    FnText1("DEC2HEX", x)
}

/// Converts a decimal number to base 16 (whose 40th bit is sign)
///
/// [documentfoundation->DEC2HEX](https://wiki.documentfoundation.org/Documentation/Calc_Functions/DEC2HEX)
///
/// __Syntax__: 
/// ```ods
///     DEC2HEX( X: TextOrNumber; Digits: Number )
/// ```
///
/// __Constraints__:
/// X shall contain only decimal digits (no space or other characters), and 
/// shall contain at least one decimal digit. When considered as a Number, 
/// INT(X) = X. Evaluators may evaluate expressions where -239≤ X ≤ 239-1.
///
/// __Semantics__:
/// Converts given number into hexadecimal (base 16) equivalent. If given Text, 
/// the text is considered a decimal number representation, and may have a 
/// leading minus sign. It is implementation-defined what happens if given a 
/// Logical value; an evaluator may produce an Error, or it may convert the 
/// Logical to Number (per “Convert to Number”) and then process as a 
/// Number.
/// 
/// The resulting value is a hexadecimal value, up to 10 digits, with the 
/// topmost bit (40th bit) being the sign bit and in two's-complement form. If 
/// the input is negative, the Digits argument is ignored; otherwise, the 
/// Digits indicates the number of digits in the output, with leading 0 digits 
/// added as necessary to bring it up to that number of digits. If there are 
/// more digits than specified by the Digits parameter, the results are 
/// implementation-defined.
///
/// __See also__: [crate::of::int()], [crate::of::dec2hex()], 
#[inline]
pub fn dec2hex_<A: TextOrNumber, B: Number>(x: A, digits: B) -> FnText2<A, B> {
    FnText2("DEC2HEX", x, digits)
}

/// Converts a decimal number to base 8 (whose 30th bit is sign)
///
/// [documentfoundation->DEC2OCT](https://wiki.documentfoundation.org/Documentation/Calc_Functions/DEC2OCT)
///
/// __Syntax__: 
/// ```ods
///     DEC2OCT( X: TextOrNumber )
/// ```
///
/// __Constraints__:
/// X shall contain only decimal digits (no space or other characters), and 
/// shall contain at least one decimal digit. When considered as a Number, 
/// INT(X) = X. Evaluators may evaluate expressions where -229≤ X ≤ 229-1.
///
/// __Semantics__:
/// Converts given number into octal (base 8) equivalent. If given Text, the 
/// text is considered a decimal number representation, and may have a leading 
/// minus sign. It is implementation-defined what happens if given a Logical 
/// value; an evaluator may produce an Error, or it may convert the Logical to 
/// Number (per “Convert to Number”) and then process as a Number.
/// 
/// The resulting value is a octal value, up to 10 digits, with the topmost bit 
/// (30th bit) being the sign bit and in two's-complement form. If the input is 
/// negative, the Digits argument is ignored; otherwise, the Digits indicates 
/// the number of digits in the output, with leading 0 digits added as 
/// necessary to bring it up to that number of digits. If there are more digits 
/// than specified by the Digits parameter, the results are 
/// implementation-defined.
///
/// __See also__: [crate::of::int()], [crate::of::oct2dec()], [crate::of::dec2oct_()], 
#[inline]
pub fn dec2oct<A: TextOrNumber>(x: A) -> FnText1<A> {
    FnText1("DEC2OCT", x)
}

/// Converts a decimal number to base 8 (whose 30th bit is sign)
///
/// [documentfoundation->DEC2OCT](https://wiki.documentfoundation.org/Documentation/Calc_Functions/DEC2OCT)
///
/// __Syntax__: 
/// ```ods
///     DEC2OCT( X: TextOrNumber; Digits: Number )
/// ```
///
/// __Constraints__:
/// X shall contain only decimal digits (no space or other characters), and 
/// shall contain at least one decimal digit. When considered as a Number, 
/// INT(X) = X. Evaluators may evaluate expressions where -229≤ X ≤ 229-1.
///
/// __Semantics__:
/// Converts given number into octal (base 8) equivalent. If given Text, the 
/// text is considered a decimal number representation, and may have a leading 
/// minus sign. It is implementation-defined what happens if given a Logical 
/// value; an evaluator may produce an Error, or it may convert the Logical to 
/// Number (per “Convert to Number”) and then process as a Number.
/// 
/// The resulting value is a octal value, up to 10 digits, with the topmost bit 
/// (30th bit) being the sign bit and in two's-complement form. If the input is 
/// negative, the Digits argument is ignored; otherwise, the Digits indicates 
/// the number of digits in the output, with leading 0 digits added as 
/// necessary to bring it up to that number of digits. If there are more digits 
/// than specified by the Digits parameter, the results are 
/// implementation-defined.
///
/// __See also__: [crate::of::int()], [crate::of::oct2dec()], [crate::of::dec2oct()], 
#[inline]
pub fn dec2oct_<A: TextOrNumber, B: Number>(x: A, digits: B) -> FnText2<A, B> {
    FnText2("DEC2OCT", x, digits)
}

/// Converts text representing a number in a given base into a base 10 number.
///
/// [documentfoundation->DECIMAL](https://wiki.documentfoundation.org/Documentation/Calc_Functions/DECIMAL)
///
/// __Syntax__: 
/// ```ods
///     DECIMAL( X: Text; Radix: Integer )
/// ```
///
/// __Constraints__:
/// 2 ≤ Radix ≤ 36
///
/// __Semantics__:
/// Converts text X in base Radix to a Number. Uppercase letters (U+0041 
/// through U+005A) and lowercase letters (U+0061 through U+007A) are both 
/// accepted as equivalent if Radix > 10. Thus, DECIMAL("zap";36) and 
/// DECIMAL("ZAP";36) both compute 45745.
/// 
/// An Error is returned if X has characters that do not belong in base Radix. 
/// However, leading spaces and tabs in X are always ignored. If Radix is 16, a 
/// leading regular expression “0?\[Xx\]” is ignored, as is a trailing 
/// letter H or h. If Radix is 2, the letter b or B at the end is ignored (if 
/// present).
///
/// __See also__: [crate::of::base()], 
#[inline]
pub fn decimal<A: Text, B: Number>(x: A, radix: B) -> FnNumber2<A, B> {
    FnNumber2("DECIMAL", x, radix)
}

/// Converts a hexadecimal number (40th bit is sign) to base 2 (whose 10th bit 
/// is sign)
///
/// [documentfoundation->HEX2BIN](https://wiki.documentfoundation.org/Documentation/Calc_Functions/HEX2BIN)
///
/// __Syntax__: 
/// ```ods
///     HEX2BIN( X: TextOrNumber )
/// ```
///
/// __Constraints__:
/// X shall contain only hexadecimal digits (no space or other characters), and 
/// shall contain at least one hexadecimal digit. When considered as a Number, 
/// INT(X) = X. Evaluators may evaluate expressions where X is considered in 
/// base 10, -512 ≤ X ≤ 511.
///
/// __Semantics__:
/// Converts given hexadecimal number into binary (base 2) equivalent. If given 
/// Text, the text is considered a hexadecimal number representation; if its 
/// 40th bit is 1, it is considered a negative number. It is 
/// implementation-defined what happens if given a Logical value; an evaluator 
/// may produce an Error, or it may convert the Logical to Number (per 
/// “Convert to Number”) and then process as a Number.
/// 
/// The resulting value is a binary value, up to 10 digits, with the topmost 
/// bit (10th bit) being the sign bit and in two's-complement form. If the 
/// input is negative (40th bit is 1), the Digits argument is ignored; 
/// otherwise, the Digits indicates the number of digits in the output, with 
/// leading 0 digits added as necessary to bring it up to that number of 
/// digits. If there are more digits than specified by the Digits parameter, 
/// the results are implementation-defined.
///
/// __See also__: [crate::of::int()], [crate::of::hex2bin_()], 
#[inline]
pub fn hex2bin<A: TextOrNumber>(x: A) -> FnText1<A> {
    FnText1("HEX2BIN", x)
}

/// Converts a hexadecimal number (40th bit is sign) to base 2 (whose 10th bit 
/// is sign)
///
/// [documentfoundation->HEX2BIN](https://wiki.documentfoundation.org/Documentation/Calc_Functions/HEX2BIN)
///
/// __Syntax__: 
/// ```ods
///     HEX2BIN( X: TextOrNumber; Digits: Number )
/// ```
///
/// __Constraints__:
/// X shall contain only hexadecimal digits (no space or other characters), and 
/// shall contain at least one hexadecimal digit. When considered as a Number, 
/// INT(X) = X. Evaluators may evaluate expressions where X is considered in 
/// base 10, -512 ≤ X ≤ 511.
///
/// __Semantics__:
/// Converts given hexadecimal number into binary (base 2) equivalent. If given 
/// Text, the text is considered a hexadecimal number representation; if its 
/// 40th bit is 1, it is considered a negative number. It is 
/// implementation-defined what happens if given a Logical value; an evaluator 
/// may produce an Error, or it may convert the Logical to Number (per 
/// “Convert to Number”) and then process as a Number.
/// 
/// The resulting value is a binary value, up to 10 digits, with the topmost 
/// bit (10th bit) being the sign bit and in two's-complement form. If the 
/// input is negative (40th bit is 1), the Digits argument is ignored; 
/// otherwise, the Digits indicates the number of digits in the output, with 
/// leading 0 digits added as necessary to bring it up to that number of 
/// digits. If there are more digits than specified by the Digits parameter, 
/// the results are implementation-defined.
///
/// __See also__: [crate::of::int()], [crate::of::hex2bin()], 
#[inline]
pub fn hex2bin_<A: TextOrNumber, B: Number>(x: A, digits: B) -> FnText2<A, B> {
    FnText2("HEX2BIN", x, digits)
}

/// Converts a hexadecimal number (40th bit is sign) to decimal
///
/// [documentfoundation->HEX2DEC](https://wiki.documentfoundation.org/Documentation/Calc_Functions/HEX2DEC)
///
/// __Syntax__: 
/// ```ods
///     HEX2DEC( X: TextOrNumber )
/// ```
///
/// __Constraints__:
/// X shall contain only hexadecimal digits (no space or other characters), and 
/// shall contain at least one hexadecimal digit. When considered as a Number, 
/// INT(X) = X. Evaluators may evaluate expressions where X shall have 1 though 
/// 10 (inclusive) hexadecimal digits.
///
/// __Semantics__:
/// Converts given hexadecimal number into decimal equivalent. If given Text, 
/// the text is considered a hexadecimal number representation. If X's 40th bit 
/// is 1, it is considered a negative number. It is implementation-defined what 
/// happens if given a Logical value; an evaluator may produce an Error, or it 
/// may convert the Logical to Number (per “Convert to Number”) and then 
/// process as a Number.
/// 
/// The resulting value is a decimal number.
///
/// __See also__: [crate::of::int()], 
#[inline]
pub fn hex2dec<A: TextOrNumber>(x: A) -> FnNumber1<A> {
    FnNumber1("HEX2DEC", x)
}

/// Converts a hexadecimal number (40th bit is sign) to base 8 (whose 30th bit 
/// is sign)
///
/// [documentfoundation->HEX2OCT](https://wiki.documentfoundation.org/Documentation/Calc_Functions/HEX2OCT)
///
/// __Syntax__: 
/// ```ods
///     HEX2OCT( X: TextOrNumber )
/// ```
///
/// __Constraints__:
/// X shall contain hexadecimal digits (no spaces or other characters), and 
/// shall contain at least one hexadecimal digit. When considered as Number, 
/// INT(X) = X. Evaluators may evaluate expressions where X has 1 to 10 
/// (inclusive) hexadecimal digits, base 10 value of X is -2 29 < X < 2 29 -1.
///
/// __Semantics__:
/// Converts given hexadecimal number into octal (base 8) equivalent. If given 
/// Text, the text is considered a hexadecimal number representation; if its 
/// 40th bit is 1, it is considered a negative number. It is 
/// implementation-defined what happens if given a Logical value; an evaluator 
/// may produce an Error, or it may convert the Logical to Number (per 
/// “Convert to Number”) and then process as a Number.
/// 
/// The resulting value is an octal value, up to 10 digits, with the topmost 
/// bit (10th bit) being the sign bit and in two's-complement form. If the 
/// input is negative (40th bit is 1), the Digits argument is ignored; 
/// otherwise, the Digits indicates the number of digits in the output, with 
/// leading 0 digits added as necessary to bring it up to that number of 
/// digits. If there are more digits than specified by the Digits parameter, 
/// the results are implementation-defined.
///
/// __See also__: [crate::of::int()], [crate::of::hex2oct_()], 
#[inline]
pub fn hex2oct<A: TextOrNumber>(x: A) -> FnText1<A> {
    FnText1("HEX2OCT", x)
}

/// Converts a hexadecimal number (40th bit is sign) to base 8 (whose 30th bit 
/// is sign)
///
/// [documentfoundation->HEX2OCT](https://wiki.documentfoundation.org/Documentation/Calc_Functions/HEX2OCT)
///
/// __Syntax__: 
/// ```ods
///     HEX2OCT( X: TextOrNumber; Digits: Number )
/// ```
///
/// __Constraints__:
/// X shall contain hexadecimal digits (no spaces or other characters), and 
/// shall contain at least one hexadecimal digit. When considered as Number, 
/// INT(X) = X. Evaluators may evaluate expressions where X has 1 to 10 
/// (inclusive) hexadecimal digits, base 10 value of X is -2 29 < X < 2 29 -1.
///
/// __Semantics__:
/// Converts given hexadecimal number into octal (base 8) equivalent. If given 
/// Text, the text is considered a hexadecimal number representation; if its 
/// 40th bit is 1, it is considered a negative number. It is 
/// implementation-defined what happens if given a Logical value; an evaluator 
/// may produce an Error, or it may convert the Logical to Number (per 
/// “Convert to Number”) and then process as a Number.
/// 
/// The resulting value is an octal value, up to 10 digits, with the topmost 
/// bit (10th bit) being the sign bit and in two's-complement form. If the 
/// input is negative (40th bit is 1), the Digits argument is ignored; 
/// otherwise, the Digits indicates the number of digits in the output, with 
/// leading 0 digits added as necessary to bring it up to that number of 
/// digits. If there are more digits than specified by the Digits parameter, 
/// the results are implementation-defined.
///
/// __See also__: [crate::of::int()], [crate::of::hex2oct()], 
#[inline]
pub fn hex2oct_<A: TextOrNumber, B: Number>(x: A, digits: B) -> FnText2<A, B> {
    FnText2("HEX2OCT", x, digits)
}

/// Converts an octal number (30th bit is sign) to base 2 (whose 10th bit is 
/// sign)
///
/// [documentfoundation->OCT2BIN](https://wiki.documentfoundation.org/Documentation/Calc_Functions/OCT2BIN)
///
/// __Syntax__: 
/// ```ods
///     OCT2BIN( X: TextOrNumber )
/// ```
///
/// __Constraints__:
/// X shall contain only octal digits (no space or other characters), and shall 
/// contain at least one octal digit. When considered as a Number, INT(X) = X. 
/// Evaluators may evaluate expressions where X is considered in base 10, -512 
/// ≤ X ≤ 511.
///
/// __Semantics__:
/// Converts given octal (base 8) number into binary (base 2) equivalent. If 
/// given Text, the text is considered an octal number representation; if its 
/// 30th bit is 1, it is considered a negative number. It is 
/// implementation-defined what happens if given a Logical value; an evaluator 
/// may produce an Error, or it may convert the Logical to Number (per 
/// “Convert to Number”) and then process as a Number.
/// 
/// The resulting value is a binary value, up to 10 digits, with the topmost 
/// bit (10th bit) being the sign bit and in two's-complement form. If the 
/// input is negative (30th bit is 1), the Digits argument is ignored; 
/// otherwise, the Digits indicates the number of digits in the output, with 
/// leading 0 digits added as necessary to bring it up to that number of 
/// digits. If there are more digits than specified by the Digits parameter, 
/// the results are implementation-defined.
///
/// __See also__: [crate::of::int()], [crate::of::oct2bin_()], 
#[inline]
pub fn oct2bin<A: TextOrNumber>(x: A) -> FnText1<A> {
    FnText1("OCT2BIN", x)
}

/// Converts an octal number (30th bit is sign) to base 2 (whose 10th bit is 
/// sign)
///
/// [documentfoundation->OCT2BIN](https://wiki.documentfoundation.org/Documentation/Calc_Functions/OCT2BIN)
///
/// __Syntax__: 
/// ```ods
///     OCT2BIN( X: TextOrNumber; Digits: Number )
/// ```
///
/// __Constraints__:
/// X shall contain only octal digits (no space or other characters), and shall 
/// contain at least one octal digit. When considered as a Number, INT(X) = X. 
/// Evaluators may evaluate expressions where X is considered in base 10, -512 
/// ≤ X ≤ 511.
///
/// __Semantics__:
/// Converts given octal (base 8) number into binary (base 2) equivalent. If 
/// given Text, the text is considered an octal number representation; if its 
/// 30th bit is 1, it is considered a negative number. It is 
/// implementation-defined what happens if given a Logical value; an evaluator 
/// may produce an Error, or it may convert the Logical to Number (per 
/// “Convert to Number”) and then process as a Number.
/// 
/// The resulting value is a binary value, up to 10 digits, with the topmost 
/// bit (10th bit) being the sign bit and in two's-complement form. If the 
/// input is negative (30th bit is 1), the Digits argument is ignored; 
/// otherwise, the Digits indicates the number of digits in the output, with 
/// leading 0 digits added as necessary to bring it up to that number of 
/// digits. If there are more digits than specified by the Digits parameter, 
/// the results are implementation-defined.
///
/// __See also__: [crate::of::int()], [crate::of::oct2bin()], 
#[inline]
pub fn oct2bin_<A: TextOrNumber, B: Number>(x: A, digits: B) -> FnText2<A, B> {
    FnText2("OCT2BIN", x, digits)
}

/// Converts an octal number (30th bit is sign) to decimal
///
/// [documentfoundation->OCT2DEC](https://wiki.documentfoundation.org/Documentation/Calc_Functions/OCT2DEC)
///
/// __Syntax__: 
/// ```ods
///     OCT2DEC( X: TextOrNumber )
/// ```
///
/// __Constraints__:
/// X shall contain only octal digits (no space or other characters), and shall 
/// contain at least one octal digit. When considered as a Number, INT(X) = X. 
/// Evaluators may evaluate expressions where X shall have 1 though 10 
/// (inclusive) octal digits.
///
/// __Semantics__:
/// Converts given octal number into decimal equivalent. If given Text, the 
/// text is considered a octal number representation. If X's 30th bit is 1, it 
/// is considered a negative number. It is implementation-defined what happens 
/// if given a Logical value; an evaluator may produce an Error, or it may 
/// convert the Logical to Number (per “Convert to Number”) and then 
/// process as a Number.
/// 
/// The resulting value is a decimal number.
///
/// __See also__: [crate::of::int()], 
#[inline]
pub fn oct2dec<A: TextOrNumber>(x: A) -> FnNumber1<A> {
    FnNumber1("OCT2DEC", x)
}

/// Converts an octal number (30th bit is sign) to hexadecimal (whose 40th bit 
/// is sign)
///
/// [documentfoundation->OCT2HEX](https://wiki.documentfoundation.org/Documentation/Calc_Functions/OCT2HEX)
///
/// __Syntax__: 
/// ```ods
///     OCT2HEX( X: TextOrNumber )
/// ```
///
/// __Constraints__:
/// X shall contain only octal digits (no space or other characters), and shall 
/// contain at least one octal digit. When considered as a Number, INT(X) = X. 
/// Evaluators may evaluate expressions where X shall have 1 to 10 (inclusive) 
/// octal digits.
///
/// __Semantics__:
/// Converts given octal (base 8) number into hexadecimal (base 16) equivalent. 
/// If given Text, the text is considered an octal number representation; if 
/// its 30th bit is 1, it is considered a negative number. It is 
/// implementation-defined what happens if given a Logical value; an evaluator 
/// may produce an Error, or it may convert the Logical to Number (per 
/// “Convert to Number”) and then process as a Number.
/// 
/// The resulting value is a hexadecimal value, up to 10 digits, with the 
/// topmost bit (40th bit) being the sign bit and in two's-complement form. If 
/// the input is negative (30th bit is 1), the Digits argument is ignored; 
/// otherwise, the Digits indicates the number of digits in the output, with 
/// leading 0 digits added as necessary to bring it up to that number of 
/// digits. If there are more digits than specified by the Digits parameter, 
/// the results are implementation-defined.
///
/// __See also__: [crate::of::int()], [crate::of::oct2hex_()], 
#[inline]
pub fn oct2hex<A: TextOrNumber>(x: A) -> FnText1<A> {
    FnText1("OCT2HEX", x)
}

/// Converts an octal number (30th bit is sign) to hexadecimal (whose 40th bit 
/// is sign)
///
/// [documentfoundation->OCT2HEX](https://wiki.documentfoundation.org/Documentation/Calc_Functions/OCT2HEX)
///
/// __Syntax__: 
/// ```ods
///     OCT2HEX( X: TextOrNumber; Digits: Number )
/// ```
///
/// __Constraints__:
/// X shall contain only octal digits (no space or other characters), and shall 
/// contain at least one octal digit. When considered as a Number, INT(X) = X. 
/// Evaluators may evaluate expressions where X shall have 1 to 10 (inclusive) 
/// octal digits.
///
/// __Semantics__:
/// Converts given octal (base 8) number into hexadecimal (base 16) equivalent. 
/// If given Text, the text is considered an octal number representation; if 
/// its 30th bit is 1, it is considered a negative number. It is 
/// implementation-defined what happens if given a Logical value; an evaluator 
/// may produce an Error, or it may convert the Logical to Number (per 
/// “Convert to Number”) and then process as a Number.
/// 
/// The resulting value is a hexadecimal value, up to 10 digits, with the 
/// topmost bit (40th bit) being the sign bit and in two's-complement form. If 
/// the input is negative (30th bit is 1), the Digits argument is ignored; 
/// otherwise, the Digits indicates the number of digits in the output, with 
/// leading 0 digits added as necessary to bring it up to that number of 
/// digits. If there are more digits than specified by the Digits parameter, 
/// the results are implementation-defined.
///
/// __See also__: [crate::of::int()], [crate::of::oct2hex()], 
#[inline]
pub fn oct2hex_<A: TextOrNumber, B: Number>(x: A, digits: B) -> FnText2<A, B> {
    FnText2("OCT2HEX", x, digits)
}

/// Convert to Roman numerals
///
/// [documentfoundation->ROMAN](https://wiki.documentfoundation.org/Documentation/Calc_Functions/ROMAN)
///
/// __Syntax__: 
/// ```ods
///     ROMAN( N: Integer )
/// ```
///
/// __Constraints__:
/// N ≥ 0, N < 4000, 0 ≤ Format ≤ 4, ISLOGICAL(1) or 
/// NOT(ISLOGICAL(Format))
///
/// __Semantics__:
/// Return the Roman numeral representation of N. Format specifies the level of 
/// conciseness, and defaults to 0, the classic representation, with larger 
/// numbers requiring increasing conciseness.
/// 
/// To support legacy documents, evaluators with Logical types that are 
/// distinct from Number may accept the format parameter as a scalar, and 
/// accept TRUE specifying format 0, and FALSE specifying format 4.
/// 
/// The following identity shall hold: ARABIC(ROMAN(x; any)) = x, when ROMAN(x; 
/// any) is not an Error.
/// 
/// If N is 0, an empty string is returned.
/// 
/// Table 31 - ROMAN lists the values of individual roman numerals; roman 
/// numerals that precede (directly or indirectly) a larger-valued roman number 
/// subtract their value from the final value.
/// 
/// Evaluators that accept 0 as a value of N should return the string “0”. 
/// Evaluators that accept negative values of N should include a negative sign 
/// (“-”) as the first character.
///
/// __See also__: [crate::of::infix operator "&"()], [crate::of::islogical()], [crate::of::arabic()], [crate::of::roman_()], 
#[inline]
pub fn roman<A: Number>(n: A) -> FnText1<A> {
    FnText1("ROMAN", n)
}

/// Convert to Roman numerals
///
/// [documentfoundation->ROMAN](https://wiki.documentfoundation.org/Documentation/Calc_Functions/ROMAN)
///
/// __Syntax__: 
/// ```ods
///     ROMAN( N: Integer; Format: Integer )
/// ```
///
/// __Constraints__:
/// N ≥ 0, N < 4000, 0 ≤ Format ≤ 4, ISLOGICAL(1) or 
/// NOT(ISLOGICAL(Format))
///
/// __Semantics__:
/// Return the Roman numeral representation of N. Format specifies the level of 
/// conciseness, and defaults to 0, the classic representation, with larger 
/// numbers requiring increasing conciseness.
/// 
/// To support legacy documents, evaluators with Logical types that are 
/// distinct from Number may accept the format parameter as a scalar, and 
/// accept TRUE specifying format 0, and FALSE specifying format 4.
/// 
/// The following identity shall hold: ARABIC(ROMAN(x; any)) = x, when ROMAN(x; 
/// any) is not an Error.
/// 
/// If N is 0, an empty string is returned.
/// 
/// Table 31 - ROMAN lists the values of individual roman numerals; roman 
/// numerals that precede (directly or indirectly) a larger-valued roman number 
/// subtract their value from the final value.
/// 
/// Evaluators that accept 0 as a value of N should return the string “0”. 
/// Evaluators that accept negative values of N should include a negative sign 
/// (“-”) as the first character.
///
/// __See also__: [crate::of::infix operator "&"()], [crate::of::islogical()], [crate::of::arabic()], [crate::of::roman()], 
#[inline]
pub fn roman_<A: Number>(n: A, format: RomanStyle) -> FnText2<A, RomanStyle> {
    FnText2("ROMAN", n, format)
}
