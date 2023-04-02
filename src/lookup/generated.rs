use crate::*;
#[allow(unused_imports)]
use super::*;

/// Uses an index to return a value from a list of values
#[inline]
pub fn choose<A: Number, B: Sequence>(idx: A, values: B) -> FnAny2<A, B> {
    FnAny2("CHOOSE", idx, values)
}

///  Look for a matching value in the first row of the given table, and return the value of the indicated row.
#[inline]
pub fn hlookup<A: Any, B: Reference, C: Number>(lookup: A, data_source: B, row: C, range_lookup: bool) -> FnAny4<A, B, C, bool> {
    FnAny4("HLOOKUP", lookup, data_source, row, range_lookup)
}

///  Returns a value using a row and column index value (and optionally an area index).
#[inline]
pub fn index<A: Reference, B: Number, C: Number, D: Number>(data_source: A, row: Option<B>, col: Option<C>, area_number: Option<D>) -> FnAny4<A, Option<B>, Option<C>, Option<D>> {
    FnAny4("INDEX", data_source, row, col, area_number)
}

///  Return a reference given a string representation of a reference.
#[inline]
pub fn indirect<A: Text>(refs: A, format_a1: bool) -> FnReference2<A, bool> {
    FnReference2("INDIRECT", refs, format_a1)
}

///  Look for criterion in an already-sorted array, and return a corresponding result
#[inline]
pub fn lookup<A: Any, B: Reference, C: Reference>(find: A, searched: B, results: C) -> FnAny3<A, B, C> {
    FnAny3("LOOKUP", find, searched, results)
}

/// Finds a Search item in a sequence, and returns its position (starting from 1)
#[inline]
pub fn match_<A: Scalar, B: Reference>(search: A, search_region: B, match_type: Option<MatchType>) -> FnAny3<A, B, Option<MatchType>> {
    FnAny3("MATCH", search, search_region, match_type)
}

///  Executes a formula expression while substituting a row reference and a column reference.
#[inline]
pub fn multiple_operations<A: Reference, B: Reference, C: Reference, D: Reference, E: Reference>(formula_cell: A, row_cell: B, row_replacement: C, column_cell: Option<D>, column_replacement: Option<E>) -> FnAny5<A, B, C, Option<D>, Option<E>> {
    FnAny5("MULTIPLE.OPERATIONS", formula_cell, row_cell, row_replacement, column_cell, column_replacement)
}

///  Modifies a reference's position and dimension.
#[inline]
pub fn offset<A: Reference, B: Number, C: Number, D: Number, E: Number>(re: A, row_offset: B, column_offset: C, new_height: D, new_width: E) -> FnReference5<A, B, C, D, E> {
    FnReference5("OFFSET", re, row_offset, column_offset, new_height, new_width)
}

///  Look for a matching value in the first column of the given table, and return the value of the indicated column.
#[inline]
pub fn vlookup<A: Any, B: Reference, C: Number>(lookup: A, data_source: B, column: C, range_lookup: bool) -> FnAny4<A, B, C, bool> {
    FnAny4("VLOOKUP", lookup, data_source, column, range_lookup)
}
