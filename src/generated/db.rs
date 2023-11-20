//! 
//! Database functions use the variables, Database 4.11.9, Field 4.11.10, and 
//! Criteria 4.11.11.
//! 
//! The results of database functions may change when the values of the 
//! HOST-USE-REGULAR-EXPRESSIONS or HOST-USE-WILDCARDS or 
//! HOST-SEARCH-CRITERIA-MUST-APPLY-TO-WHOLE-CELL properties change. 3.4

use crate::*;
#[allow(unused_imports)]
use crate::db::*;

/// Finds the average of values in a given field from the records (rows) in a 
/// database that match a search criteria.
/// Syntax: DAVERAGE( D Database;; F Field;; C Criteria; )
///
/// Constraints:
/// None
///
/// Semantics:
/// Perform AVERAGE on data records in database D field F that match criteria 
/// C.
///
/// See also: "AVERAGE", "COUNT", "DSUM", "DCOUNT", "SUM", 
#[inline]
pub fn daverage<A: Database, B: Field, C: Criteria>(d: A, f: B, c: C) -> FnNumber3<A, B, C> {
    FnNumber3("DAVERAGE", d, f, c)
}

/// Counts the number of records (rows) in a database that match a search 
/// criteria and contain numerical values.
/// Syntax: DCOUNT( D Database;[; F Field]; C Criteria; )
///
/// Constraints:
/// None
///
/// Semantics:
/// Perform COUNT on data records in database D field F that match criteria C. 
/// If the Field argument is omitted, DCOUNT returns the count of all records 
/// that satisfy Criteria C.
///
/// See also: "COUNT", "COUNTA", "DCOUNTA", "DSUM", 
#[inline]
pub fn dcount<A: Database, B: Field, C: Criteria>(d: A, f: Option<B>, c: C) -> FnNumber3<A, Option<B>, C> {
    FnNumber3("DCOUNT", d, f, c)
}

/// Counts the number of records (rows) in a database that match a search 
/// criteria and contain values (as COUNTA).
/// Syntax: DCOUNTA( D Database;[; F Field]; C Criteria; )
///
/// Constraints:
/// None
///
/// Semantics:
/// Perform COUNTA on data records in database D field F that match criteria C. 
/// If the Field argument is omitted, DCOUNTA returns the count of all records 
/// that satisfy criteria C.
///
/// See also: "COUNT", "COUNTA", "DCOUNT", "DSUM", 
#[inline]
pub fn dcounta<A: Database, B: Field, C: Criteria>(d: A, f: Option<B>, c: C) -> FnNumber3<A, Option<B>, C> {
    FnNumber3("DCOUNTA", d, f, c)
}

/// Gets the single value in the field from the single record (row) in a 
/// database that matches a search criteria.
/// Syntax: DGET( D Database;; F Field;; C Criteria; )
///
/// Constraints:
/// None
///
/// Semantics:
/// Extracts the value in field F of the one data record in database D that 
/// matches criteria C. If no records match, or more than one matches, it 
/// returns an Error.
///
/// See also: "DMAX", "DMIN", "DSUM", 
#[inline]
pub fn dget<A: Database, B: Field, C: Criteria>(d: A, f: B, c: C) -> FnNumber3<A, B, C> {
    FnNumber3("DGET", d, f, c)
}

/// Finds the maximum value in a given field from the records (rows) in a 
/// database that match a search criteria.
/// Syntax: DMAX( D Database;; F Field;; C Criteria; )
///
/// Constraints:
/// None
///
/// Semantics:
/// Perform MAX on only the data records in database D field F that match 
/// criteria C.
///
/// See also: "MAX", "DMIN", "MIN", 
#[inline]
pub fn dmax<A: Database, B: Field, C: Criteria>(d: A, f: B, c: C) -> FnNumber3<A, B, C> {
    FnNumber3("DMAX", d, f, c)
}

