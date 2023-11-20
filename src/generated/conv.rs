use crate::*;
#[allow(unused_imports)]
use crate::conv::*;

#[inline]
pub fn arabic<A: Text>(x: A) -> FnNumber1<A> {
    FnNumber1("ARABIC", x)
}

#[inline]
pub fn base<A: Number, B: Number>(x: A, radix: B) -> FnText2<A, B> {
    FnText2("BASE", x, radix)
}

#[inline]
pub fn base_<A: Number, B: Number, C: Number>(x: A, radix: B, minimum_length: C) -> FnText3<A, B, C> {
    FnText3("BASE", x, radix, minimum_length)
}

#[inline]
pub fn bin2dec<A: TextOrNumber>(x: A) -> FnNumber1<A> {
    FnNumber1("BIN2DEC", x)
}

#[inline]
pub fn bin2hex<A: TextOrNumber>(x: A) -> FnText1<A> {
    FnText1("BIN2HEX", x)
}

#[inline]
pub fn bin2hex_<A: TextOrNumber, B: Number>(x: A, digits: B) -> FnText2<A, B> {
    FnText2("BIN2HEX", x, digits)
}

#[inline]
pub fn bin2oct<A: TextOrNumber>(x: A) -> FnText1<A> {
    FnText1("BIN2OCT", x)
}

#[inline]
pub fn bin2oct_<A: TextOrNumber, B: Number>(x: A, digits: B) -> FnText2<A, B> {
    FnText2("BIN2OCT", x, digits)
}

#[inline]
pub fn dec2bin<A: TextOrNumber>(x: A) -> FnText1<A> {
    FnText1("DEC2BIN", x)
}

#[inline]
pub fn dec2bin_<A: TextOrNumber, B: Number>(x: A, digits: B) -> FnText2<A, B> {
    FnText2("DEC2BIN", x, digits)
}

#[inline]
pub fn dec2hex<A: TextOrNumber>(x: A) -> FnText1<A> {
    FnText1("DEC2HEX", x)
}

#[inline]
pub fn dec2hex_<A: TextOrNumber, B: Number>(x: A, digits: B) -> FnText2<A, B> {
    FnText2("DEC2HEX", x, digits)
}

#[inline]
pub fn dec2oct<A: TextOrNumber>(x: A) -> FnText1<A> {
    FnText1("DEC2OCT", x)
}

#[inline]
pub fn dec2oct_<A: TextOrNumber, B: Number>(x: A, digits: B) -> FnText2<A, B> {
    FnText2("DEC2OCT", x, digits)
}

#[inline]
pub fn decimal<A: Text, B: Number>(x: A, radix: B) -> FnNumber2<A, B> {
    FnNumber2("DECIMAL", x, radix)
}

#[inline]
pub fn hex2bin<A: TextOrNumber>(x: A) -> FnText1<A> {
    FnText1("HEX2BIN", x)
}

#[inline]
pub fn hex2bin_<A: TextOrNumber, B: Number>(x: A, digits: B) -> FnText2<A, B> {
    FnText2("HEX2BIN", x, digits)
}

#[inline]
pub fn hex2dec<A: TextOrNumber>(x: A) -> FnNumber1<A> {
    FnNumber1("HEX2DEC", x)
}

#[inline]
pub fn hex2oct<A: TextOrNumber>(x: A) -> FnText1<A> {
    FnText1("HEX2OCT", x)
}

#[inline]
pub fn hex2oct_<A: TextOrNumber, B: Number>(x: A, digits: B) -> FnText2<A, B> {
    FnText2("HEX2OCT", x, digits)
}

#[inline]
pub fn oct2bin<A: TextOrNumber>(x: A) -> FnText1<A> {
    FnText1("OCT2BIN", x)
}

#[inline]
pub fn oct2bin_<A: TextOrNumber, B: Number>(x: A, digits: B) -> FnText2<A, B> {
    FnText2("OCT2BIN", x, digits)
}

#[inline]
pub fn oct2dec<A: TextOrNumber>(x: A) -> FnNumber1<A> {
    FnNumber1("OCT2DEC", x)
}

#[inline]
pub fn oct2hex<A: TextOrNumber>(x: A) -> FnText1<A> {
    FnText1("OCT2HEX", x)
}

#[inline]
pub fn oct2hex_<A: TextOrNumber, B: Number>(x: A, digits: B) -> FnText2<A, B> {
    FnText2("OCT2HEX", x, digits)
}

#[inline]
pub fn roman<A: Number>(n: A) -> FnText1<A> {
    FnText1("ROMAN", n)
}

#[inline]
pub fn roman_<A: Number>(n: A, format: RomanStyle) -> FnText2<A, RomanStyle> {
    FnText2("ROMAN", n, format)
}
