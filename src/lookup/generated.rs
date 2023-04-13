use crate::*;
#[allow(unused_imports)]
use super::*;

///  Returns a cell address (reference) as text
#[inline]
pub fn address___<A: Number, B: Number, C: Text>(row: A, col: B, abs: AddressAbs, a1_style: bool, sheet: C) -> FnText5<A, B, AddressAbs, bool, C> {
    FnText5("ADDRESS", row, col, abs, a1_style, sheet)
}

///  Returns a cell address (reference) as text
#[inline]
pub fn address__<A: Number, B: Number>(row: A, col: B, abs: AddressAbs, a1_style: bool) -> FnText4<A, B, AddressAbs, bool> {
    FnText4("ADDRESS", row, col, abs, a1_style)
}

///  Returns a cell address (reference) as text
#[inline]
pub fn address_<A: Number, B: Number>(row: A, col: B, abs: AddressAbs) -> FnText3<A, B, AddressAbs> {
    FnText3("ADDRESS", row, col, abs)
}

///  Returns a cell address (reference) as text
#[inline]
pub fn address<A: Number, B: Number>(row: A, col: B) -> FnText2<A, B> {
    FnText2("ADDRESS", row, col)
}

/// Uses an index to return a value from a list of values
#[inline]
pub fn choose<A: Number, B: Sequence>(idx: A, values: B) -> FnAny2<A, B> {
    FnAny2("CHOOSE", idx, values)
}

///  Look for a matching value in the first row of the given table, and return the value of the indicated row.
#[inline]
pub fn hlookup_<A: Any, B: Array, C: Number>(lookup: A, data_source: B, row: C, range_lookup: bool) -> FnAny4<A, B, C, bool> {
    FnAny4("HLOOKUP", lookup, data_source, row, range_lookup)
}

///  Look for a matching value in the first row of the given table, and return the value of the indicated row.
#[inline]
pub fn hlookup<A: Any, B: Array, C: Number>(lookup: A, data_source: B, row: C) -> FnAny3<A, B, C> {
    FnAny3("HLOOKUP", lookup, data_source, row)
}

///  Returns a value using a row and column index value (and optionally an area index).
#[inline]
pub fn index__<A: Reference, B: Number, C: Number, D: Number>(data_source: A, row: Option<B>, col: C, area_number: D) -> FnAny4<A, Option<B>, C, D> {
    FnAny4("INDEX", data_source, row, col, area_number)
}

///  Returns a value using a row and column index value (and optionally an area index).
#[inline]
pub fn index_<A: Reference, B: Number, C: Number>(data_source: A, row: Option<B>, col: C) -> FnAny3<A, Option<B>, C> {
    FnAny3("INDEX", data_source, row, col)
}

///  Returns a value using a row and column index value (and optionally an area index).
#[inline]
pub fn index<A: Reference, B: Number>(data_source: A, row: Option<B>) -> FnAny2<A, Option<B>> {
    FnAny2("INDEX", data_source, row)
}

///  Return a reference given a string representation of a reference.
#[inline]
pub fn indirect_<A: Text>(refs: A, format_a1: bool) -> FnReference2<A, bool> {
    FnReference2("INDIRECT", refs, format_a1)
}

///  Return a reference given a string representation of a reference.
#[inline]
pub fn indirect<A: Text>(refs: A) -> FnReference1<A> {
    FnReference1("INDIRECT", refs)
}

///  Look for criterion in an already-sorted array, and return a corresponding result
#[inline]
pub fn lookup_<A: Any, B: Array, C: Array>(find: A, searched: B, results: C) -> FnAny3<A, B, C> {
    FnAny3("LOOKUP", find, searched, results)
}

///  Look for criterion in an already-sorted array, and return a corresponding result
#[inline]
pub fn lookup<A: Any, B: Array>(find: A, searched: B) -> FnAny2<A, B> {
    FnAny2("LOOKUP", find, searched)
}

/// Finds a Search item in a sequence, and returns its position (starting from 1)
#[inline]
pub fn match_<A: Scalar, B: Array>(search: A, search_region: B, match_type: MatchType) -> FnAny3<A, B, MatchType> {
    FnAny3("MATCH", search, search_region, match_type)
}

/// Finds a Search item in a sequence, and returns its position (starting from 1)
#[inline]
pub fn match_<A: Scalar, B: Array>(search: A, search_region: B) -> FnAny2<A, B> {
    FnAny2("MATCH", search, search_region)
}

///  Executes a formula expression while substituting a row reference and a column reference.
#[inline]
pub fn multiple_operations__<A: Reference, B: Reference, C: Reference, D: Reference, E: Reference>(formula_cell: A, row_cell: B, row_replacement: C, column_cell: D, column_replacement: E) -> FnAny5<A, B, C, D, E> {
    FnAny5("MULTIPLE.OPERATIONS", formula_cell, row_cell, row_replacement, column_cell, column_replacement)
}

///  Executes a formula expression while substituting a row reference and a column reference.
#[inline]
pub fn multiple_operations_<A: Reference, B: Reference, C: Reference, D: Reference>(formula_cell: A, row_cell: B, row_replacement: C, column_cell: D) -> FnAny4<A, B, C, D> {
    FnAny4("MULTIPLE.OPERATIONS", formula_cell, row_cell, row_replacement, column_cell)
}

///  Executes a formula expression while substituting a row reference and a column reference.
#[inline]
pub fn multiple_operations<A: Reference, B: Reference, C: Reference>(formula_cell: A, row_cell: B, row_replacement: C) -> FnAny3<A, B, C> {
    FnAny3("MULTIPLE.OPERATIONS", formula_cell, row_cell, row_replacement)
}

///  Modifies a reference's position and dimension.
#[inline]
pub fn offset__<A: Reference, B: Number, C: Number, D: Number, E: Number>(re: A, row_offset: B, column_offset: C, new_height: D, new_width: E) -> FnReference5<A, B, C, D, E> {
    FnReference5("OFFSET", re, row_offset, column_offset, new_height, new_width)
}

///  Modifies a reference's position and dimension.
#[inline]
pub fn offset_<A: Reference, B: Number, C: Number, D: Number>(re: A, row_offset: B, column_offset: C, new_height: D) -> FnReference4<A, B, C, D> {
    FnReference4("OFFSET", re, row_offset, column_offset, new_height)
}

///  Modifies a reference's position and dimension.
#[inline]
pub fn offset<A: Reference, B: Number, C: Number>(re: A, row_offset: B, column_offset: C) -> FnReference3<A, B, C> {
    FnReference3("OFFSET", re, row_offset, column_offset)
}

///  Look for a matching value in the first column of the given table, and return the value of the indicated column.
#[inline]
pub fn vlookup_<A: Any, B: Array, C: Number>(lookup: A, data_source: B, column: C, range_lookup: bool) -> FnAny4<A, B, C, bool> {
    FnAny4("VLOOKUP", lookup, data_source, column, range_lookup)
}

///  Look for a matching value in the first column of the given table, and return the value of the indicated column.
#[inline]
pub fn vlookup<A: Any, B: Array, C: Number>(lookup: A, data_source: B, column: C) -> FnAny3<A, B, C> {
    FnAny3("VLOOKUP", lookup, data_source, column)
}
