use crate::*;
#[allow(unused_imports)]
use crate::text::*;

#[inline]
pub fn asc<A: Text>(t: A) -> FnText1<A> {
    FnText1("ASC", t)
}

#[inline]
pub fn char<A: Number>(n: A) -> FnText1<A> {
    FnText1("CHAR", n)
}

#[inline]
pub fn clean<A: Text>(t: A) -> FnText1<A> {
    FnText1("CLEAN", t)
}

#[inline]
pub fn code<A: Text>(t: A) -> FnNumber1<A> {
    FnNumber1("CODE", t)
}

#[inline]
pub fn concatenate<A: Sequence>(t: A) -> FnText1<A> {
    FnText1("CONCATENATE", t)
}

#[inline]
pub fn dollar<A: Number>(n: A) -> FnText1<A> {
    FnText1("DOLLAR", n)
}

#[inline]
pub fn dollar_<A: Number, B: Number>(n: A, d: B) -> FnText2<A, B> {
    FnText2("DOLLAR", n, d)
}

#[inline]
pub fn exact<A: Text, B: Text>(t1: A, t2: B) -> FnLogical2<A, B> {
    FnLogical2("EXACT", t1, t2)
}

#[inline]
pub fn find<A: Text, B: Text>(search: A, t: B) -> FnNumber2<A, B> {
    FnNumber2("FIND", search, t)
}

#[inline]
pub fn find_<A: Text, B: Text, C: Number>(search: A, t: B, start: C) -> FnNumber3<A, B, C> {
    FnNumber3("FIND", search, t, start)
}

#[inline]
pub fn fixed<A: Number>(n: A) -> FnText1<A> {
    FnText1("FIXED", n)
}

#[inline]
pub fn fixed_<A: Number, B: Number>(n: A, d: B) -> FnText2<A, B> {
    FnText2("FIXED", n, d)
}

#[inline]
pub fn fixed__<A: Number, B: Number, C: Logical>(n: A, d: B, omit_separators: C) -> FnText3<A, B, C> {
    FnText3("FIXED", n, d, omit_separators)
}

#[inline]
pub fn jis<A: Text>(t: A) -> FnText1<A> {
    FnText1("JIS", t)
}

#[inline]
pub fn left<A: Text>(t: A) -> FnText1<A> {
    FnText1("LEFT", t)
}

#[inline]
pub fn left_<A: Text, B: Number>(t: A, length: B) -> FnText2<A, B> {
    FnText2("LEFT", t, length)
}

#[inline]
pub fn len<A: Text>(t: A) -> FnNumber1<A> {
    FnNumber1("LEN", t)
}

#[inline]
pub fn lower<A: Text>(t: A) -> FnText1<A> {
    FnText1("LOWER", t)
}

#[inline]
pub fn mid<A: Text, B: Number, C: Number>(t: A, start: B, length: C) -> FnText3<A, B, C> {
    FnText3("MID", t, start, length)
}

#[inline]
pub fn proper<A: Text>(t: A) -> FnText1<A> {
    FnText1("PROPER", t)
}

#[inline]
pub fn replace<A: Text, B: Number, C: Number, D: Text>(t: A, start: B, count: C, new: D) -> FnText4<A, B, C, D> {
    FnText4("REPLACE", t, start, count, new)
}

#[inline]
pub fn rept<A: Text, B: Number>(t: A, count: B) -> FnText2<A, B> {
    FnText2("REPT", t, count)
}

#[inline]
pub fn right<A: Text>(t: A) -> FnText1<A> {
    FnText1("RIGHT", t)
}

#[inline]
pub fn right_<A: Text, B: Number>(t: A, length: B) -> FnText2<A, B> {
    FnText2("RIGHT", t, length)
}

#[inline]
pub fn search<A: Text, B: Text>(search: A, t: B) -> FnNumber2<A, B> {
    FnNumber2("SEARCH", search, t)
}

#[inline]
pub fn search_<A: Text, B: Text, C: Number>(search: A, t: B, start: C) -> FnNumber3<A, B, C> {
    FnNumber3("SEARCH", search, t, start)
}

#[inline]
pub fn substitute<A: Text, B: Text, C: Text>(t: A, old: B, new: C) -> FnText3<A, B, C> {
    FnText3("SUBSTITUTE", t, old, new)
}

#[inline]
pub fn substitute_<A: Text, B: Text, C: Text, D: Number>(t: A, old: B, new: C, which: D) -> FnText4<A, B, C, D> {
    FnText4("SUBSTITUTE", t, old, new, which)
}

#[inline]
pub fn t<A: Any>(x: A) -> FnText1<A> {
    FnText1("T", x)
}

#[inline]
pub fn text<A: Scalar, B: Text>(x: A, format_code: B) -> FnText2<A, B> {
    FnText2("TEXT", x, format_code)
}

#[inline]
pub fn trim<A: Text>(t: A) -> FnText1<A> {
    FnText1("TRIM", t)
}

#[inline]
pub fn unichar<A: Number>(n: A) -> FnText1<A> {
    FnText1("UNICHAR", n)
}

#[inline]
pub fn unicode<A: Text>(t: A) -> FnNumber1<A> {
    FnNumber1("UNICODE", t)
}

#[inline]
pub fn upper<A: Text>(t: A) -> FnText1<A> {
    FnText1("UPPER", t)
}
