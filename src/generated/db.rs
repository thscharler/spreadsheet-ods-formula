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
///
/// __Syntax__: 
/// ```ods
///     DAVERAGE( D: Database; F: Field; C: Criteria )
/// ```
///
/// __Constraints__:
/// None
///
/// __Semantics__:
/// Perform AVERAGE on data records in database D field F that match criteria 
/// C.
///
/// __See also__: "AVERAGE", "COUNT", "DSUM", "DCOUNT", "SUM", 
#[inline]
pub fn daverage<A: Database, B: Field, C: Criteria>(d: A, f: B, c: C) -> FnNumber3<A, B, C> {
    FnNumber3("DAVERAGE", d, f, c)
}

/// Counts the number of records (rows) in a database that match a search 
/// criteria and contain numerical values.
///
/// __Syntax__: 
/// ```ods
///     DCOUNT( D: Database[; F: Field]; C: Criteria )
/// ```
///
/// __Constraints__:
/// None
///
/// __Semantics__:
/// Perform COUNT on data records in database D field F that match criteria C. 
/// If the Field argument is omitted, DCOUNT returns the count of all records 
/// that satisfy Criteria C.
///
/// __See also__: "COUNT", "COUNTA", "DCOUNTA", "DSUM", 
#[inline]
pub fn dcount<A: Database, B: Field, C: Criteria>(d: A, f: Option<B>, c: C) -> FnNumber3<A, Option<B>, C> {
    FnNumber3("DCOUNT", d, f, c)
}

/// Counts the number of records (rows) in a database that match a search 
/// criteria and contain values (as COUNTA).
///
/// __Syntax__: 
/// ```ods
///     DCOUNTA( D: Database[; F: Field]; C: Criteria )
/// ```
///
/// __Constraints__:
/// None
///
/// __Semantics__:
/// Perform COUNTA on data records in database D field F that match criteria C. 
/// If the Field argument is omitted, DCOUNTA returns the count of all records 
/// that satisfy criteria C.
///
/// __See also__: "COUNT", "COUNTA", "DCOUNT", "DSUM", 
#[inline]
pub fn dcounta<A: Database, B: Field, C: Criteria>(d: A, f: Option<B>, c: C) -> FnNumber3<A, Option<B>, C> {
    FnNumber3("DCOUNTA", d, f, c)
}

/// Gets the single value in the field from the single record (row) in a 
/// database that matches a search criteria.
///
/// __Syntax__: 
/// ```ods
///     DGET( D: Database; F: Field; C: Criteria )
/// ```
///
/// __Constraints__:
/// None
///
/// __Semantics__:
/// Extracts the value in field F of the one data record in database D that 
/// matches criteria C. If no records match, or more than one matches, it 
/// returns an Error.
///
/// __See also__: "DMAX", "DMIN", "DSUM", 
#[inline]
pub fn dget<A: Database, B: Field, C: Criteria>(d: A, f: B, c: C) -> FnNumber3<A, B, C> {
    FnNumber3("DGET", d, f, c)
}

/// Finds the maximum value in a given field from the records (rows) in a 
/// database that match a search criteria.
///
/// __Syntax__: 
/// ```ods
///     DMAX( D: Database; F: Field; C: Criteria )
/// ```
///
/// __Constraints__:
/// None
///
/// __Semantics__:
/// Perform MAX on only the data records in database D field F that match 
/// criteria C.
///
/// __See also__: "MAX", "DMIN", "MIN", 
#[inline]
pub fn dmax<A: Database, B: Field, C: Criteria>(d: A, f: B, c: C) -> FnNumber3<A, B, C> {
    FnNumber3("DMAX", d, f, c)
}

/// Finds the minimum value in a given field from the records (rows) in a 
/// database that match a search criteria.
///
/// __Syntax__: 
/// ```ods
///     DMIN( D: Database; F: Field; C: Criteria )
/// ```
///
/// __Constraints__:
/// None
///
/// __Semantics__:
/// Perform MIN on only the data records in database D field F that match 
/// criteria C.
///
/// __See also__: "MIN", "DMAX", "MAX", 
#[inline]
pub fn dmin<A: Database, B: Field, C: Criteria>(d: A, f: B, c: C) -> FnNumber3<A, B, C> {
    FnNumber3("DMIN", d, f, c)
}

