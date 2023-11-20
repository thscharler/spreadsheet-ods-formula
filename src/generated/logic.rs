use crate::*;
#[allow(unused_imports)]
use crate::logic::*;

#[inline]
pub fn and<A: Sequence>(l: A) -> FnLogical1<A> {
    FnLogical1("AND", l)
}

#[inline]
pub fn false_() -> FnLogical0 {
    FnLogical0("FALSE", )
}

#[inline]
pub fn iferror<A: Any, B: Any>(x: A, alternative: B) -> FnAny2<A, B> {
    FnAny2("IFERROR", x, alternative)
}

#[inline]
pub fn ifna<A: Any, B: Any>(x: A, alternative: B) -> FnAny2<A, B> {
    FnAny2("IFNA", x, alternative)
}

#[inline]
pub fn not<A: Logical>(l: A) -> FnLogical1<A> {
    FnLogical1("NOT", l)
}

#[inline]
pub fn or<A: Sequence>(l: A) -> FnLogical1<A> {
    FnLogical1("OR", l)
}

#[inline]
pub fn true_() -> FnLogical0 {
    FnLogical0("TRUE", )
}

#[inline]
pub fn xor<A: Sequence>(l: A) -> FnLogical1<A> {
    FnLogical1("XOR", l)
}
