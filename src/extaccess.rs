use crate::{Any, FnNumber3, FnNumber4, FnText2, Param, Text};

pub enum DDEConversion {
    NumberLocalized,
    NumberEnUS,
}

impl Param for DDEConversion {
    type Type = &'static str;

    fn as_param(&self) -> Self::Type {
        match self {
            DDEConversion::NumberLocalized => "0",
            DDEConversion::NumberEnUS => "1",
        }
    }
}

///  Returns data from a DDE request.
#[inline]
pub fn dde<A: Text, B: Text, C: Text>(server: A, topic: B, item: C) -> FnNumber3<A, B, C> {
    FnNumber3("DDE", server, topic, item)
}

///  Returns data from a DDE request.
#[inline]
pub fn dde_conv<A: Text, B: Text, C: Text>(
    server: A,
    topic: B,
    item: C,
    mode: DDEConversion,
) -> FnNumber4<A, B, C, <DDEConversion as Param>::Type> {
    FnNumber4("DDE", server, topic, item, mode.as_param())
}

///  Returns data from a DDE request.
#[inline]
pub fn dde_text<A: Text, B: Text, C: Text>(
    server: A,
    topic: B,
    item: C,
) -> FnNumber4<A, B, C, &'static str> {
    FnNumber4("DDE", server, topic, item, "2")
}

/// Creation of a hyperlink involving an evaluated expression.
#[inline]
pub fn hyperlink<A: Text, B: Any>(iri: A, fun: B) -> FnText2<A, B> {
    FnText2("HYPERLINK", iri, fun)
}
