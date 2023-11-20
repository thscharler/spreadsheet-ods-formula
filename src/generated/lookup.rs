//! 
//! These functions look up information. Note that IF() 6.15.4 can be 
//! considered a trivial lookup function, but it is listed as a logical 
//! function instead.

use crate::*;
#[allow(unused_imports)]
use crate::lookup::*;

/// Returns a cell address (reference) as text.
/// Syntax: ADDRESS( Row Integer;; Column Integer; )
///
/// Constraints:
/// Row ≥ 1, Column ≥ 1, 1 ≤ Abs ≤ 4; A1Style = TRUE. Evaluators may 
/// evaluate expressions that do not meet the constraint A1Style = TRUE.
///
/// Semantics:
/// Returns a cell address (reference) as text. The text does not include the 
/// surrounding [...] of a reference. If a Sheet name is given, the sheet name 
/// in the text returned is followed by a “.” and the column/row reference 
/// if A1Style is TRUE, or a “!” and the column/row reference if A1Style is 
/// FALSE; otherwise no “.” respectively “!” is included. Columns are 
/// identified using uppercase letters. The value of Abs determines if the 
/// column and/or row is absolute or relative. The value of A1Style determines 
/// if A1 reference style or R1C1 reference style is used.
/// 
/// Table 21 - ADDRESS
/// 
/// Abs
/// 
/// Meaning
/// 
/// A1Style = TRUE
/// 
/// A1Style = FALSE
/// 
/// 1
/// 
/// fully absolute
/// 
/// $A$1
/// 
/// R1C1
/// 
/// 2
/// 
/// row absolute, column relative
/// 
/// A$1
/// 
/// R1C[1]
/// 
/// 3
/// 
/// row relative, column absolute
/// 
/// $A1
/// 
/// R[1]C1
/// 
/// 4
/// 
/// fully relative
/// 
/// A1
/// 
/// R[1]C[1]
/// 
/// Note that the INDIRECT function accepts this format.
///
/// See also: "INDIRECT", 
#[inline]
pub fn address<A: Number, B: Number>(row: A, column: B) -> FnText2<A, B> {
    FnText2("ADDRESS", row, column)
}

/// Returns a cell address (reference) as text.
/// Syntax: ADDRESS( Row Integer;; Column Integer;[; Abs Integer] )
///
/// Constraints:
/// Row ≥ 1, Column ≥ 1, 1 ≤ Abs ≤ 4; A1Style = TRUE. Evaluators may 
/// evaluate expressions that do not meet the constraint A1Style = TRUE.
///
/// Semantics:
/// Returns a cell address (reference) as text. The text does not include the 
/// surrounding [...] of a reference. If a Sheet name is given, the sheet name 
/// in the text returned is followed by a “.” and the column/row reference 
/// if A1Style is TRUE, or a “!” and the column/row reference if A1Style is 
/// FALSE; otherwise no “.” respectively “!” is included. Columns are 
/// identified using uppercase letters. The value of Abs determines if the 
/// column and/or row is absolute or relative. The value of A1Style determines 
/// if A1 reference style or R1C1 reference style is used.
/// 
/// Table 21 - ADDRESS
/// 
/// Abs
/// 
/// Meaning
/// 
/// A1Style = TRUE
/// 
/// A1Style = FALSE
/// 
/// 1
/// 
/// fully absolute
/// 
/// $A$1
/// 
/// R1C1
/// 
/// 2
/// 
/// row absolute, column relative
/// 
/// A$1
/// 
/// R1C[1]
/// 
/// 3
/// 
/// row relative, column absolute
/// 
/// $A1
/// 
/// R[1]C1
/// 
/// 4
/// 
/// fully relative
/// 
/// A1
/// 
/// R[1]C[1]
/// 
/// Note that the INDIRECT function accepts this format.
///
/// See also: "INDIRECT", 
#[inline]
pub fn address_<A: Number, B: Number>(row: A, column: B, abs: AddressAbs) -> FnText3<A, B, AddressAbs> {
    FnText3("ADDRESS", row, column, abs)
}

/// Returns a cell address (reference) as text.
/// Syntax: ADDRESS( Row Integer;; Column Integer;[; Abs Integer][; A1Style Logical] )
///
/// Constraints:
/// Row ≥ 1, Column ≥ 1, 1 ≤ Abs ≤ 4; A1Style = TRUE. Evaluators may 
/// evaluate expressions that do not meet the constraint A1Style = TRUE.
///
/// Semantics:
/// Returns a cell address (reference) as text. The text does not include the 
/// surrounding [...] of a reference. If a Sheet name is given, the sheet name 
/// in the text returned is followed by a “.” and the column/row reference 
/// if A1Style is TRUE, or a “!” and the column/row reference if A1Style is 
/// FALSE; otherwise no “.” respectively “!” is included. Columns are 
/// identified using uppercase letters. The value of Abs determines if the 
/// column and/or row is absolute or relative. The value of A1Style determines 
/// if A1 reference style or R1C1 reference style is used.
/// 
/// Table 21 - ADDRESS
/// 
/// Abs
/// 
/// Meaning
/// 
/// A1Style = TRUE
/// 
/// A1Style = FALSE
/// 
/// 1
/// 
/// fully absolute
/// 
/// $A$1
/// 
/// R1C1
/// 
/// 2
/// 
/// row absolute, column relative
/// 
/// A$1
/// 
/// R1C[1]
/// 
/// 3
/// 
/// row relative, column absolute
/// 
/// $A1
/// 
/// R[1]C1
/// 
/// 4
/// 
/// fully relative
/// 
/// A1
/// 
/// R[1]C[1]
/// 
/// Note that the INDIRECT function accepts this format.
///
/// See also: "INDIRECT", 
#[inline]
pub fn address__<A: Number, B: Number, C: Logical>(row: A, column: B, abs: AddressAbs, a1_style: C) -> FnText4<A, B, AddressAbs, C> {
    FnText4("ADDRESS", row, column, abs, a1_style)
}