/// Finds the minimum value in a given field from the records (rows) in a 
/// database that match a search criteria.
/// Syntax: DMIN( D Database;; F Field;; C Criteria; )
///
/// Constraints:
/// None
///
/// Semantics:
/// Perform MIN on only the data records in database D field F that match 
/// criteria C.
///
/// See also: "MIN", "DMAX", "MAX", 
#[inline]
pub fn dmin<A: Database, B: Field, C: Criteria>(d: A, f: B, c: C) -> FnNumber3<A, B, C> {
    FnNumber3("DMIN", d, f, c)
}

/// Finds the product of values in a given field from the records (rows) in a 
/// database that match a search criteria.
/// Syntax: DPRODUCT( D Database;; F Field;; C Criteria; )
///
/// Constraints:
/// None
///
/// Semantics:
/// Multiply together only the data records in database D field F that match 
/// criteria C.
///
/// See also: "SUM", "DSUM", 
#[inline]
pub fn dproduct<A: Database, B: Field, C: Criteria>(d: A, f: B, c: C) -> FnNumber3<A, B, C> {
    FnNumber3("DPRODUCT", d, f, c)
}

/// Finds the sample standard deviation in a given field from the records 
/// (rows) in a database that match a search criteria.
/// Syntax: DSTDEV( D Database;; F Field;; C Criteria; )
///
/// Constraints:
/// None
///
/// Semantics:
/// Perform STDEV on only the data records in database D field F that match 
/// criteria C.
///
/// See also: "STDEV", "DSTDEVP", 
#[inline]
pub fn dstdev<A: Database, B: Field, C: Criteria>(d: A, f: B, c: C) -> FnNumber3<A, B, C> {
    FnNumber3("DSTDEV", d, f, c)
}

/// Finds the population standard deviation in a given field from the records 
/// (rows) in a database that match a search criteria.
/// Syntax: DSTDEVP( D Database;; F Field;; C Criteria; )
///
/// Constraints:
/// None
///
/// Semantics:
/// Perform STDEVP on only the data records in database D field F that match 
/// criteria C.
///
/// See also: "STDEVP", "DSTDEV", 
#[inline]
pub fn dstdevp<A: Database, B: Field, C: Criteria>(d: A, f: B, c: C) -> FnNumber3<A, B, C> {
    FnNumber3("DSTDEVP", d, f, c)
}

/// Finds the sum of values in a given field from the records (rows) in a 
/// database that match a search criteria.
/// Syntax: DSUM( D Database;; F Field;; C Criteria; )
///
/// Constraints:
/// None
///
/// Semantics:
/// Perform SUM on only the data records in database D field F that match 
/// criteria C.
///
/// See also: "SUM", "DMIN", "DMAX", 
#[inline]
pub fn dsum<A: Database, B: Field, C: Criteria>(d: A, f: B, c: C) -> FnNumber3<A, B, C> {
    FnNumber3("DSUM", d, f, c)
}

/// Finds the sample variance in a given field from the records (rows) in a 
/// database that match a search criteria.
/// Syntax: DVAR( D Database;; F Field;; C Criteria; )
///
/// Constraints:
/// None
///
/// Semantics:
/// Perform VAR on only the data records in database D field F that match 
/// criteria C.
///
/// See also: "VAR", "DVARP", 
#[inline]
pub fn dvar<A: Database, B: Field, C: Criteria>(d: A, f: B, c: C) -> FnNumber3<A, B, C> {
    FnNumber3("DVAR", d, f, c)
}

/// Finds the population variance in a given field from the records (rows) in a 
/// database that match a search criteria.
/// Syntax: DVARP( D Database;; F Field;; C Criteria; )
///
/// Constraints:
/// None
///
/// Semantics:
/// Perform VARP on only the data records in database D field F that match 
/// criteria C.
///
/// See also: "VARP", "DVAR", 
#[inline]
pub fn dvarp<A: Database, B: Field, C: Criteria>(d: A, f: B, c: C) -> FnNumber3<A, B, C> {
    FnNumber3("DVARP", d, f, c)
}
