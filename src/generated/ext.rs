use crate::*;
#[allow(unused_imports)]
use crate::ext::*;

#[inline]
pub fn dde<A: Text, B: Text, C: Text>(server: A, topic: B, item: C) -> FnText3<A, B, C> {
    FnText3("DDE", server, topic, item)
}

#[inline]
pub fn dde_<A: Text, B: Text, C: Text, D: Number>(server: A, topic: B, item: C, mode: D) -> FnText4<A, B, C, D> {
    FnText4("DDE", server, topic, item, mode)
}

#[inline]
pub fn hyperlink<A: Text>(i_r_i: A) -> FnText1<A> {
    FnText1("HYPERLINK", i_r_i)
}

#[inline]
pub fn hyperlink_<A: Text, B: TextOrNumber>(i_r_i: A, function_result: B) -> FnText2<A, B> {
    FnText2("HYPERLINK", i_r_i, function_result)
}