/// Returns a cell address (reference) as text.
/// Syntax: ADDRESS( Row Integer;; Column Integer;[; Abs Integer][; A1Style Logical][; Sheet Text] )
///
/// Constraints:
/// Row ≥ 1, Column ≥ 1, 1 ≤ Abs ≤ 4; A1Style = TRUE. Evaluators may 
/// evaluate expressions that do not meet the constraint A1Style = TRUE.
///
/// Semantics:
/// Returns a cell address (reference) as text. The text does not include the 
/// surrounding [...] of a reference. If a Sheet name is given, the sheet name 
/// in the text returned is followed by a “.” and the column/row reference 
/// if A1Style is TRUE, or a “!” and the column/row reference if A1Style is 
/// FALSE; otherwise no “.” respectively “!” is included. Columns are 
/// identified using uppercase letters. The value of Abs determines if the 
/// column and/or row is absolute or relative. The value of A1Style determines 
/// if A1 reference style or R1C1 reference style is used.
/// 
/// Table 21 - ADDRESS
/// 
/// Abs
/// 
/// Meaning
/// 
/// A1Style = TRUE
/// 
/// A1Style = FALSE
/// 
/// 1
/// 
/// fully absolute
/// 
/// $A$1
/// 
/// R1C1
/// 
/// 2
/// 
/// row absolute, column relative
/// 
/// A$1
/// 
/// R1C[1]
/// 
/// 3
/// 
/// row relative, column absolute
/// 
/// $A1
/// 
/// R[1]C1
/// 
/// 4
/// 
/// fully relative
/// 
/// A1
/// 
/// R[1]C[1]
/// 
/// Note that the INDIRECT function accepts this format.
///
/// See also: "INDIRECT", 
#[inline]
pub fn address___<A: Number, B: Number, C: Logical, D: Text>(row: A, column: B, abs: AddressAbs, a1_style: C, sheet: D) -> FnText5<A, B, AddressAbs, C, D> {
    FnText5("ADDRESS", row, column, abs, a1_style, sheet)
}

/// Uses an index to return a value from a list of values.
/// Syntax: CHOOSE( Index Integer;{; Value Any}+ )
///
/// Constraints:
/// Returns an Error if Index < 1 or if there is no corresponding value in the 
/// list of Values.
///
/// Semantics:
/// Uses Index to determine which value, from a list of values, to return. If 
/// Index is 1, CHOOSE returns the first Value; if Index is 2, CHOOSE returns 
/// the second value, and so on. Note that the Values may be formula 
/// expressions. Expression paths of parameters other than the one chosen are 
/// not calculated or evaluated for side effects.
///
/// See also: "IF", 
#[inline]
pub fn choose<A: Number, B: Sequence>(index: A, value: B) -> FnAny2<A, B> {
    FnAny2("CHOOSE", index, value)
}

/// Look for a matching value in the first row of the given table, and return 
/// the value of the indicated row.
/// Syntax: HLOOKUP( Lookup Any;; DataSource Reference|Array;; Row Integer; )
///
/// Constraints:
/// Row ≥ 1; Searched portion of DataSource shall not include Logical values. 
/// Evaluators may evaluate expressions that do not meet the constraint that 
/// the searched portion of a DataSource not include Logical values.
///
/// Semantics:
/// 
/// If RangeLookup is omitted or TRUE or not 0, the first row of DataSource is 
/// assumed to be sorted in ascending order, with smaller numbers before larger 
/// ones, smaller text values before larger ones (e.g., "A" before "B", and "B" 
/// before "BA"), and FALSE before TRUE. If the types are mixed, Numbers are 
/// sorted before Text, and Text before Logicals; evaluators without a separate 
/// Logical type may include a Logical as a Number. The lookup will try to 
/// match an entry of value Lookup. If none is found the largest entry less 
/// than Lookup is taken as a match. From a sequence of identical values ≤ 
/// Lookup the last entry is taken. If there is no data less than or equal to 
/// Lookup, the #N/A Error is returned. If Lookup is of type Text and the value 
/// found is of type Number, the #N/A Error is returned. If DataSource is not 
/// sorted, the result is undetermined and implementation-dependent. In most 
/// cases it will be arbitrary and just plain wrong due to binary search 
/// algorithms.
/// 
/// If RangeLookup is FALSE or 0, DataSource does not need to be sorted and an 
/// exact match is searched. Each value in the first row of DataSource is 
/// examined in order (starting at the left) until its value matches Lookup.
/// 
/// Both methods, if there is a match, return the corresponding value in row 
/// Row, relative to the DataSource, where the topmost row in DataSource is 1.
/// 
/// The values returned may vary depending upon the 
/// HOST-USE-REGULAR-EXPRESSIONS or HOST-USE-WILDCARDS or 
/// HOST-SEARCH-CRITERIA-MUST-APPLY-TO-WHOLE-CELL properties. 3.4
///
/// See also: "INDEX", "MATCH", "OFFSET", "VLOOKUP", 
#[inline]
pub fn hlookup<A: Any, B: ReferenceOrArray, C: Number>(lookup: A, data_source: B, row: C) -> FnAny3<A, B, C> {
    FnAny3("HLOOKUP", lookup, data_source, row)
}

