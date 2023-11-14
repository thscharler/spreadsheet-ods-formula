use crate::*;
#[allow(unused_imports)]
use crate::db::*;

///  Finds the average of values in a given field from the records (rows) in a database that match a search criteria.
#[inline]
pub fn daverage<A: Database, B: Field, C: Criteria>(d: A, f: B, c: C) -> FnNumber3<A, B, C> {
    FnNumber3("DAVERAGE", d, f, c)
}

///  Counts the number of records (rows) in a database that match a search criteria and contain numerical values.
#[inline]
pub fn dcount<A: Database, B: Field, C: Criteria>(d: A, f: Option<B>, c: C) -> FnNumber3<A, Option<B>, C> {
    FnNumber3("DCOUNT", d, f, c)
}

///  Counts the number of records (rows) in a database that match a search criteria and contain values (as COUNTA).
#[inline]
pub fn dcounta<A: Database, B: Field, C: Criteria>(d: A, f: Option<B>, c: C) -> FnNumber3<A, Option<B>, C> {
    FnNumber3("DCOUNTA", d, f, c)
}

///  Gets the single value in the field from the single record (row) in a database that matches a search criteria.
#[inline]
pub fn dget<A: Database, B: Field, C: Criteria>(d: A, f: B, c: C) -> FnNumber3<A, B, C> {
    FnNumber3("DGET", d, f, c)
}

///  Finds the maximum value in a given field from the records (rows) in a database that match a search criteria.
#[inline]
pub fn dmax<A: Database, B: Field, C: Criteria>(d: A, f: B, c: C) -> FnNumber3<A, B, C> {
    FnNumber3("DMAX", d, f, c)
}

///  Finds the minimum value in a given field from the records (rows) in a database that match a search criteria.
#[inline]
pub fn dmin<A: Database, B: Field, C: Criteria>(d: A, f: B, c: C) -> FnNumber3<A, B, C> {
    FnNumber3("DMIN", d, f, c)
}

///  Finds the product of values in a given field from the records (rows) in a database that match a search criteria.
#[inline]
pub fn dproduct<A: Database, B: Field, C: Criteria>(d: A, f: B, c: C) -> FnNumber3<A, B, C> {
    FnNumber3("DPRODUCT", d, f, c)
}

///  Finds the sample standard deviation in a given field from the records (rows) in a database that match a search criteria.
#[inline]
pub fn dstdev<A: Database, B: Field, C: Criteria>(d: A, f: B, c: C) -> FnNumber3<A, B, C> {
    FnNumber3("DSTDEV", d, f, c)
}

///  Finds the population standard deviation in a given field from the records (rows) in a database that match a search criteria.
#[inline]
pub fn dstdevp<A: Database, B: Field, C: Criteria>(d: A, f: B, c: C) -> FnNumber3<A, B, C> {
    FnNumber3("DSTDEVP", d, f, c)
}

///  Finds the sum of values in a given field from the records (rows) in a database that match a search criteria.
#[inline]
pub fn dsum<A: Database, B: Field, C: Criteria>(d: A, f: B, c: C) -> FnNumber3<A, B, C> {
    FnNumber3("DSUM", d, f, c)
}

///  Finds the sample variance in a given field from the records (rows) in a database that match a search criteria.
#[inline]
pub fn dvar<A: Database, B: Field, C: Criteria>(d: A, f: B, c: C) -> FnNumber3<A, B, C> {
    FnNumber3("DVAR", d, f, c)
}

///  Finds the population variance in a given field from the records (rows) in a database that match a search criteria.
#[inline]
pub fn dvarp<A: Database, B: Field, C: Criteria>(d: A, f: B, c: C) -> FnNumber3<A, B, C> {
    FnNumber3("DVARP", d, f, c)
}
