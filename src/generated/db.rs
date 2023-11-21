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
/// [documentfoundation->DAVERAGE](https://wiki.documentfoundation.org/Documentation/Calc_Functions/DAVERAGE)
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
/// __See also__: [crate::of::average()], [crate::of::count()], [crate::of::dsum()], [crate::of::dcount()], [crate::of::sum()], 
#[inline]
pub fn daverage<A: Database, B: Field, C: Criteria>(d: A, f: B, c: C) -> FnNumber3<A, B, C> {
    FnNumber3("DAVERAGE", d, f, c)
}

/// Counts the number of records (rows) in a database that match a search 
/// criteria and contain numerical values.
///
/// [documentfoundation->DCOUNT](https://wiki.documentfoundation.org/Documentation/Calc_Functions/DCOUNT)
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
/// __See also__: [crate::of::count()], [crate::of::counta()], [crate::of::dcounta()], [crate::of::dsum()], 
#[inline]
pub fn dcount<A: Database, B: Field, C: Criteria>(d: A, f: Option<B>, c: C) -> FnNumber3<A, Option<B>, C> {
    FnNumber3("DCOUNT", d, f, c)
}

/// Counts the number of records (rows) in a database that match a search 
/// criteria and contain values (as COUNTA).
///
/// [documentfoundation->DCOUNTA](https://wiki.documentfoundation.org/Documentation/Calc_Functions/DCOUNTA)
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
/// __See also__: [crate::of::count()], [crate::of::counta()], [crate::of::dcount()], [crate::of::dsum()], 
#[inline]
pub fn dcounta<A: Database, B: Field, C: Criteria>(d: A, f: Option<B>, c: C) -> FnNumber3<A, Option<B>, C> {
    FnNumber3("DCOUNTA", d, f, c)
}

/// Gets the single value in the field from the single record (row) in a 
/// database that matches a search criteria.
///
/// [documentfoundation->DGET](https://wiki.documentfoundation.org/Documentation/Calc_Functions/DGET)
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
/// __See also__: [crate::of::dmax()], [crate::of::dmin()], [crate::of::dsum()], 
#[inline]
pub fn dget<A: Database, B: Field, C: Criteria>(d: A, f: B, c: C) -> FnNumber3<A, B, C> {
    FnNumber3("DGET", d, f, c)
}

/// Finds the maximum value in a given field from the records (rows) in a 
/// database that match a search criteria.
///
/// [documentfoundation->DMAX](https://wiki.documentfoundation.org/Documentation/Calc_Functions/DMAX)
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
/// __See also__: [crate::of::max()], [crate::of::dmin()], [crate::of::min()], 
#[inline]
pub fn dmax<A: Database, B: Field, C: Criteria>(d: A, f: B, c: C) -> FnNumber3<A, B, C> {
    FnNumber3("DMAX", d, f, c)
}

/// Finds the minimum value in a given field from the records (rows) in a 
/// database that match a search criteria.
///
/// [documentfoundation->DMIN](https://wiki.documentfoundation.org/Documentation/Calc_Functions/DMIN)
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
/// __See also__: [crate::of::min()], [crate::of::dmax()], [crate::of::max()], 
#[inline]
pub fn dmin<A: Database, B: Field, C: Criteria>(d: A, f: B, c: C) -> FnNumber3<A, B, C> {
    FnNumber3("DMIN", d, f, c)
}

/// Finds the product of values in a given field from the records (rows) in a 
/// database that match a search criteria.
///
/// [documentfoundation->DPRODUCT](https://wiki.documentfoundation.org/Documentation/Calc_Functions/DPRODUCT)
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
/// __See also__: [crate::of::sum()], [crate::of::dsum()], 
#[inline]
pub fn dproduct<A: Database, B: Field, C: Criteria>(d: A, f: B, c: C) -> FnNumber3<A, B, C> {
    FnNumber3("DPRODUCT", d, f, c)
}

/// Finds the sample standard deviation in a given field from the records 
/// (rows) in a database that match a search criteria.
///
/// [documentfoundation->DSTDEV](https://wiki.documentfoundation.org/Documentation/Calc_Functions/DSTDEV)
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
/// __See also__: [crate::of::stdev()], [crate::of::dstdevp()], 
#[inline]
pub fn dstdev<A: Database, B: Field, C: Criteria>(d: A, f: B, c: C) -> FnNumber3<A, B, C> {
    FnNumber3("DSTDEV", d, f, c)
}

/// Finds the population standard deviation in a given field from the records 
/// (rows) in a database that match a search criteria.
///
/// [documentfoundation->DSTDEVP](https://wiki.documentfoundation.org/Documentation/Calc_Functions/DSTDEVP)
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
/// __See also__: [crate::of::stdevp()], [crate::of::dstdev()], 
#[inline]
pub fn dstdevp<A: Database, B: Field, C: Criteria>(d: A, f: B, c: C) -> FnNumber3<A, B, C> {
    FnNumber3("DSTDEVP", d, f, c)
}

/// Finds the sum of values in a given field from the records (rows) in a 
/// database that match a search criteria.
///
/// [documentfoundation->DSUM](https://wiki.documentfoundation.org/Documentation/Calc_Functions/DSUM)
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
/// __See also__: [crate::of::sum()], [crate::of::dmin()], [crate::of::dmax()], 
#[inline]
pub fn dsum<A: Database, B: Field, C: Criteria>(d: A, f: B, c: C) -> FnNumber3<A, B, C> {
    FnNumber3("DSUM", d, f, c)
}

/// Finds the sample variance in a given field from the records (rows) in a 
/// database that match a search criteria.
///
/// [documentfoundation->DVAR](https://wiki.documentfoundation.org/Documentation/Calc_Functions/DVAR)
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
/// __See also__: [crate::of::var()], [crate::of::dvarp()], 
#[inline]
pub fn dvar<A: Database, B: Field, C: Criteria>(d: A, f: B, c: C) -> FnNumber3<A, B, C> {
    FnNumber3("DVAR", d, f, c)
}

/// Finds the population variance in a given field from the records (rows) in a 
/// database that match a search criteria.
///
/// [documentfoundation->DVARP](https://wiki.documentfoundation.org/Documentation/Calc_Functions/DVARP)
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
/// __See also__: [crate::of::varp()], [crate::of::dvar()], 
#[inline]
pub fn dvarp<A: Database, B: Field, C: Criteria>(d: A, f: B, c: C) -> FnNumber3<A, B, C> {
    FnNumber3("DVARP", d, f, c)
}