/// Look for a matching value in the first row of the given table, and return 
/// the value of the indicated row.
/// Syntax: HLOOKUP( Lookup Any;; DataSource Reference|Array;; Row Integer;[; RangeLookup Logical] )
///
/// Constraints:
/// Row ≥ 1; Searched portion of DataSource shall not include Logical values. 
/// Evaluators may evaluate expressions that do not meet the constraint that 
/// the searched portion of a DataSource not include Logical values.
///
/// Semantics:
/// 
/// If RangeLookup is omitted or TRUE or not 0, the first row of DataSource is 
/// assumed to be sorted in ascending order, with smaller numbers before larger 
/// ones, smaller text values before larger ones (e.g., "A" before "B", and "B" 
/// before "BA"), and FALSE before TRUE. If the types are mixed, Numbers are 
/// sorted before Text, and Text before Logicals; evaluators without a separate 
/// Logical type may include a Logical as a Number. The lookup will try to 
/// match an entry of value Lookup. If none is found the largest entry less 
/// than Lookup is taken as a match. From a sequence of identical values ≤ 
/// Lookup the last entry is taken. If there is no data less than or equal to 
/// Lookup, the #N/A Error is returned. If Lookup is of type Text and the value 
/// found is of type Number, the #N/A Error is returned. If DataSource is not 
/// sorted, the result is undetermined and implementation-dependent. In most 
/// cases it will be arbitrary and just plain wrong due to binary search 
/// algorithms.
/// 
/// If RangeLookup is FALSE or 0, DataSource does not need to be sorted and an 
/// exact match is searched. Each value in the first row of DataSource is 
/// examined in order (starting at the left) until its value matches Lookup.
/// 
/// Both methods, if there is a match, return the corresponding value in row 
/// Row, relative to the DataSource, where the topmost row in DataSource is 1.
/// 
/// The values returned may vary depending upon the 
/// HOST-USE-REGULAR-EXPRESSIONS or HOST-USE-WILDCARDS or 
/// HOST-SEARCH-CRITERIA-MUST-APPLY-TO-WHOLE-CELL properties. 3.4
///
/// See also: "INDEX", "MATCH", "OFFSET", "VLOOKUP", 
#[inline]
pub fn hlookup_<A: Any, B: ReferenceOrArray, C: Number, D: Logical>(lookup: A, data_source: B, row: C, range_lookup: D) -> FnAny4<A, B, C, D> {
    FnAny4("HLOOKUP", lookup, data_source, row, range_lookup)
}

/// Returns a value using a row and column index value (and optionally an area 
/// index).
/// Syntax: INDEX( DataSource ReferenceList|Array; )
///
/// Constraints:
/// Row ≥ 0, Column ≥ 0,
/// 1 ≤ AreaNumber ≤ number of references in DataSource if that is a 
/// ReferenceList, else AreaNumber = 1
///
/// Semantics:
/// 
/// Given a DataSource, returns the value at the given Row and Column (starting 
/// numbering at 1, relative to the top-left of the DataSource) of the given 
/// area AreaNumber. If AreaNumber is not given, it defaults to 1 (the first 
/// and possibly only area). This function is essentially a two-dimensional 
/// version of CHOOSE, which does not accept range parameters.
/// 
/// If Row is omitted or an empty parameter (two consecutive ;; semicolons) or 
/// 0, an entire column of the given area AreaNumber in DataSource is returned. 
/// If Column is omitted or an empty parameter (two consecutive ;; semicolons) 
/// or 0, an entire row of the given area AreaNumber in DataSource is returned. 
/// If both, Row and Column, are omitted or empty or 0, the entire given area 
/// AreaNumber is returned.
/// 
/// If DataSource is a one-dimensional column vector, Column is optional or can 
/// be omitted as an empty parameter (two consecutive ;; semicolons). If 
/// DataSource is a one-dimensional row vector, Row is optional, which 
/// effectively makes Row act as the column offset into the vector, or can be 
/// omitted as an empty parameter (two consecutive ;; semicolons).
/// 
/// If Row or Column have a value greater than the dimension of the 
/// corresponding given area AreaNumber, an Error is returned.
///
/// See also: "AREAS", "CHOOSE", 
#[inline]
pub fn index<A: ReferenceOrArray>(data_source: A) -> FnAny1<A> {
    FnAny1("INDEX", data_source)
}

/// Returns a value using a row and column index value (and optionally an area 
/// index).
/// Syntax: INDEX( DataSource ReferenceList|Array;[; Row Integer] )
///
/// Constraints:
/// Row ≥ 0, Column ≥ 0,
/// 1 ≤ AreaNumber ≤ number of references in DataSource if that is a 
/// ReferenceList, else AreaNumber = 1
///
/// Semantics:
/// 
/// Given a DataSource, returns the value at the given Row and Column (starting 
/// numbering at 1, relative to the top-left of the DataSource) of the given 
/// area AreaNumber. If AreaNumber is not given, it defaults to 1 (the first 
/// and possibly only area). This function is essentially a two-dimensional 
/// version of CHOOSE, which does not accept range parameters.
/// 
/// If Row is omitted or an empty parameter (two consecutive ;; semicolons) or 
/// 0, an entire column of the given area AreaNumber in DataSource is returned. 
/// If Column is omitted or an empty parameter (two consecutive ;; semicolons) 
/// or 0, an entire row of the given area AreaNumber in DataSource is returned. 
/// If both, Row and Column, are omitted or empty or 0, the entire given area 
/// AreaNumber is returned.
/// 
/// If DataSource is a one-dimensional column vector, Column is optional or can 
/// be omitted as an empty parameter (two consecutive ;; semicolons). If 
/// DataSource is a one-dimensional row vector, Row is optional, which 
/// effectively makes Row act as the column offset into the vector, or can be 
/// omitted as an empty parameter (two consecutive ;; semicolons).
/// 
/// If Row or Column have a value greater than the dimension of the 
/// corresponding given area AreaNumber, an Error is returned.
///
/// See also: "AREAS", "CHOOSE", 
#[inline]
pub fn index_<A: ReferenceOrArray, B: Number>(data_source: A, row: B) -> FnAny2<A, B> {
    FnAny2("INDEX", data_source, row)
}

/// Returns a value using a row and column index value (and optionally an area 
/// index).
/// Syntax: INDEX( DataSource ReferenceList|Array;[; Row Integer][; Column Integer] )
///
/// Constraints:
/// Row ≥ 0, Column ≥ 0,
/// 1 ≤ AreaNumber ≤ number of references in DataSource if that is a 
/// ReferenceList, else AreaNumber = 1
///
/// Semantics:
/// 
/// Given a DataSource, returns the value at the given Row and Column (starting 
/// numbering at 1, relative to the top-left of the DataSource) of the given 
/// area AreaNumber. If AreaNumber is not given, it defaults to 1 (the first 
/// and possibly only area). This function is essentially a two-dimensional 
/// version of CHOOSE, which does not accept range parameters.
/// 
/// If Row is omitted or an empty parameter (two consecutive ;; semicolons) or 
/// 0, an entire column of the given area AreaNumber in DataSource is returned. 
/// If Column is omitted or an empty parameter (two consecutive ;; semicolons) 
/// or 0, an entire row of the given area AreaNumber in DataSource is returned. 
/// If both, Row and Column, are omitted or empty or 0, the entire given area 
/// AreaNumber is returned.
/// 
/// If DataSource is a one-dimensional column vector, Column is optional or can 
/// be omitted as an empty parameter (two consecutive ;; semicolons). If 
/// DataSource is a one-dimensional row vector, Row is optional, which 
/// effectively makes Row act as the column offset into the vector, or can be 
/// omitted as an empty parameter (two consecutive ;; semicolons).
/// 
/// If Row or Column have a value greater than the dimension of the 
/// corresponding given area AreaNumber, an Error is returned.
///
/// See also: "AREAS", "CHOOSE", 
#[inline]
pub fn index__<A: ReferenceOrArray, B: Number, C: Number>(data_source: A, row: B, column: C) -> FnAny3<A, B, C> {
    FnAny3("INDEX", data_source, row, column)
}

