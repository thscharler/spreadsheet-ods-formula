use crate::*;
#[allow(unused_imports)]
use crate::info::*;

#[inline]
pub fn areas<A: Reference>(r: A) -> FnNumber1<A> {
    FnNumber1("AREAS", r)
}

#[inline]
pub fn cell<>(info_type: CellInfo) -> FnAny1<CellInfo> {
    FnAny1("CELL", info_type)
}

#[inline]
pub fn cell_<A: Reference>(info_type: CellInfo, r: A) -> FnAny2<CellInfo, A> {
    FnAny2("CELL", info_type, r)
}

#[inline]
pub fn column() -> FnNumber0 {
    FnNumber0("COLUMN", )
}

#[inline]
pub fn column_<A: Reference>(r: A) -> FnNumber1<A> {
    FnNumber1("COLUMN", r)
}

#[inline]
pub fn columns<A: ReferenceOrArray>(r: A) -> FnNumber1<A> {
    FnNumber1("COLUMNS", r)
}

#[inline]
pub fn count<A: Sequence>(n: A) -> FnNumber1<A> {
    FnNumber1("COUNT", n)
}

#[inline]
pub fn counta<A: Sequence>(any_value: A) -> FnNumber1<A> {
    FnNumber1("COUNTA", any_value)
}

#[inline]
pub fn countblank<A: Reference>(r: A) -> FnNumber1<A> {
    FnNumber1("COUNTBLANK", r)
}

#[inline]
pub fn countif<A: Reference, B: Criterion>(r: A, c: B) -> FnNumber2<A, B> {
    FnNumber2("COUNTIF", r, c)
}

#[inline]
pub fn error_type<A: Any>(e: A) -> FnNumber1<A> {
    FnNumber1("ERROR.TYPE", e)
}

#[inline]
pub fn formula<A: Reference>(x: A) -> FnText1<A> {
    FnText1("FORMULA", x)
}

#[inline]
pub fn info<>(category: InfoInfo) -> FnAny1<InfoInfo> {
    FnAny1("INFO", category)
}

#[inline]
pub fn isblank<A: Scalar>(x: A) -> FnLogical1<A> {
    FnLogical1("ISBLANK", x)
}

#[inline]
pub fn iserr<A: Scalar>(x: A) -> FnLogical1<A> {
    FnLogical1("ISERR", x)
}

#[inline]
pub fn iserror<A: Scalar>(x: A) -> FnLogical1<A> {
    FnLogical1("ISERROR", x)
}

#[inline]
pub fn iseven<A: Number>(x: A) -> FnLogical1<A> {
    FnLogical1("ISEVEN", x)
}

#[inline]
pub fn isformula<A: Reference>(x: A) -> FnLogical1<A> {
    FnLogical1("ISFORMULA", x)
}

#[inline]
pub fn islogical<A: Scalar>(x: A) -> FnLogical1<A> {
    FnLogical1("ISLOGICAL", x)
}

#[inline]
pub fn isna<A: Scalar>(x: A) -> FnLogical1<A> {
    FnLogical1("ISNA", x)
}

#[inline]
pub fn isnontext<A: Scalar>(x: A) -> FnLogical1<A> {
    FnLogical1("ISNONTEXT", x)
}

#[inline]
pub fn isnumber<A: Scalar>(x: A) -> FnLogical1<A> {
    FnLogical1("ISNUMBER", x)
}

#[inline]
pub fn isodd<A: Number>(x: A) -> FnLogical1<A> {
    FnLogical1("ISODD", x)
}

#[inline]
pub fn isref<A: Any>(x: A) -> FnLogical1<A> {
    FnLogical1("ISREF", x)
}

#[inline]
pub fn istext<A: Scalar>(x: A) -> FnLogical1<A> {
    FnLogical1("ISTEXT", x)
}

#[inline]
pub fn n<A: Any>(x: A) -> FnNumber1<A> {
    FnNumber1("N", x)
}

#[inline]
pub fn na() -> FnAny0 {
    FnAny0("NA", )
}

#[inline]
pub fn numbervalue<A: Text>(x: A) -> FnNumber1<A> {
    FnNumber1("NUMBERVALUE", x)
}

#[inline]
pub fn numbervalue_<A: Text, B: Text>(x: A, decimal_separator: B) -> FnNumber2<A, B> {
    FnNumber2("NUMBERVALUE", x, decimal_separator)
}

#[inline]
pub fn numbervalue__<A: Text, B: Text, C: Text>(x: A, decimal_separator: B, group_separator: C) -> FnNumber3<A, B, C> {
    FnNumber3("NUMBERVALUE", x, decimal_separator, group_separator)
}

#[inline]
pub fn row() -> FnNumber0 {
    FnNumber0("ROW", )
}

#[inline]
pub fn row_<A: Reference>(r: A) -> FnNumber1<A> {
    FnNumber1("ROW", r)
}

#[inline]
pub fn rows<A: ReferenceOrArray>(r: A) -> FnNumber1<A> {
    FnNumber1("ROWS", r)
}

#[inline]
pub fn sheet() -> FnNumber0 {
    FnNumber0("SHEET", )
}

#[inline]
pub fn sheet_<A: TextOrReference>(r: A) -> FnNumber1<A> {
    FnNumber1("SHEET", r)
}

#[inline]
pub fn sheets() -> FnNumber0 {
    FnNumber0("SHEETS", )
}

#[inline]
pub fn sheets_<A: Reference>(r: A) -> FnNumber1<A> {
    FnNumber1("SHEETS", r)
}

#[inline]
pub fn type_<A: Any>(value: A) -> FnNumber1<A> {
    FnNumber1("TYPE", value)
}

#[inline]
pub fn value<A: Text>(x: A) -> FnNumber1<A> {
    FnNumber1("VALUE", x)
}
