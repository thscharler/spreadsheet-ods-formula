use crate::*;
#[allow(unused_imports)]
use crate::lookup::*;

#[inline]
pub fn address<A: Number, B: Number>(row: A, column: B) -> FnText2<A, B> {
    FnText2("ADDRESS", row, column)
}

#[inline]
pub fn address_<A: Number, B: Number>(row: A, column: B, abs: AddressAbs) -> FnText3<A, B, AddressAbs> {
    FnText3("ADDRESS", row, column, abs)
}

#[inline]
pub fn address__<A: Number, B: Number, C: Logical>(row: A, column: B, abs: AddressAbs, a1_style: C) -> FnText4<A, B, AddressAbs, C> {
    FnText4("ADDRESS", row, column, abs, a1_style)
}

#[inline]
pub fn address___<A: Number, B: Number, C: Logical, D: Text>(row: A, column: B, abs: AddressAbs, a1_style: C, sheet: D) -> FnText5<A, B, AddressAbs, C, D> {
    FnText5("ADDRESS", row, column, abs, a1_style, sheet)
}

#[inline]
pub fn choose<A: Number, B: Sequence>(index: A, value: B) -> FnAny2<A, B> {
    FnAny2("CHOOSE", index, value)
}

#[inline]
pub fn hlookup<A: Any, B: ReferenceOrArray, C: Number>(lookup: A, data_source: B, row: C) -> FnAny3<A, B, C> {
    FnAny3("HLOOKUP", lookup, data_source, row)
}

#[inline]
pub fn hlookup_<A: Any, B: ReferenceOrArray, C: Number, D: Logical>(lookup: A, data_source: B, row: C, range_lookup: D) -> FnAny4<A, B, C, D> {
    FnAny4("HLOOKUP", lookup, data_source, row, range_lookup)
}

#[inline]
pub fn index<A: ReferenceOrArray>(data_source: A) -> FnAny1<A> {
    FnAny1("INDEX", data_source)
}

#[inline]
pub fn index_<A: ReferenceOrArray, B: Number>(data_source: A, row: B) -> FnAny2<A, B> {
    FnAny2("INDEX", data_source, row)
}

#[inline]
pub fn index__<A: ReferenceOrArray, B: Number, C: Number>(data_source: A, row: B, column: C) -> FnAny3<A, B, C> {
    FnAny3("INDEX", data_source, row, column)
}

#[inline]
pub fn index___<A: ReferenceOrArray, B: Number, C: Number, D: Number>(data_source: A, row: B, column: C, area_number: D) -> FnAny4<A, B, C, D> {
    FnAny4("INDEX", data_source, row, column, area_number)
}

#[inline]
pub fn indirect<A: Text>(ref_: A) -> FnReference1<A> {
    FnReference1("INDIRECT", ref_)
}

#[inline]
pub fn indirect_<A: Text, B: Logical>(ref_: A, a1: B) -> FnReference2<A, B> {
    FnReference2("INDIRECT", ref_, a1)
}

#[inline]
pub fn lookup<A: Any, B: ReferenceOrArray>(find: A, searched: B) -> FnAny2<A, B> {
    FnAny2("LOOKUP", find, searched)
}

#[inline]
pub fn lookup_<A: Any, B: ReferenceOrArray, C: ReferenceOrArray>(find: A, searched: B, results: C) -> FnAny3<A, B, C> {
    FnAny3("LOOKUP", find, searched, results)
}

#[inline]
pub fn match_<A: Scalar, B: ReferenceOrArray>(search: A, search_region: B) -> FnAny2<A, B> {
    FnAny2("MATCH", search, search_region)
}

#[inline]
pub fn match__<A: Scalar, B: ReferenceOrArray>(search: A, search_region: B, match_type: MatchType) -> FnAny3<A, B, MatchType> {
    FnAny3("MATCH", search, search_region, match_type)
}

#[inline]
pub fn multiple_operations<A: Reference, B: Reference, C: Reference>(formula_cell: A, row_cell: B, row_replacement: C) -> FnAny3<A, B, C> {
    FnAny3("MULTIPLE.OPERATIONS", formula_cell, row_cell, row_replacement)
}

#[inline]
pub fn multiple_operations_<A: Reference, B: Reference, C: Reference, D: Reference>(formula_cell: A, row_cell: B, row_replacement: C, column_cell: D) -> FnAny4<A, B, C, D> {
    FnAny4("MULTIPLE.OPERATIONS", formula_cell, row_cell, row_replacement, column_cell)
}

#[inline]
pub fn multiple_operations__<A: Reference, B: Reference, C: Reference, D: Reference, E: Reference>(formula_cell: A, row_cell: B, row_replacement: C, column_cell: D, column_replacement: E) -> FnAny5<A, B, C, D, E> {
    FnAny5("MULTIPLE.OPERATIONS", formula_cell, row_cell, row_replacement, column_cell, column_replacement)
}

#[inline]
pub fn offset<A: Reference, B: Number, C: Number>(r: A, row_offset: B, column_offset: C) -> FnReference3<A, B, C> {
    FnReference3("OFFSET", r, row_offset, column_offset)
}

#[inline]
pub fn offset_<A: Reference, B: Number, C: Number, D: Number>(r: A, row_offset: B, column_offset: C, new_height: D) -> FnReference4<A, B, C, D> {
    FnReference4("OFFSET", r, row_offset, column_offset, new_height)
}

#[inline]
pub fn offset__<A: Reference, B: Number, C: Number, D: Number, E: Number>(r: A, row_offset: B, column_offset: C, new_height: D, new_width: E) -> FnReference5<A, B, C, D, E> {
    FnReference5("OFFSET", r, row_offset, column_offset, new_height, new_width)
}

#[inline]
pub fn vlookup<A: Any, B: ReferenceOrArray, C: Number>(lookup: A, data_source: B, column: C) -> FnAny3<A, B, C> {
    FnAny3("VLOOKUP", lookup, data_source, column)
}

#[inline]
pub fn vlookup_<A: Any, B: ReferenceOrArray, C: Number, D: Logical>(lookup: A, data_source: B, column: C, range_lookup: D) -> FnAny4<A, B, C, D> {
    FnAny4("VLOOKUP", lookup, data_source, column, range_lookup)
}