/// Returns a value using a row and column index value (and optionally an area 
/// index).
/// Syntax: INDEX( DataSource ReferenceList|Array;[; Row Integer][; Column Integer][; AreaNumber Integer] )
///
/// Constraints:
/// Row ≥ 0, Column ≥ 0,
/// 1 ≤ AreaNumber ≤ number of references in DataSource if that is a 
/// ReferenceList, else AreaNumber = 1
///
/// Semantics:
/// 
/// Given a DataSource, returns the value at the given Row and Column (starting 
/// numbering at 1, relative to the top-left of the DataSource) of the given 
/// area AreaNumber. If AreaNumber is not given, it defaults to 1 (the first 
/// and possibly only area). This function is essentially a two-dimensional 
/// version of CHOOSE, which does not accept range parameters.
/// 
/// If Row is omitted or an empty parameter (two consecutive ;; semicolons) or 
/// 0, an entire column of the given area AreaNumber in DataSource is returned. 
/// If Column is omitted or an empty parameter (two consecutive ;; semicolons) 
/// or 0, an entire row of the given area AreaNumber in DataSource is returned. 
/// If both, Row and Column, are omitted or empty or 0, the entire given area 
/// AreaNumber is returned.
/// 
/// If DataSource is a one-dimensional column vector, Column is optional or can 
/// be omitted as an empty parameter (two consecutive ;; semicolons). If 
/// DataSource is a one-dimensional row vector, Row is optional, which 
/// effectively makes Row act as the column offset into the vector, or can be 
/// omitted as an empty parameter (two consecutive ;; semicolons).
/// 
/// If Row or Column have a value greater than the dimension of the 
/// corresponding given area AreaNumber, an Error is returned.
///
/// See also: "AREAS", "CHOOSE", 
#[inline]
pub fn index___<A: ReferenceOrArray, B: Number, C: Number, D: Number>(data_source: A, row: B, column: C, area_number: D) -> FnAny4<A, B, C, D> {
    FnAny4("INDEX", data_source, row, column, area_number)
}

/// Return a reference given a string representation of a reference.
/// Syntax: INDIRECT( Ref Text; )
///
/// Constraints:
/// Ref is valid reference
///
/// Semantics:
/// Given text for a reference (such as “A3”), returns a reference. If A1 
/// is False, it is interpreted as an R1C1 reference style. For 
/// interoperability, if the Ref text includes a sheet name, evaluators should 
/// be able to parse both, the “.” dot and the “!” exclamation mark, as 
/// the sheet name separator. If evaluators support the A1 = FALSE case of the 
/// ADDRESS 6.14.2 function and include the “!” exclamation mark as the 
/// sheet name separator, evaluators shall correctly parse that in the A1 = 
/// FALSE case of this INDIRECT function. Evaluators shall correctly parse the 
/// “.” dot as the sheet name separator in the A1 = TRUE case.
///
/// See also: "ADDRESS", 
#[inline]
pub fn indirect<A: Text>(ref_: A) -> FnReference1<A> {
    FnReference1("INDIRECT", ref_)
}

/// Return a reference given a string representation of a reference.
/// Syntax: INDIRECT( Ref Text;[; A1 Logical] )
///
/// Constraints:
/// Ref is valid reference
///
/// Semantics:
/// Given text for a reference (such as “A3”), returns a reference. If A1 
/// is False, it is interpreted as an R1C1 reference style. For 
/// interoperability, if the Ref text includes a sheet name, evaluators should 
/// be able to parse both, the “.” dot and the “!” exclamation mark, as 
/// the sheet name separator. If evaluators support the A1 = FALSE case of the 
/// ADDRESS 6.14.2 function and include the “!” exclamation mark as the 
/// sheet name separator, evaluators shall correctly parse that in the A1 = 
/// FALSE case of this INDIRECT function. Evaluators shall correctly parse the 
/// “.” dot as the sheet name separator in the A1 = TRUE case.
///
/// See also: "ADDRESS", 
#[inline]
pub fn indirect_<A: Text, B: Logical>(ref_: A, a1: B) -> FnReference2<A, B> {
    FnReference2("INDIRECT", ref_, a1)
}

