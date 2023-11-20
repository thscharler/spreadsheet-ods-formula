//!
//! External access
//!

use crate::{Any, FnNumber4, Text};

pub use crate::generated::ext::*;

pub enum DDEConversion {
    NumberLocalized,
    NumberEnUS,
}

impl Any for DDEConversion {
    #[inline]
    fn formula(&self, buf: &mut String) {
        buf.push_str(match self {
            DDEConversion::NumberLocalized => "0",
            DDEConversion::NumberEnUS => "1",
        });
    }
}

///  Returns data from a DDE request.
#[inline]
pub fn dde_conv<A: Text, B: Text, C: Text>(
    server: A,
    topic: B,
    item: C,
    mode: DDEConversion,
) -> FnNumber4<A, B, C, DDEConversion> {
    FnNumber4("DDE", server, topic, item, mode)
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
