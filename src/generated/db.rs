use crate::*;
#[allow(unused_imports)]
use crate::db::*;

#[inline]
pub fn daverage<A: Database, B: Field, C: Criteria>(d: A, f: B, c: C) -> FnNumber3<A, B, C> {
    FnNumber3("DAVERAGE", d, f, c)
}

#[inline]
pub fn dcount<A: Database, B: Field, C: Criteria>(d: A, f: Option<B>, c: C) -> FnNumber3<A, Option<B>, C> {
    FnNumber3("DCOUNT", d, f, c)
}

#[inline]
pub fn dcounta<A: Database, B: Field, C: Criteria>(d: A, f: Option<B>, c: C) -> FnNumber3<A, Option<B>, C> {
    FnNumber3("DCOUNTA", d, f, c)
}

#[inline]
pub fn dget<A: Database, B: Field, C: Criteria>(d: A, f: B, c: C) -> FnNumber3<A, B, C> {
    FnNumber3("DGET", d, f, c)
}

#[inline]
pub fn dmax<A: Database, B: Field, C: Criteria>(d: A, f: B, c: C) -> FnNumber3<A, B, C> {
    FnNumber3("DMAX", d, f, c)
}

#[inline]
pub fn dmin<A: Database, B: Field, C: Criteria>(d: A, f: B, c: C) -> FnNumber3<A, B, C> {
    FnNumber3("DMIN", d, f, c)
}

#[inline]
pub fn dproduct<A: Database, B: Field, C: Criteria>(d: A, f: B, c: C) -> FnNumber3<A, B, C> {
    FnNumber3("DPRODUCT", d, f, c)
}

#[inline]
pub fn dstdev<A: Database, B: Field, C: Criteria>(d: A, f: B, c: C) -> FnNumber3<A, B, C> {
    FnNumber3("DSTDEV", d, f, c)
}

#[inline]
pub fn dstdevp<A: Database, B: Field, C: Criteria>(d: A, f: B, c: C) -> FnNumber3<A, B, C> {
    FnNumber3("DSTDEVP", d, f, c)
}

#[inline]
pub fn dsum<A: Database, B: Field, C: Criteria>(d: A, f: B, c: C) -> FnNumber3<A, B, C> {
    FnNumber3("DSUM", d, f, c)
}

#[inline]
pub fn dvar<A: Database, B: Field, C: Criteria>(d: A, f: B, c: C) -> FnNumber3<A, B, C> {
    FnNumber3("DVAR", d, f, c)
}

#[inline]
pub fn dvarp<A: Database, B: Field, C: Criteria>(d: A, f: B, c: C) -> FnNumber3<A, B, C> {
    FnNumber3("DVARP", d, f, c)
}