/// Finds the product of values in a given field from the records (rows) in a 
/// database that match a search criteria.
///
/// __Syntax__: 
/// ```ods
///     DPRODUCT( D: Database; F: Field; C: Criteria )
/// ```
///
/// __Constraints__:
/// None
///
/// __Semantics__:
/// Multiply together only the data records in database D field F that match 
/// criteria C.
///
/// __See also__: "SUM", "DSUM", 
#[inline]
pub fn dproduct<A: Database, B: Field, C: Criteria>(d: A, f: B, c: C) -> FnNumber3<A, B, C> {
    FnNumber3("DPRODUCT", d, f, c)
}

/// Finds the sample standard deviation in a given field from the records 
/// (rows) in a database that match a search criteria.
///
/// __Syntax__: 
/// ```ods
///     DSTDEV( D: Database; F: Field; C: Criteria )
/// ```
///
/// __Constraints__:
/// None
///
/// __Semantics__:
/// Perform STDEV on only the data records in database D field F that match 
/// criteria C.
///
/// __See also__: "STDEV", "DSTDEVP", 
#[inline]
pub fn dstdev<A: Database, B: Field, C: Criteria>(d: A, f: B, c: C) -> FnNumber3<A, B, C> {
    FnNumber3("DSTDEV", d, f, c)
}

/// Finds the population standard deviation in a given field from the records 
/// (rows) in a database that match a search criteria.
///
/// __Syntax__: 
/// ```ods
///     DSTDEVP( D: Database; F: Field; C: Criteria )
/// ```
///
/// __Constraints__:
/// None
///
/// __Semantics__:
/// Perform STDEVP on only the data records in database D field F that match 
/// criteria C.
///
/// __See also__: "STDEVP", "DSTDEV", 
#[inline]
pub fn dstdevp<A: Database, B: Field, C: Criteria>(d: A, f: B, c: C) -> FnNumber3<A, B, C> {
    FnNumber3("DSTDEVP", d, f, c)
}

/// Finds the sum of values in a given field from the records (rows) in a 
/// database that match a search criteria.
///
/// __Syntax__: 
/// ```ods
///     DSUM( D: Database; F: Field; C: Criteria )
/// ```
///
/// __Constraints__:
/// None
///
/// __Semantics__:
/// Perform SUM on only the data records in database D field F that match 
/// criteria C.
///
/// __See also__: "SUM", "DMIN", "DMAX", 
#[inline]
pub fn dsum<A: Database, B: Field, C: Criteria>(d: A, f: B, c: C) -> FnNumber3<A, B, C> {
    FnNumber3("DSUM", d, f, c)
}

/// Finds the sample variance in a given field from the records (rows) in a 
/// database that match a search criteria.
///
/// __Syntax__: 
/// ```ods
///     DVAR( D: Database; F: Field; C: Criteria )
/// ```
///
/// __Constraints__:
/// None
///
/// __Semantics__:
/// Perform VAR on only the data records in database D field F that match 
/// criteria C.
///
/// __See also__: "VAR", "DVARP", 
#[inline]
pub fn dvar<A: Database, B: Field, C: Criteria>(d: A, f: B, c: C) -> FnNumber3<A, B, C> {
    FnNumber3("DVAR", d, f, c)
}

/// Finds the population variance in a given field from the records (rows) in a 
/// database that match a search criteria.
///
/// __Syntax__: 
/// ```ods
///     DVARP( D: Database; F: Field; C: Criteria )
/// ```
///
/// __Constraints__:
/// None
///
/// __Semantics__:
/// Perform VARP on only the data records in database D field F that match 
/// criteria C.
///
/// __See also__: "VARP", "DVAR", 
#[inline]
pub fn dvarp<A: Database, B: Field, C: Criteria>(d: A, f: B, c: C) -> FnNumber3<A, B, C> {
    FnNumber3("DVARP", d, f, c)
}