/// Look for criterion in an already-sorted array, and return a corresponding 
/// result.
/// Syntax: LOOKUP( Find Any;; Searched Reference|Array; )
///
/// Constraints:
/// The searched portion of Searched shall be sorted in ascending order; if 
/// provided, Results shall have the same length as Searched. The searched 
/// portion of Searched shall not include Logical values. Evaluators may 
/// evaluate expressions that do not meet the constraint that the searched 
/// portion of a Searched not include Logical values.
///
/// Semantics:
/// This function searches for Find in a row or column of the previously-sorted 
/// array Searched and returns a corresponding value. The match is the largest 
/// value in the row/column of Searched that is less than or equal to Find (so 
/// an exact match is always preferred over inexact ones). From a sequence of 
/// identical values ≤ Find the last entry is taken. If Find is smaller than 
/// the smallest value in the first row or column (depending on the array 
/// dimensions), LOOKUP returns the #N/A Error. If Find is of type Text and the 
/// value found is of type Number, the #N/A Error is returned.
/// 
/// The searched portion of Searched shall be sorted in ascending order, and so 
/// that values of type Number precede values of type Text if both types are 
/// included (e.g., -2, 0, 2, “A”, “B”).
/// 
/// There are two major uses for this function; the 3-parameter version 
/// (vector) and the 2-parameter version (non-vector array).
///
/// Note:
/// Interoperability is improved by use of HLOOKUP or VLOOKUP in expressions 
/// over LOOKUP.
/// 
/// When given two parameters, Searched is first examined:
/// 
/// •If Searched is square or is taller than it is wide (more rows than 
/// columns), LOOKUP searches in the first column (similar to VLOOKUP), and 
/// returns the corresponding value in the last column.
/// 
/// •If Searched covers an area that is wider than it is tall (more columns 
/// than rows), LOOKUP searches in the first row (similar to HLOOKUP), and 
/// returns the corresponding value in the last row.
/// 
/// When given 3 parameters, Results shall be a vector (either a row or a 
/// column) or an Error is raised. The function determines the index of the 
/// match in the first column respectively row of Searched, and returns the 
/// value in Results with the same index.
/// 
/// Searched is first examined:
/// 
/// •If Searched is square or is taller than it is wide (more rows than 
/// columns), LOOKUP searches in the first column (similar to VLOOKUP).
/// 
/// •If Searched covers an area that is wider than it is tall (more columns 
/// than rows), LOOKUP searches in the first row (similar to HLOOKUP).
/// 
/// The lengths of the search vector and the result vector do not need to be 
/// identical. When the match position falls outside the length of the result 
/// vector, an Error is returned if the result vector is given as an array 
/// object. If it is a cell range, it gets automatically extended to the length 
/// of the searched vector, but in the direction of the result vector. If just 
/// a single cell reference was passed, a column vector is generated. If the 
/// cell range cannot be extended due to the sheet's size limit, then the #N/A 
/// Error is returned.
/// 
/// The values returned may vary depending upon the 
/// HOST-USE-REGULAR-EXPRESSIONS or HOST-USE-WILDCARDS or 
/// HOST-SEARCH-CRITERIA-MUST-APPLY-TO-WHOLE-CELL properties. 3.4
///
/// See also: "HLOOKUP", "INDEX", "MATCH", "OFFSET", "VLOOKUP", 
#[inline]
pub fn lookup<A: Any, B: ReferenceOrArray>(find: A, searched: B) -> FnAny2<A, B> {
    FnAny2("LOOKUP", find, searched)
}

/// Look for criterion in an already-sorted array, and return a corresponding 
/// result.
/// Syntax: LOOKUP( Find Any;; Searched Reference|Array;[; Results Reference|Array] )
///
/// Constraints:
/// The searched portion of Searched shall be sorted in ascending order; if 
/// provided, Results shall have the same length as Searched. The searched 
/// portion of Searched shall not include Logical values. Evaluators may 
/// evaluate expressions that do not meet the constraint that the searched 
/// portion of a Searched not include Logical values.
///
/// Semantics:
/// This function searches for Find in a row or column of the previously-sorted 
/// array Searched and returns a corresponding value. The match is the largest 
/// value in the row/column of Searched that is less than or equal to Find (so 
/// an exact match is always preferred over inexact ones). From a sequence of 
/// identical values ≤ Find the last entry is taken. If Find is smaller than 
/// the smallest value in the first row or column (depending on the array 
/// dimensions), LOOKUP returns the #N/A Error. If Find is of type Text and the 
/// value found is of type Number, the #N/A Error is returned.
/// 
/// The searched portion of Searched shall be sorted in ascending order, and so 
/// that values of type Number precede values of type Text if both types are 
/// included (e.g., -2, 0, 2, “A”, “B”).
/// 
/// There are two major uses for this function; the 3-parameter version 
/// (vector) and the 2-parameter version (non-vector array).
///
/// Note:
/// Interoperability is improved by use of HLOOKUP or VLOOKUP in expressions 
/// over LOOKUP.
/// 
/// When given two parameters, Searched is first examined:
/// 
/// •If Searched is square or is taller than it is wide (more rows than 
/// columns), LOOKUP searches in the first column (similar to VLOOKUP), and 
/// returns the corresponding value in the last column.
/// 
/// •If Searched covers an area that is wider than it is tall (more columns 
/// than rows), LOOKUP searches in the first row (similar to HLOOKUP), and 
/// returns the corresponding value in the last row.
/// 
/// When given 3 parameters, Results shall be a vector (either a row or a 
/// column) or an Error is raised. The function determines the index of the 
/// match in the first column respectively row of Searched, and returns the 
/// value in Results with the same index.
/// 
/// Searched is first examined:
/// 
/// •If Searched is square or is taller than it is wide (more rows than 
/// columns), LOOKUP searches in the first column (similar to VLOOKUP).
/// 
/// •If Searched covers an area that is wider than it is tall (more columns 
/// than rows), LOOKUP searches in the first row (similar to HLOOKUP).
/// 
/// The lengths of the search vector and the result vector do not need to be 
/// identical. When the match position falls outside the length of the result 
/// vector, an Error is returned if the result vector is given as an array 
/// object. If it is a cell range, it gets automatically extended to the length 
/// of the searched vector, but in the direction of the result vector. If just 
/// a single cell reference was passed, a column vector is generated. If the 
/// cell range cannot be extended due to the sheet's size limit, then the #N/A 
/// Error is returned.
/// 
/// The values returned may vary depending upon the 
/// HOST-USE-REGULAR-EXPRESSIONS or HOST-USE-WILDCARDS or 
/// HOST-SEARCH-CRITERIA-MUST-APPLY-TO-WHOLE-CELL properties. 3.4
///
/// See also: "HLOOKUP", "INDEX", "MATCH", "OFFSET", "VLOOKUP", 
#[inline]
pub fn lookup_<A: Any, B: ReferenceOrArray, C: ReferenceOrArray>(find: A, searched: B, results: C) -> FnAny3<A, B, C> {
    FnAny3("LOOKUP", find, searched, results)
}

