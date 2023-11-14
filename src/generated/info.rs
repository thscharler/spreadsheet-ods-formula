use crate::*;
#[allow(unused_imports)]
use crate::info::*;

/// Returns the number of areas in a given list of references.
#[inline]
pub fn areas<A: Reference>(refs: A) -> FnNumber1<A> {
    FnNumber1("AREAS", refs)
}

///  Returns information about position, formatting or contents in a reference.
#[inline]
pub fn cell_<A: Reference>(info_type: CellInfo, refs: A) -> FnAny2<CellInfo, A> {
    FnAny2("CELL", info_type, refs)
}

///  Returns information about position, formatting or contents in a reference.
#[inline]
pub fn cell<>(info_type: CellInfo) -> FnAny1<CellInfo> {
    FnAny1("CELL", info_type)
}

/// Returns the column number(s) of a reference
#[inline]
pub fn column_<A: Reference>(refs: A) -> FnNumber1<A> {
    FnNumber1("COLUMN", refs)
}

/// Returns the column number(s) of a reference
#[inline]
pub fn column() -> FnNumber0 {
    FnNumber0("COLUMN", )
}

/// Returns the number of columns in a given range.
#[inline]
pub fn columns<A: Reference>(refs: A) -> FnNumber1<A> {
    FnNumber1("COLUMNS", refs)
}

/// Count the number of Numbers provided
#[inline]
pub fn count<A: Sequence>(seq: A) -> FnNumber1<A> {
    FnNumber1("COUNT", seq)
}

///  Count the number of non-empty values.
#[inline]
pub fn counta<A: Sequence>(seq: A) -> FnNumber1<A> {
    FnNumber1("COUNTA", seq)
}

/// Count the number of blank cells
#[inline]
pub fn countblank<A: Sequence>(seq: A) -> FnNumber1<A> {
    FnNumber1("COUNTBLANK", seq)
}

/// Count the number of cells in a range that meet a criteria.
#[inline]
pub fn countif<A: Sequence, B: Criterion>(seq: A, crit: B) -> FnNumber2<A, B> {
    FnNumber2("COUNTIF", seq, crit)
}

/// Returns Number representing the specific Error type
#[inline]
pub fn error_type<A: Any>(error: A) -> FnNumber1<A> {
    FnNumber1("ERROR_TYPE", error)
}

/// Returns information about the environment
#[inline]
pub fn info<>(category: InfoInfo) -> FnAny1<InfoInfo> {
    FnAny1("INFO", category)
}

/// Return TRUE if the referenced cell is blank, else return FALSE
#[inline]
pub fn isblank<A: Any>(x: A) -> FnLogical1<A> {
    FnLogical1("ISBLANK", x)
}

/// Return TRUE if the parameter has type Error and is not #N/A, else return FALSE.
#[inline]
pub fn iserr<A: Any>(x: A) -> FnLogical1<A> {
    FnLogical1("ISERR", x)
}

/// Return TRUE if the parameter has type Error, else return FALSE.
#[inline]
pub fn iserror<A: Any>(x: A) -> FnLogical1<A> {
    FnLogical1("ISERROR", x)
}

/// Return TRUE if the value is even, else return FALSE
#[inline]
pub fn iseven<A: Number>(n: A) -> FnLogical1<A> {
    FnLogical1("ISEVEN", n)
}

/// Return TRUE if the reference refers to a formula, else return FALSE.
#[inline]
pub fn isformula<A: Reference>(refs: A) -> FnLogical1<A> {
    FnLogical1("ISFORMULA", refs)
}

/// Return TRUE if the parameter has type Logical, else return FALSE
#[inline]
pub fn islogical<A: Any>(x: A) -> FnLogical1<A> {
    FnLogical1("ISLOGICAL", x)
}

/// Return TRUE if the parameter has type Error and is #N/A, else return FALSE.
#[inline]
pub fn isna<A: Any>(x: A) -> FnLogical1<A> {
    FnLogical1("ISNA", x)
}

/// Return TRUE if the parameter does not have type Text, else return FALSE.
#[inline]
pub fn isnontext<A: Any>(x: A) -> FnLogical1<A> {
    FnLogical1("ISNONTEXT", x)
}

/// Return TRUE if the parameter has type Number, else return FALSE
#[inline]
pub fn isnumber<A: Any>(x: A) -> FnLogical1<A> {
    FnLogical1("ISNUMBER", x)
}

/// Return TRUE if the value is even, else return FALSE.
#[inline]
pub fn isodd<A: Any>(x: A) -> FnLogical1<A> {
    FnLogical1("ISODD", x)
}

/// Return TRUE if the parameter is of type reference, else return FALSE.
#[inline]
pub fn isref<A: Any>(x: A) -> FnLogical1<A> {
    FnLogical1("ISREF", x)
}

/// Return TRUE if the parameter has type Text, else return FALSE.
#[inline]
pub fn istext<A: Any>(x: A) -> FnLogical1<A> {
    FnLogical1("ISTEXT", x)
}

/// Return the number of a value.
#[inline]
pub fn n<A: Any>(x: A) -> FnLogical1<A> {
    FnLogical1("N", x)
}

/// Return the constant Error value #N/A.
#[inline]
pub fn na() -> FnLogical0 {
    FnLogical0("NA", )
}

/// Convert text to number, in a locale-independent way.
#[inline]
pub fn numbervalue__<A: Text, B: Text, C: Text>(text: A, sep: B, grp: C) -> FnNumber3<A, B, C> {
    FnNumber3("NUMBERVALUE", text, sep, grp)
}

/// Convert text to number, in a locale-independent way.
#[inline]
pub fn numbervalue_<A: Text, B: Text>(text: A, sep: B) -> FnNumber2<A, B> {
    FnNumber2("NUMBERVALUE", text, sep)
}

/// Convert text to number, in a locale-independent way.
#[inline]
pub fn numbervalue<A: Text>(text: A) -> FnNumber1<A> {
    FnNumber1("NUMBERVALUE", text)
}

/// Returns the row number(s) of a reference.
#[inline]
pub fn row_<A: Reference>(refs: A) -> FnNumber1<A> {
    FnNumber1("ROW", refs)
}

/// Returns the row number(s) of a reference.
#[inline]
pub fn row() -> FnNumber0 {
    FnNumber0("ROW", )
}

/// Returns the number of rows in a given range.
#[inline]
pub fn rows<A: Reference>(refs: A) -> FnNumber1<A> {
    FnNumber1("ROWS", refs)
}

/// Returns the sheet number of the reference or the string representing a sheet name.
#[inline]
pub fn sheet_<A: Reference>(refs: A) -> FnNumber1<A> {
    FnNumber1("SHEET", refs)
}

/// Returns the sheet number of the reference or the string representing a sheet name.
#[inline]
pub fn sheet() -> FnNumber0 {
    FnNumber0("SHEET", )
}

/// Returns the number of sheets in a reference or current document.
#[inline]
pub fn sheets_<A: Reference>(refs: A) -> FnNumber1<A> {
    FnNumber1("SHEETS", refs)
}

/// Returns the number of sheets in a reference or current document.
#[inline]
pub fn sheets() -> FnNumber0 {
    FnNumber0("SHEETS", )
}

/// Returns a number indicating the type of the provided value.
#[inline]
pub fn type_<A: Any>(a: A) -> FnNumber1<A> {
    FnNumber1("TYPE", a)
}

/// Convert text to number
#[inline]
pub fn value<A: Text>(text: A) -> FnNumber1<A> {
    FnNumber1("VALUE", text)
}
