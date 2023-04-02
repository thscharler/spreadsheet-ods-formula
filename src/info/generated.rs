use crate::*;
#[allow(unused_imports)]
use super::*;

/// Returns the number of areas in a given list of references.
#[inline]
pub fn areas<A: Reference>(refs: A) -> FnNumber1<A> {
    FnNumber1("AREAS", refs)
}

/// Returns information about position, formatting or contents in a reference.
#[inline]
pub fn cell_col<A: Reference>(refs: A) -> FnNumber1<A> {
    FnNumber1("CELL_COL", refs)
}

/// Returns information about position, formatting or contents in a reference.
#[inline]
pub fn cell_row<A: Reference>(refs: A) -> FnNumber1<A> {
    FnNumber1("CELL_ROW", refs)
}

/// Returns information about position, formatting or contents in a reference.
#[inline]
pub fn cell_sheet<A: Reference>(refs: A) -> FnNumber1<A> {
    FnNumber1("CELL_SHEET", refs)
}

/// Returns information about position, formatting or contents in a reference.
#[inline]
pub fn cell_address<A: Reference>(refs: A) -> FnReference1<A> {
    FnReference1("CELL_ADDRESS", refs)
}

/// Returns information about position, formatting or contents in a reference.
#[inline]
pub fn cell_filename<A: Reference>(refs: A) -> FnText1<A> {
    FnText1("CELL_FILENAME", refs)
}

/// Returns information about position, formatting or contents in a reference.
#[inline]
pub fn cell_contents<A: Reference>(refs: A) -> FnText1<A> {
    FnText1("CELL_CONTENTS", refs)
}

/// Returns information about position, formatting or contents in a reference.
#[inline]
pub fn cell_colored<A: Reference>(refs: A) -> FnLogical1<A> {
    FnLogical1("CELL_COLORED", refs)
}

/// Returns information about position, formatting or contents in a reference.
#[inline]
pub fn cell_format<A: Reference>(refs: A) -> FnText1<A> {
    FnText1("CELL_FORMAT", refs)
}

/// Returns information about position, formatting or contents in a reference.
#[inline]
pub fn cell_type<A: Reference>(refs: A) -> FnText1<A> {
    FnText1("CELL_TYPE", refs)
}

/// Returns information about position, formatting or contents in a reference.
#[inline]
pub fn cell_width<A: Reference>(refs: A) -> FnNumber1<A> {
    FnNumber1("CELL_WIDTH", refs)
}

/// Returns information about position, formatting or contents in a reference.
#[inline]
pub fn cell_protect<A: Reference>(refs: A) -> FnLogical1<A> {
    FnLogical1("CELL_PROTECT", refs)
}

/// Returns information about position, formatting or contents in a reference.
#[inline]
pub fn cell_parentheses<A: Reference>(refs: A) -> FnLogical1<A> {
    FnLogical1("CELL_PARENTHESES", refs)
}

/// Returns information about position, formatting or contents in a reference.
#[inline]
pub fn cell_prefix<A: Reference>(refs: A) -> FnText1<A> {
    FnText1("CELL_PREFIX", refs)
}

/// Returns the column number(s) of a reference
#[inline]
pub fn column<A: Reference>(refs: A) -> FnNumber1<A> {
    FnNumber1("COLUMN", refs)
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

///  Returns formula at given reference as text.
#[inline]
pub fn cell_formula<A: Reference>(refs: A) -> FnText1<A> {
    FnText1("CELL_FORMULA", refs)
}

/// Returns information about the environment
#[inline]
pub fn info_directory() -> FnText0 {
    FnText0("INFO_DIRECTORY", )
}

/// Returns information about the environment
#[inline]
pub fn info_memavail() -> FnNumber0 {
    FnNumber0("INFO_MEMAVAIL", )
}

/// Returns information about the environment
#[inline]
pub fn info_memused() -> FnNumber0 {
    FnNumber0("INFO_MEMUSED", )
}

/// Returns information about the environment
#[inline]
pub fn info_numfile() -> FnNumber0 {
    FnNumber0("INFO_NUMFILE", )
}

/// Returns information about the environment
#[inline]
pub fn info_osversion() -> FnText0 {
    FnText0("INFO_OSVERSION", )
}

/// Returns information about the environment
#[inline]
pub fn info_origin() -> FnText0 {
    FnText0("INFO_ORIGIN", )
}

/// Returns information about the environment
#[inline]
pub fn info_recal() -> FnText0 {
    FnText0("INFO_RECAL", )
}

/// Returns information about the environment
#[inline]
pub fn info_release() -> FnText0 {
    FnText0("INFO_RELEASE", )
}

/// Returns information about the environment
#[inline]
pub fn info_system() -> FnText0 {
    FnText0("INFO_SYSTEM", )
}

/// Returns information about the environment
#[inline]
pub fn info_totmem() -> FnNumber0 {
    FnNumber0("INFO_TOTMEM", )
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
pub fn numbervalue<A: Text, B: Text, C: Text>(text: A, sep: Option<B>, grp: Option<C>) -> FnNumber3<A, Option<B>, Option<C>> {
    FnNumber3("NUMBERVALUE", text, sep, grp)
}

/// Returns the row number(s) of a reference.
#[inline]
pub fn row<A: Reference>(refs: A) -> FnNumber1<A> {
    FnNumber1("ROW", refs)
}

/// Returns the number of rows in a given range.
#[inline]
pub fn rows<A: Reference>(refs: A) -> FnNumber1<A> {
    FnNumber1("ROWS", refs)
}

/// Returns the sheet number of the reference or the string representing a sheet name.
#[inline]
pub fn sheet<A: Reference>(refs: A) -> FnNumber1<A> {
    FnNumber1("SHEET", refs)
}

/// Returns the number of sheets in a reference or current document.
#[inline]
pub fn sheets<A: Reference>(refs: A) -> FnNumber1<A> {
    FnNumber1("SHEETS", refs)
}

/// Returns a number indicating the type of the provided value.
#[inline]
pub fn value_type<A: Any>(a: A) -> FnNumber1<A> {
    FnNumber1("VALUE_TYPE", a)
}

/// Convert text to number
#[inline]
pub fn value<A: Text>(text: A) -> FnNumber1<A> {
    FnNumber1("VALUE", text)
}