/// Finds a Search item in a sequence, and returns its position (starting from 
/// 1).
/// Syntax: MATCH( Search Scalar;; SearchRegion Reference|Array; )
///
/// Constraints:
/// -1 ≤ MatchType ≤ 1; The searched portion of SearchRegion shall not 
/// include Logical values. Evaluators may evaluate expressions that do not 
/// meet the constraint that the searched portion of a SearchRegion not include 
/// Logical values.
/// 
/// SearchRegion shall be a vector (a single row or column)
///
/// Semantics:
/// 
/// •MatchType = -1 finds the smallest value that is greater than or equal to 
/// Search in a SearchRegion where values are sorted in descending order. From 
/// a sequence of identical values ≥ Search the last value is taken. If no 
/// value ≥ Search exists, the #N/A Error is returned. If Search is of type 
/// Number and the value found is of type Text, the #N/A Error is returned.
/// 
/// •MatchType = 0 finds the first value that is equal to Search. Values in 
/// SearchRegion do not need to be sorted. If no value equal to Search exists, 
/// the #N/A Error is returned.
/// 
/// •MatchType = 1 or omitted finds the largest value that is less than or 
/// equal to Search in a SearchRegion where values are sorted in ascending 
/// order. From a sequence of identical values ≤ Search the last value is 
/// taken. If no value ≤ Search exists, the #N/A Error is returned. If Search 
/// is of type Text and the value found is of type Number, the #N/A Error is 
/// returned.
/// 
/// If a match is found, MATCH returns the relative position (starting from 1). 
/// For Text the comparison is case-insensitive. MatchType determines the type 
/// of search; if MatchType is 0, the SearchRegion shall be considered 
/// unsorted, and the first match is returned. If MatchType is 1, the 
/// SearchRegion may be assumed to be sorted in ascending order, with smaller 
/// Numbers before larger ones, smaller Text values before larger ones (e.g., 
/// "A" before "B", and "B" before "BA"), and FALSE before TRUE. If the types 
/// are mixed, Numbers are sorted before Text, and Text before Logicals; 
/// evaluators without a separate Logical type may include a Logical as a 
/// Number. If MatchType is -1, then SearchRegion may be assumed to be sorted 
/// in descending order (the opposite of the above). If MatchType is 1 or -1, 
/// evaluators may use binary search or other techniques so that they do not 
/// need to examine every value in linear order. MatchType defaults to 1.
/// 
/// The values returned may vary depending upon the 
/// HOST-USE-REGULAR-EXPRESSIONS or HOST-USE-WILDCARDS or 
/// HOST-SEARCH-CRITERIA-MUST-APPLY-TO-WHOLE-CELL properties. 3.4
///
/// See also: "HLOOKUP", "OFFSET", "VLOOKUP", 
#[inline]
pub fn match_<A: Scalar, B: ReferenceOrArray>(search: A, search_region: B) -> FnAny2<A, B> {
    FnAny2("MATCH", search, search_region)
}

/// Finds a Search item in a sequence, and returns its position (starting from 
/// 1).
/// Syntax: MATCH( Search Scalar;; SearchRegion Reference|Array;[; MatchType Integer] )
///
/// Constraints:
/// -1 ≤ MatchType ≤ 1; The searched portion of SearchRegion shall not 
/// include Logical values. Evaluators may evaluate expressions that do not 
/// meet the constraint that the searched portion of a SearchRegion not include 
/// Logical values.
/// 
/// SearchRegion shall be a vector (a single row or column)
///
/// Semantics:
/// 
/// •MatchType = -1 finds the smallest value that is greater than or equal to 
/// Search in a SearchRegion where values are sorted in descending order. From 
/// a sequence of identical values ≥ Search the last value is taken. If no 
/// value ≥ Search exists, the #N/A Error is returned. If Search is of type 
/// Number and the value found is of type Text, the #N/A Error is returned.
/// 
/// •MatchType = 0 finds the first value that is equal to Search. Values in 
/// SearchRegion do not need to be sorted. If no value equal to Search exists, 
/// the #N/A Error is returned.
/// 
/// •MatchType = 1 or omitted finds the largest value that is less than or 
/// equal to Search in a SearchRegion where values are sorted in ascending 
/// order. From a sequence of identical values ≤ Search the last value is 
/// taken. If no value ≤ Search exists, the #N/A Error is returned. If Search 
/// is of type Text and the value found is of type Number, the #N/A Error is 
/// returned.
/// 
/// If a match is found, MATCH returns the relative position (starting from 1). 
/// For Text the comparison is case-insensitive. MatchType determines the type 
/// of search; if MatchType is 0, the SearchRegion shall be considered 
/// unsorted, and the first match is returned. If MatchType is 1, the 
/// SearchRegion may be assumed to be sorted in ascending order, with smaller 
/// Numbers before larger ones, smaller Text values before larger ones (e.g., 
/// "A" before "B", and "B" before "BA"), and FALSE before TRUE. If the types 
/// are mixed, Numbers are sorted before Text, and Text before Logicals; 
/// evaluators without a separate Logical type may include a Logical as a 
/// Number. If MatchType is -1, then SearchRegion may be assumed to be sorted 
/// in descending order (the opposite of the above). If MatchType is 1 or -1, 
/// evaluators may use binary search or other techniques so that they do not 
/// need to examine every value in linear order. MatchType defaults to 1.
/// 
/// The values returned may vary depending upon the 
/// HOST-USE-REGULAR-EXPRESSIONS or HOST-USE-WILDCARDS or 
/// HOST-SEARCH-CRITERIA-MUST-APPLY-TO-WHOLE-CELL properties. 3.4
///
/// See also: "HLOOKUP", "OFFSET", "VLOOKUP", 
#[inline]
pub fn match__<A: Scalar, B: ReferenceOrArray>(search: A, search_region: B, match_type: MatchType) -> FnAny3<A, B, MatchType> {
    FnAny3("MATCH", search, search_region, match_type)
}

