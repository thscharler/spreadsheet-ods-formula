use crate::*;
#[allow(unused_imports)]
use crate::textb::*;

#[inline]
pub fn findb<A: Text, B: Text>(search: A, t: B) -> FnNumber2<A, B> {
    FnNumber2("FINDB", search, t)
}

#[inline]
pub fn findb_<A: Text, B: Text, C: Number>(search: A, t: B, start: C) -> FnNumber3<A, B, C> {
    FnNumber3("FINDB", search, t, start)
}

#[inline]
pub fn leftb<A: Text>(t: A) -> FnText1<A> {
    FnText1("LEFTB", t)
}

#[inline]
pub fn leftb_<A: Text, B: Number>(t: A, length: B) -> FnText2<A, B> {
    FnText2("LEFTB", t, length)
}

#[inline]
pub fn lenb<A: Text>(t: A) -> FnNumber1<A> {
    FnNumber1("LENB", t)
}

#[inline]
pub fn midb<A: Text, B: Number, C: Number>(t: A, start: B, length: C) -> FnText3<A, B, C> {
    FnText3("MIDB", t, start, length)
}

#[inline]
pub fn replaceb<A: Text, B: Number, C: Number, D: Text>(t: A, start: B, len: C, new: D) -> FnText4<A, B, C, D> {
    FnText4("REPLACEB", t, start, len, new)
}

#[inline]
pub fn rightb<A: Text>(t: A) -> FnText1<A> {
    FnText1("RIGHTB", t)
}

#[inline]
pub fn rightb_<A: Text, B: Number>(t: A, length: B) -> FnText2<A, B> {
    FnText2("RIGHTB", t, length)
}

#[inline]
pub fn searchb<A: Text, B: Text>(search: A, t: B) -> FnNumber2<A, B> {
    FnNumber2("SEARCHB", search, t)
}

#[inline]
pub fn searchb_<A: Text, B: Text, C: Number>(search: A, t: B, start: C) -> FnNumber3<A, B, C> {
    FnNumber3("SEARCHB", search, t, start)
}