/// Executes a formula expression while substituting a row reference and a 
/// column reference.
/// Syntax: MULTIPLE.OPERATIONS( FormulaCell Reference;; RowCell Reference;; RowReplacement Reference; )
///
/// Semantics:
/// 
/// •FormulaCell: reference to the cell that contains the formula expression 
/// to calculate.
/// 
/// •RowCell: reference that is to be replaced by RowReplacement.
/// 
/// •RowReplacement: reference that replaces RowCell.
/// 
/// •ColumnCell: reference that is to be replaced by ColumnReplacement.
/// 
/// •ColumnReplacement: reference that replaces ColumnCell.
/// 
/// MULTIPLE.OPERATIONS executes the formula expression pointed to by 
/// FormulaCell and all formula expressions it depends on while replacing all 
/// references to RowCell with references to RowReplacement respectively all 
/// references to ColumnCell with references to ColumnReplacement.
/// 
/// If calls to MULTIPLE.OPERATIONS are encountered in dependencies, 
/// replacements of target cells shall occur in queued order, with each 
/// replacement using the result of the previous replacement.
///
/// Note:
/// The function may be used to create tables of expressions that depend on two 
/// input parameters.
#[inline]
pub fn multiple_operations<A: Reference, B: Reference, C: Reference>(formula_cell: A, row_cell: B, row_replacement: C) -> FnAny3<A, B, C> {
    FnAny3("MULTIPLE.OPERATIONS", formula_cell, row_cell, row_replacement)
}

/// Executes a formula expression while substituting a row reference and a 
/// column reference.
/// Syntax: MULTIPLE.OPERATIONS( FormulaCell Reference;; RowCell Reference;; RowReplacement Reference;[; ColumnCell Reference] )
///
/// Semantics:
/// 
/// •FormulaCell: reference to the cell that contains the formula expression 
/// to calculate.
/// 
/// •RowCell: reference that is to be replaced by RowReplacement.
/// 
/// •RowReplacement: reference that replaces RowCell.
/// 
/// •ColumnCell: reference that is to be replaced by ColumnReplacement.
/// 
/// •ColumnReplacement: reference that replaces ColumnCell.
/// 
/// MULTIPLE.OPERATIONS executes the formula expression pointed to by 
/// FormulaCell and all formula expressions it depends on while replacing all 
/// references to RowCell with references to RowReplacement respectively all 
/// references to ColumnCell with references to ColumnReplacement.
/// 
/// If calls to MULTIPLE.OPERATIONS are encountered in dependencies, 
/// replacements of target cells shall occur in queued order, with each 
/// replacement using the result of the previous replacement.
///
/// Note:
/// The function may be used to create tables of expressions that depend on two 
/// input parameters.
#[inline]
pub fn multiple_operations_<A: Reference, B: Reference, C: Reference, D: Reference>(formula_cell: A, row_cell: B, row_replacement: C, column_cell: D) -> FnAny4<A, B, C, D> {
    FnAny4("MULTIPLE.OPERATIONS", formula_cell, row_cell, row_replacement, column_cell)
}

/// Executes a formula expression while substituting a row reference and a 
/// column reference.
/// Syntax: MULTIPLE.OPERATIONS( FormulaCell Reference;; RowCell Reference;; RowReplacement Reference;[; ColumnCell Reference][; ColumnReplacement Reference] )
///
/// Semantics:
/// 
/// •FormulaCell: reference to the cell that contains the formula expression 
/// to calculate.
/// 
/// •RowCell: reference that is to be replaced by RowReplacement.
/// 
/// •RowReplacement: reference that replaces RowCell.
/// 
/// •ColumnCell: reference that is to be replaced by ColumnReplacement.
/// 
/// •ColumnReplacement: reference that replaces ColumnCell.
/// 
/// MULTIPLE.OPERATIONS executes the formula expression pointed to by 
/// FormulaCell and all formula expressions it depends on while replacing all 
/// references to RowCell with references to RowReplacement respectively all 
/// references to ColumnCell with references to ColumnReplacement.
/// 
/// If calls to MULTIPLE.OPERATIONS are encountered in dependencies, 
/// replacements of target cells shall occur in queued order, with each 
/// replacement using the result of the previous replacement.
///
/// Note:
/// The function may be used to create tables of expressions that depend on two 
/// input parameters.
#[inline]
pub fn multiple_operations__<A: Reference, B: Reference, C: Reference, D: Reference, E: Reference>(formula_cell: A, row_cell: B, row_replacement: C, column_cell: D, column_replacement: E) -> FnAny5<A, B, C, D, E> {
    FnAny5("MULTIPLE.OPERATIONS", formula_cell, row_cell, row_replacement, column_cell, column_replacement)
}

/// Modifies a reference's position and dimension.
/// Syntax: OFFSET( R Reference;; RowOffset Integer;; ColumnOffset Integer; )
///
/// Constraints:
/// NewWidth > 0; NewHeight > 0
/// The modified reference shall be in a valid range, starting from column/row 
/// one to the maximum column/row.
///
/// Semantics:
/// Shifts reference by RowOffset rows and by ColumnOffset columns. Optionally, 
/// the dimension can be set to NewWidth and/or NewHeight, if omitted the 
/// width/height of the original reference is taken. Note that NewHeight may be 
/// empty (two consecutive semicolons ;;) followed by a given NewWidth 
/// argument. Returns the modified reference.
///
/// See also: "COLUMN", "COLUMNS", "ROW", "ROWS", 
#[inline]
pub fn offset<A: Reference, B: Number, C: Number>(r: A, row_offset: B, column_offset: C) -> FnReference3<A, B, C> {
    FnReference3("OFFSET", r, row_offset, column_offset)
}

/// Modifies a reference's position and dimension.
/// Syntax: OFFSET( R Reference;; RowOffset Integer;; ColumnOffset Integer;[; NewHeight Integer] )
///
/// Constraints:
/// NewWidth > 0; NewHeight > 0
/// The modified reference shall be in a valid range, starting from column/row 
/// one to the maximum column/row.
///
/// Semantics:
/// Shifts reference by RowOffset rows and by ColumnOffset columns. Optionally, 
/// the dimension can be set to NewWidth and/or NewHeight, if omitted the 
/// width/height of the original reference is taken. Note that NewHeight may be 
/// empty (two consecutive semicolons ;;) followed by a given NewWidth 
/// argument. Returns the modified reference.
///
/// See also: "COLUMN", "COLUMNS", "ROW", "ROWS", 
#[inline]
pub fn offset_<A: Reference, B: Number, C: Number, D: Number>(r: A, row_offset: B, column_offset: C, new_height: D) -> FnReference4<A, B, C, D> {
    FnReference4("OFFSET", r, row_offset, column_offset, new_height)
}

/// Modifies a reference's position and dimension.
/// Syntax: OFFSET( R Reference;; RowOffset Integer;; ColumnOffset Integer;[; NewHeight Integer][; NewWidth Integer] )
///
/// Constraints:
/// NewWidth > 0; NewHeight > 0
/// The modified reference shall be in a valid range, starting from column/row 
/// one to the maximum column/row.
///
/// Semantics:
/// Shifts reference by RowOffset rows and by ColumnOffset columns. Optionally, 
/// the dimension can be set to NewWidth and/or NewHeight, if omitted the 
/// width/height of the original reference is taken. Note that NewHeight may be 
/// empty (two consecutive semicolons ;;) followed by a given NewWidth 
/// argument. Returns the modified reference.
///
/// See also: "COLUMN", "COLUMNS", "ROW", "ROWS", 
#[inline]
pub fn offset__<A: Reference, B: Number, C: Number, D: Number, E: Number>(r: A, row_offset: B, column_offset: C, new_height: D, new_width: E) -> FnReference5<A, B, C, D, E> {
    FnReference5("OFFSET", r, row_offset, column_offset, new_height, new_width)
}

/// Look for a matching value in the first column of the given table, and 
/// return the value of the indicated column.
/// Syntax: VLOOKUP( Lookup Any;; DataSource Reference|Array;; Column Integer; )
///
/// Constraints:
/// Column ≥ 1; The searched portion of DataSource shall not include Logical 
/// values. Evaluators may evaluate expressions that do not meet the constraint 
/// that the searched portion of a DataSource not include Logical values.
///
/// Semantics:
/// 
/// If RangeLookup is omitted or TRUE or not 0, the first column of DataSource 
/// is assumed to be sorted in ascending order, with smaller Numbers before 
/// larger ones, smaller Text values before larger ones (e.g., "A" before "B", 
/// and "B" before "BA"), and FALSE before TRUE. If the types are mixed, 
/// Numbers are sorted before Text, and Text before Logicals; evaluators 
/// without a separate Logical type may include a Logical as a Number. The 
/// lookup will try to match an entry of value Lookup. From a sequence of 
/// identical values ≤ Lookup the last entry is taken. If none is found the 
/// largest entry less than Lookup is taken as a match. If there is no data 
/// less than or equal to Lookup, the #N/A Error is returned. If Lookup is of 
/// type Text and the value found is of type Number, the #N/A Error is 
/// returned. If DataSource is not sorted, the result is undetermined and 
/// implementation-dependent. In most cases it will be arbitrary and just plain 
/// wrong due to binary search algorithms.
/// 
/// If RangeLookup is FALSE or 0, DataSource does not need to be sorted and an 
/// exact match is searched. Each value in the first column of DataSource is 
/// examined in order (starting at the top) until its value matches Lookup. If 
/// no value matches, the #N/A Error is returned.
/// 
/// Both methods, if there is a match, return the corresponding value in column 
/// Column, relative to the DataSource, where the leftmost column in DataSource 
/// is 1.
/// 
/// The values returned may vary depending upon the 
/// HOST-USE-REGULAR-EXPRESSIONS or HOST-USE-WILDCARDS or 
/// HOST-SEARCH-CRITERIA-MUST-APPLY-TO-WHOLE-CELL properties. 3.4
///
/// See also: "HLOOKUP", "INDEX", "MATCH", "OFFSET", 
#[inline]
pub fn vlookup<A: Any, B: ReferenceOrArray, C: Number>(lookup: A, data_source: B, column: C) -> FnAny3<A, B, C> {
    FnAny3("VLOOKUP", lookup, data_source, column)
}

/// Look for a matching value in the first column of the given table, and 
/// return the value of the indicated column.
/// Syntax: VLOOKUP( Lookup Any;; DataSource Reference|Array;; Column Integer;[; RangeLookup Logical] )
///
/// Constraints:
/// Column ≥ 1; The searched portion of DataSource shall not include Logical 
/// values. Evaluators may evaluate expressions that do not meet the constraint 
/// that the searched portion of a DataSource not include Logical values.
///
/// Semantics:
/// 
/// If RangeLookup is omitted or TRUE or not 0, the first column of DataSource 
/// is assumed to be sorted in ascending order, with smaller Numbers before 
/// larger ones, smaller Text values before larger ones (e.g., "A" before "B", 
/// and "B" before "BA"), and FALSE before TRUE. If the types are mixed, 
/// Numbers are sorted before Text, and Text before Logicals; evaluators 
/// without a separate Logical type may include a Logical as a Number. The 
/// lookup will try to match an entry of value Lookup. From a sequence of 
/// identical values ≤ Lookup the last entry is taken. If none is found the 
/// largest entry less than Lookup is taken as a match. If there is no data 
/// less than or equal to Lookup, the #N/A Error is returned. If Lookup is of 
/// type Text and the value found is of type Number, the #N/A Error is 
/// returned. If DataSource is not sorted, the result is undetermined and 
/// implementation-dependent. In most cases it will be arbitrary and just plain 
/// wrong due to binary search algorithms.
/// 
/// If RangeLookup is FALSE or 0, DataSource does not need to be sorted and an 
/// exact match is searched. Each value in the first column of DataSource is 
/// examined in order (starting at the top) until its value matches Lookup. If 
/// no value matches, the #N/A Error is returned.
/// 
/// Both methods, if there is a match, return the corresponding value in column 
/// Column, relative to the DataSource, where the leftmost column in DataSource 
/// is 1.
/// 
/// The values returned may vary depending upon the 
/// HOST-USE-REGULAR-EXPRESSIONS or HOST-USE-WILDCARDS or 
/// HOST-SEARCH-CRITERIA-MUST-APPLY-TO-WHOLE-CELL properties. 3.4
///
/// See also: "HLOOKUP", "INDEX", "MATCH", "OFFSET", 
#[inline]
pub fn vlookup_<A: Any, B: ReferenceOrArray, C: Number, D: Logical>(lookup: A, data_source: B, column: C, range_lookup: D) -> FnAny4<A, B, C, D> {
    FnAny4("VLOOKUP", lookup, data_source, column, range_lookup)
}
