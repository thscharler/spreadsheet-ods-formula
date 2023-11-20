//! 
//! Information functions provide information about a data value, the 
//! spreadsheet, or underlying environment, including special functions for 
//! converting between data types.

use crate::*;
#[allow(unused_imports)]
use crate::info::*;

/// Returns the number of areas in a given list of references.
/// Syntax: AREAS( R ReferenceList; )
///
/// Constraints:
/// None
///
/// Semantics:
/// Returns the number of areas in the reference list R.
///
/// See also: "Infix Operator Reference Concatenation", "INDEX", 
#[inline]
pub fn areas<A: Reference>(r: A) -> FnNumber1<A> {
    FnNumber1("AREAS", r)
}

/// Returns information about position, formatting or contents in a reference.
/// Syntax: CELL( Info_Type Text; )
///
/// Constraints:
/// None
///
/// Semantics:
/// The parameters are
/// 
/// •Info_Type: the text string which specifies the type of information. 
/// Please refer to Table 17 - CELL.
/// 
/// •R : if R is a reference to a cell, it is the cell whose information will 
/// be returned; if R is a reference to a range, the top-left cell in the range 
/// is the selected one; if R is omitted, the current cell is used.
/// 
/// Table 17 - CELL
/// 
/// Info_Type
/// 
/// Comment
/// 
/// COL
/// 
/// Returns the column number of the cell.
/// 
/// ROW
/// 
/// Returns the row number of the cell.
/// 
/// SHEET
/// 
/// Returns the sheet number of the cell.
/// 
/// ADDRESS
/// 
/// Returns the absolute address of the cell. The sheet name is included if 
/// given in the reference and does not reference the same sheet as the sheet 
/// the expression is evaluated upon. For an external reference a Source as 
/// specified in the syntax rules for References 5.8 is included.
/// 
/// FILENAME
/// 
/// Returns the file name of the file that contains the cell as an IRI. If the 
/// file is newly created and has not yet been saved, the file name is empty 
/// text “”.
/// 
/// CONTENTS
/// 
/// Returns the contents of the cell, without formatting properties.
/// 
/// COLOR
/// 
/// Returns 1 if color formatting is set for negative value in this cell; 
/// otherwise returns 0
/// 
/// FORMAT
/// 
/// Returns a text string which shows the number format of the cell.
/// 
/// ,(comma) = number with thousands separator
/// 
/// F = number without thousands separator
/// 
/// C = currency format
/// 
/// S = exponential representation
/// 
/// P = percentage
/// 
/// To indicate the number of decimal places after the decimal separator, a 
/// number is given right after the above characters.
/// 
/// D1 = MMM-D-YY, MM-D-YY and similar formats
/// 
/// D2 = DD-MM
/// 
/// D3 = MM-YY
/// 
/// D4 = DD-MM-YYYY HH:MM:SS
/// 
/// D5 = MM-DD
/// 
/// D6 = HH:MM:SS AM/PM
/// 
/// D7 = HH:MM AM/PM
/// 
/// D8 = HH:MM:SS
/// 
/// D9 = HH:MM
/// 
/// G = All other formats
/// 
/// - (Minus) at the end = negative numbers in the cell have color setting
/// 
/// () (brackets) at the end = this cell has the format settings with 
/// parentheses for positive or all values
/// 
/// TYPE
/// 
/// Returns the text value corresponding to the type of content in the cell:
/// 
/// “b” : blank or empty cell content
/// 
/// “l” : label or text cell content
/// 
/// “v” : number value cell content
/// 
/// WIDTH
/// 
/// Returns the column width of the cell.
/// 
/// The unit is the width of one zero (0) character in default font size.
/// 
/// PROTECT
/// 
/// Returns the protection status of the cell:
/// 
/// 1 = cell is protected
/// 
/// 0 = cell is unprotected
/// 
/// PARENTHESES
/// 
/// Returns 1 if the cell has the format settings with parentheses for positive 
/// or all values, otherwise returns 0
/// 
/// PREFIX
/// 
/// Returns single character text strings corresponding to the alignment of the 
/// cell.
/// 
/// “'” (APOSTROPHE, U+0027) = left alignment
/// 
/// '"' (QUOTATION MARK, U+0022) = right alignment
/// 
/// “^” (CIRCUMFLEX ACCENT, U+005E) = centered alignment
/// 
/// “\” (REVERSE SOLIDUS, U+005C) = filled alignment
/// 
/// otherwise, returns empty string "".
/// 
/// •
#[inline]
pub fn cell<>(info_type: CellInfo) -> FnAny1<CellInfo> {
    FnAny1("CELL", info_type)
}

/// Returns information about position, formatting or contents in a reference.
/// Syntax: CELL( Info_Type Text;[; R Reference] )
///
/// Constraints:
/// None
///
/// Semantics:
/// The parameters are
/// 
/// •Info_Type: the text string which specifies the type of information. 
/// Please refer to Table 17 - CELL.
/// 
/// •R : if R is a reference to a cell, it is the cell whose information will 
/// be returned; if R is a reference to a range, the top-left cell in the range 
/// is the selected one; if R is omitted, the current cell is used.
/// 
/// Table 17 - CELL
/// 
/// Info_Type
/// 
/// Comment
/// 
/// COL
/// 
/// Returns the column number of the cell.
/// 
/// ROW
/// 
/// Returns the row number of the cell.
/// 
/// SHEET
/// 
/// Returns the sheet number of the cell.
/// 
/// ADDRESS
/// 
/// Returns the absolute address of the cell. The sheet name is included if 
/// given in the reference and does not reference the same sheet as the sheet 
/// the expression is evaluated upon. For an external reference a Source as 
/// specified in the syntax rules for References 5.8 is included.
/// 
/// FILENAME
/// 
/// Returns the file name of the file that contains the cell as an IRI. If the 
/// file is newly created and has not yet been saved, the file name is empty 
/// text “”.
/// 
/// CONTENTS
/// 
/// Returns the contents of the cell, without formatting properties.
/// 
/// COLOR
/// 
/// Returns 1 if color formatting is set for negative value in this cell; 
/// otherwise returns 0
/// 
/// FORMAT
/// 
/// Returns a text string which shows the number format of the cell.
/// 
/// ,(comma) = number with thousands separator
/// 
/// F = number without thousands separator
/// 
/// C = currency format
/// 
/// S = exponential representation
/// 
/// P = percentage
/// 
/// To indicate the number of decimal places after the decimal separator, a 
/// number is given right after the above characters.
/// 
/// D1 = MMM-D-YY, MM-D-YY and similar formats
/// 
/// D2 = DD-MM
/// 
/// D3 = MM-YY
/// 
/// D4 = DD-MM-YYYY HH:MM:SS
/// 
/// D5 = MM-DD
/// 
/// D6 = HH:MM:SS AM/PM
/// 
/// D7 = HH:MM AM/PM
/// 
/// D8 = HH:MM:SS
/// 
/// D9 = HH:MM
/// 
/// G = All other formats
/// 
/// - (Minus) at the end = negative numbers in the cell have color setting
/// 
/// () (brackets) at the end = this cell has the format settings with 
/// parentheses for positive or all values
/// 
/// TYPE
/// 
/// Returns the text value corresponding to the type of content in the cell:
/// 
/// “b” : blank or empty cell content
/// 
/// “l” : label or text cell content
/// 
/// “v” : number value cell content
/// 
/// WIDTH
/// 
/// Returns the column width of the cell.
/// 
/// The unit is the width of one zero (0) character in default font size.
/// 
/// PROTECT
/// 
/// Returns the protection status of the cell:
/// 
/// 1 = cell is protected
/// 
/// 0 = cell is unprotected
/// 
/// PARENTHESES
/// 
/// Returns 1 if the cell has the format settings with parentheses for positive 
/// or all values, otherwise returns 0
/// 
/// PREFIX
/// 
/// Returns single character text strings corresponding to the alignment of the 
/// cell.
/// 
/// “'” (APOSTROPHE, U+0027) = left alignment
/// 
/// '"' (QUOTATION MARK, U+0022) = right alignment
/// 
/// “^” (CIRCUMFLEX ACCENT, U+005E) = centered alignment
/// 
/// “\” (REVERSE SOLIDUS, U+005C) = filled alignment
/// 
/// otherwise, returns empty string "".
/// 
/// •
#[inline]
pub fn cell_<A: Reference>(info_type: CellInfo, r: A) -> FnAny2<CellInfo, A> {
    FnAny2("CELL", info_type, r)
}

/// Returns the column number(s) of a reference.
/// Syntax: COLUMN( )
///
/// Constraints:
/// AREAS(R) = 1
///
/// Semantics:
/// Returns the column number of a reference, where “A” is 1, “B” is 2, 
/// and so on. If no parameter is given, the current cell is used. If a 
/// reference has multiple columns, an array of numbers is returned with all of 
/// the columns in the reference.
///
/// See also: "AREAS", "ROW", "SHEET", 
#[inline]
pub fn column() -> FnNumber0 {
    FnNumber0("COLUMN", )
}

/// Returns the column number(s) of a reference.
/// Syntax: COLUMN([ R Reference] )
///
/// Constraints:
/// AREAS(R) = 1
///
/// Semantics:
/// Returns the column number of a reference, where “A” is 1, “B” is 2, 
/// and so on. If no parameter is given, the current cell is used. If a 
/// reference has multiple columns, an array of numbers is returned with all of 
/// the columns in the reference.
///
/// See also: "AREAS", "ROW", "SHEET", 
#[inline]
pub fn column_<A: Reference>(r: A) -> FnNumber1<A> {
    FnNumber1("COLUMN", r)
}

/// Returns the number of columns in a given range.
/// Syntax: COLUMNS( R Reference|Array; )
///
/// Constraints:
/// None
///
/// Semantics:
/// Returns the number of columns in the range or array specified. The result 
/// is not dependent on the cell content in the range.
///
/// See also: "ROWS", 
#[inline]
pub fn columns<A: ReferenceOrArray>(r: A) -> FnNumber1<A> {
    FnNumber1("COLUMNS", r)
}

/// Count the number of Numbers provided.
/// Syntax: COUNT({ N NumberSequenceList}+ )
///
/// Constraints:
/// One or more parameters.
///
/// Semantics:
/// Counts the numbers in the list N. Only numbers in references are counted; 
/// all other types are ignored. Errors are not propagated. It is 
/// implementation-defined what happens if 0 parameters are passed, but it 
/// should be an Error or 0.
///
/// See also: "COUNTA", 
#[inline]
pub fn count<A: Sequence>(n: A) -> FnNumber1<A> {
    FnNumber1("COUNT", n)
}

/// Count the number of non-empty values.
/// Syntax: COUNTA({ AnyValue Any}+ )
///
/// Constraints:
/// None.
///
/// Semantics:
/// Counts the number of non-blank values. A value is non-blank if it contains 
/// any content of any type, including an Error. In a reference, every cell 
/// that is not empty is included in the count. An empty string value ("") is 
/// not considered blank. Errors contained in a range are considered a 
/// non-blank value for purposes of the count; errors do not propagate. 
/// Constant expressions or formulas are allowed; these are evaluated and if 
/// they produce an Error value the Error value is counted as one non-blank 
/// value (and not propagated as an Error). It is implementation-defined what 
/// happens if 0 parameters are passed, but it should be an Error or 0.
///
/// See also: "COUNT", "ISBLANK", 
#[inline]
pub fn counta<A: Sequence>(any_value: A) -> FnNumber1<A> {
    FnNumber1("COUNTA", any_value)
}

/// Count the number of blank cells.
/// Syntax: COUNTBLANK( R ReferenceList; )
///
/// Constraints:
/// None.
///
/// Semantics:
/// Counts the number of blank cells in R. A cell is blank if the cell is empty 
/// for purposes of COUNTBLANK. If ISBLANK(R) is TRUE, then it is blank. A cell 
/// with numeric value zero ('0') is not blank. It is implementation-defined 
/// whether or not a cell returning the empty string ("") is considered blank; 
/// because of this, there is a (potential) subtle difference between 
/// COUNTBLANK and ISBLANK.
/// 
/// Evaluators shall support one Reference as a parameter and may support a 
/// ReferenceList as a parameter.
///
/// See also: "COUNT", "COUNTA", "COUNTIF", "ISBLANK", 
#[inline]
pub fn countblank<A: Reference>(r: A) -> FnNumber1<A> {
    FnNumber1("COUNTBLANK", r)
}

/// Count the number of cells in a range that meet a criteria.
/// Syntax: COUNTIF( R ReferenceList;; C Criterion; )
///
/// Constraints:
/// Does not accept constant values as the reference parameter.
///
/// Semantics:
/// Counts the number of cells in the reference range R that meet the Criterion 
/// C (4.11.8).
/// 
/// The values returned may vary depending upon the 
/// HOST-USE-REGULAR-EXPRESSIONS or HOST-USE-WILDCARDS or 
/// HOST-SEARCH-CRITERIA-MUST-APPLY-TO-WHOLE-CELL properties. 3.4
///
/// See also: "COUNT", "COUNTA", "COUNTBLANK", "COUNTIFS", "SUMIF", "Infix Operator \"=\"", "Infix Operator \"<>\"", "Infix Operator Ordered Comparison (\"<\", \"<=\", \">\", \">=\")", 
#[inline]
pub fn countif<A: Reference, B: Criterion>(r: A, c: B) -> FnNumber2<A, B> {
    FnNumber2("COUNTIF", r, c)
}

/// Returns Number representing the specific Error type.
/// Syntax: ERROR.TYPE( E Error; )
///
/// Constraints:
/// None.
///
/// Semantics:
/// Returns a number representing what kind of Error has occurred. Note that 
/// unlike most functions, this function does not propagate Error values. 
/// Receiving a non-Error value returns an Error. In particular, 
/// ERROR.TYPE(NA()) returns 7, and ERROR.TYPE applied to a non-Error returns 
/// an Error.
///
/// See also: "NA", 
#[inline]
pub fn error_type<A: Any>(e: A) -> FnNumber1<A> {
    FnNumber1("ERROR.TYPE", e)
}

/// Returns formula at given reference as text.
/// Syntax: FORMULA( X Reference; )
///
/// Constraints:
/// Reference X shall contain a formula.
///
/// Semantics:
/// Returns the formula in reference X as a string. The specific syntax of this 
/// returned string is implementation-defined. This function is intended to aid 
/// debugging by simplifying display of formulas in other cells. Error results 
/// of the referred formula cell are not propagated.
///
/// See also: "ISFORMULA", 
#[inline]
pub fn formula<A: Reference>(x: A) -> FnText1<A> {
    FnText1("FORMULA", x)
}

/// Returns information about the environment.
/// Syntax: INFO( Category Text; )
///
/// Constraints:
/// Category shall be valid.
///
/// Semantics:
/// Returns information about the environment in the given category.
/// 
/// Evaluators shall support at least the following categories:
/// 
/// Table 18 - INFO
/// 
/// Category
/// 
/// Meaning
/// 
/// Type
/// 
/// "directory"
/// 
/// Current directory. This shall be formatted so file names can be appended to 
/// the result (e.g., on POSIX and Windows systems it shall end with the 
/// separator “/” or “\” respectively).
/// 
/// Text
/// 
/// "memavail"
/// 
/// Amount of memory “available”, in bytes. On many modern (virtual memory) 
/// systems this value is not really available, but a system should return 0 if 
/// it is known that there is no more memory available, and greater than 0 
/// otherwise
/// 
/// Number
/// 
/// "memused"
/// 
/// Amount of memory used, in bytes, by the data
/// 
/// Number
/// 
/// "numfile"
/// 
/// Number of active worksheets in files
/// 
/// Number
/// 
/// "osversion"
/// 
/// Operating system version
/// 
/// Text
/// 
/// "origin"
/// 
/// The top leftmost visible cell's absolute reference prefixed with “$A:”. 
/// In locales where cells are ordered right-to-left, the top rightmost visible 
/// cell is used instead.
/// 
/// Text
/// 
/// "recalc"
/// 
/// Current recalculation mode. If the locale is English, this is either 
/// "Automatic" or "Manual" (the exact text depends on the locale)
/// 
/// Text
/// 
/// "release"
/// 
/// The version of the implementation.
/// 
/// Text
/// 
/// "system"
/// 
/// The type of the operating system.
/// 
/// Text
/// 
/// "totmem"
/// 
/// Total memory available in bytes, including the memory already used.
/// 
/// Number
/// 
/// Evaluators may support other categories.
///
/// See also: "CELL", 
#[inline]
pub fn info<>(category: InfoInfo) -> FnAny1<InfoInfo> {
    FnAny1("INFO", category)
}

/// Return TRUE if the referenced cell is blank, else return FALSE.
/// Syntax: ISBLANK( X Scalar; )
///
/// Constraints:
/// None
///
/// Semantics:
/// If X is of type Number, Text, or Logical, return FALSE. If X is a reference 
/// to a cell, examine the cell; if it is blank (has no value), return TRUE, 
/// but if it has a value, return FALSE. A cell with the empty string is not 
/// considered blank. This function does not propagate Error values.
///
/// See also: "ISNUMBER", "ISTEXT", 
#[inline]
pub fn isblank<A: Scalar>(x: A) -> FnLogical1<A> {
    FnLogical1("ISBLANK", x)
}

/// Return TRUE if the parameter has type Error and is not #N/A, else return 
/// FALSE.
/// Syntax: ISERR( X Scalar; )
///
/// Constraints:
/// None
///
/// Semantics:
/// If X is of type Error, and ISNA(X) is not true, returns TRUE. Otherwise it 
/// returns FALSE. Note that this function returns FALSE if given #N/A; if this 
/// is not desired, use ISERROR 6.13.16. Note that this function does not 
/// propagate Error values.
/// 
/// ISERR(X) is the same as:
/// 
/// IF(ISNA(X),FALSE(),ISERROR(X))
///
/// See also: "ERROR.TYPE", "ISERROR", "ISNA", "ISNUMBER", "ISTEXT", "NA", 
#[inline]
pub fn iserr<A: Scalar>(x: A) -> FnLogical1<A> {
    FnLogical1("ISERR", x)
}

/// Return TRUE if the parameter has type Error, else return FALSE.
/// Syntax: ISERROR( X Scalar; )
///
/// Constraints:
/// None
///
/// Semantics:
/// If X is of type Error, returns TRUE, else returns FALSE. Note that this 
/// function returns TRUE if given #N/A; if this is not desired, use ISERR 
/// 6.13.15. Note that this function does not propagate Error values.
///
/// See also: "ERROR.TYPE", "ISERR", "ISNA", "ISNUMBER", "ISTEXT", "NA", 
#[inline]
pub fn iserror<A: Scalar>(x: A) -> FnLogical1<A> {
    FnLogical1("ISERROR", x)
}

/// Return TRUE if the value is even, else return FALSE.
/// Syntax: ISEVEN( X Number; )
///
/// Constraints:
/// None
///
/// Semantics:
/// First, compute X1 = TRUNC(X). Then, if X1 is even (a division by 2 has a 
/// remainder of 0), return TRUE, else return FALSE. The result is 
/// implementation-defined if given a Logical value; an evaluator may return 
/// either an Error or the result of converting the Logical value to a Number 
/// (per Conversion to Number 6.3.5 ).
///
/// See also: "ISODD", "TRUNC", 
#[inline]
pub fn iseven<A: Number>(x: A) -> FnLogical1<A> {
    FnLogical1("ISEVEN", x)
}

/// Return TRUE if the reference refers to a formula, else return FALSE.
/// Syntax: ISFORMULA( X Reference; )
///
/// Constraints:
/// None
///
/// Semantics:
/// If X refers to a cell whose value is computed by a formula, return TRUE, 
/// else return FALSE. A formula itself may compute a constant; in that case it 
/// will still return TRUE since it is still a formula. Passing a 
/// non-reference, or a reference to more than one cell, is 
/// implementation-defined. This function does not propagate Error values.
///
/// See also: "ISTEXT", "ISNUMBER", 
#[inline]
pub fn isformula<A: Reference>(x: A) -> FnLogical1<A> {
    FnLogical1("ISFORMULA", x)
}

/// Return TRUE if the parameter has type Logical, else return FALSE.
/// Syntax: ISLOGICAL( X Scalar; )
///
/// Constraints:
/// None
///
/// Semantics:
/// If X is of type Logical, returns TRUE, else FALSE. Evaluators that do not 
/// have a distinct Logical type will return the same value ISNUMBER(X) would 
/// return. This function does not propagate Error values.
///
/// See also: "ISTEXT", "ISNUMBER", 
#[inline]
pub fn islogical<A: Scalar>(x: A) -> FnLogical1<A> {
    FnLogical1("ISLOGICAL", x)
}

/// Return TRUE if the parameter has type Error and is #N/A, else return FALSE.
/// Syntax: ISNA( X Scalar; )
///
/// Constraints:
/// None
///
/// Semantics:
/// If X is #N/A, return TRUE, else return FALSE. Note that if X is a 
/// reference, the value being referenced is considered. This function does not 
/// propagate Error values.
///
/// See also: "ERROR.TYPE", "ISERROR", "ISERR", "ISNUMBER", "ISTEXT", "NA", 
#[inline]
pub fn isna<A: Scalar>(x: A) -> FnLogical1<A> {
    FnLogical1("ISNA", x)
}

/// Return TRUE if the parameter does not have type Text, else return FALSE.
/// Syntax: ISNONTEXT( X Scalar; )
///
/// Constraints:
/// None
///
/// Semantics:
/// If X is of type Text, ISNONTEXT returns FALSE, else TRUE. If X is a 
/// reference, it examines what X references. References to empty cells are not 
/// considered text, so for reference to an empty cell ISNONTEXT will return 
/// TRUE. Empty Cell 4.7 This function does not propagate Error values.
/// 
/// ISNONTEXT(X) is equivalent to NOT(ISTEXT(X))
///
/// See also: "ISNUMBER", "ISLOGICAL", "ISTEXT", "NOT", 
#[inline]
pub fn isnontext<A: Scalar>(x: A) -> FnLogical1<A> {
    FnLogical1("ISNONTEXT", x)
}

/// Return TRUE if the parameter has type Number, else return FALSE.
/// Syntax: ISNUMBER( X Scalar; )
///
/// Constraints:
/// None
///
/// Semantics:
/// If X is of type Number, returns TRUE, else FALSE. Evaluators need not have 
/// a distinguished Logical type; in such evaluators, ISNUMBER(TRUE()) is TRUE. 
/// This function does not propagate Error values.
///
/// See also: "ISTEXT", "ISLOGICAL", 
#[inline]
pub fn isnumber<A: Scalar>(x: A) -> FnLogical1<A> {
    FnLogical1("ISNUMBER", x)
}

/// Return TRUE if the value is even, else return FALSE.
/// Syntax: ISODD( X Number; )
///
/// Constraints:
/// None
///
/// Semantics:
/// First, compute X1 = TRUNC(X). Then, if X1 is odd (a division by 2 has a 
/// remainder of 1), return TRUE, else return FALSE. The result is 
/// implementation-defined if given a Logical value; an evaluator may return 
/// either an Error or the result of converting the Logical value to a Number 
/// (per Conversion to Number 6.3.5 ).
///
/// See also: "ISEVEN", "TRUNC", 
#[inline]
pub fn isodd<A: Number>(x: A) -> FnLogical1<A> {
    FnLogical1("ISODD", x)
}

/// Return TRUE if the parameter is of type reference, else return FALSE.
/// Syntax: ISREF( X Any; )
///
/// Constraints:
/// None
///
/// Semantics:
/// If X is of type Reference or ReferenceList, return TRUE, else return FALSE. 
/// Note that unlike nearly all other functions, when given a reference this 
/// function does not then examine the value being referenced. Some functions 
/// and operators return references, and thus ISREF will return TRUE when given 
/// their results. X may be a ReferenceList, in which case ISREF returns TRUE. 
/// This function does not propagate Error values.
///
/// See also: "ISNUMBER", "ISTEXT", 
#[inline]
pub fn isref<A: Any>(x: A) -> FnLogical1<A> {
    FnLogical1("ISREF", x)
}

/// Return TRUE if the parameter has type Text, else return FALSE.
/// 
/// ISTEXT(X) is equivalent to NOT(ISNONTEXT(X)).
/// Syntax: ISTEXT( X Scalar; )
///
/// Constraints:
/// None
///
/// Semantics:
/// If X is of type Text, returns TRUE, else FALSE. References to empty cells 
/// are NOT considered Text. If X is a reference, examines what X references. 
/// References to empty cells are NOT considered Text, so a reference to an 
/// empty cell will return FALSE. Empty Cell 4.7 This function does not 
/// propagate Error values.
///
/// See also: "ISNONTEXT", "ISNUMBER", "ISLOGICAL", 
#[inline]
pub fn istext<A: Scalar>(x: A) -> FnLogical1<A> {
    FnLogical1("ISTEXT", x)
}

/// Return the number of a value.
/// Syntax: N( X Any; )
///
/// Constraints:
/// None
///
/// Semantics:
/// If X is a Reference, it is first dereferenced to a scalar. Then its type is 
/// examined. If it is of type Number, it is returned. If it is of type 
/// Logical, 1 is returned if TRUE else 0 is returned. It is 
/// implementation-defined what happens if it is provided a Text value.
///
/// See also: "T", "VALUE", 
#[inline]
pub fn n<A: Any>(x: A) -> FnNumber1<A> {
    FnNumber1("N", x)
}

/// Return the constant Error value #N/A.
/// Syntax: NA( )
///
/// Constraints:
/// Shall have 0 parameters
///
/// Semantics:
/// This function takes no arguments and returns the Error #N/A.
///
/// See also: "ERROR.TYPE", "ISERROR", 
#[inline]
pub fn na() -> FnAny0 {
    FnAny0("NA", )
}

/// Convert text to number, in a locale-independent way.
/// Syntax: NUMBERVALUE( X Text; )
///
/// Constraints:
/// LEN(DecimalSeparator) = 1, DecimalSeparator shall not appear in 
/// GroupSeparator
///
/// Semantics:
/// Converts given Text value X into Number. If X is a Reference, it is first 
/// dereferenced.
/// 
/// X is transformed according to the following rules:
/// 
/// 1.Starting from the beginning, remove all occurrences of the group 
/// separator before any decimal separator
/// 
/// 2.Starting from the beginning, replace the first occurrence in the text of 
/// the decimal separator character with the FULL STOP (U+002E) character
/// 
/// 3.Remove all whitespace characters (5.14).
/// 
/// 4.If the first character of the resulting string is a period FULL STOP 
/// (U+002E) then prepend a zero
/// 
/// 5.If the string ends in one or more instances of PERCENT SIGN (U+0025) , 
/// remove the percent sign(s)
/// 
/// If percent signs were removed in step 5, divide the value of the returned 
/// number by 100 for each percent sign removed. If the resulting string is a 
/// valid xsd:float, then return the number corresponding to that string, 
/// according to the definition provided in XML Schema, Part 2, Section 3.2.4.
/// 
/// If the string is not a valid xsd:float then return an error.
///
/// See also: "N", "T", "DATEVALUE", "TIMEVALUE", "VALUE", 
#[inline]
pub fn numbervalue<A: Text>(x: A) -> FnNumber1<A> {
    FnNumber1("NUMBERVALUE", x)
}

/// Convert text to number, in a locale-independent way.
/// Syntax: NUMBERVALUE( X Text;[; DecimalSeparator Text] )
///
/// Constraints:
/// LEN(DecimalSeparator) = 1, DecimalSeparator shall not appear in 
/// GroupSeparator
///
/// Semantics:
/// Converts given Text value X into Number. If X is a Reference, it is first 
/// dereferenced.
/// 
/// X is transformed according to the following rules:
/// 
/// 1.Starting from the beginning, remove all occurrences of the group 
/// separator before any decimal separator
/// 
/// 2.Starting from the beginning, replace the first occurrence in the text of 
/// the decimal separator character with the FULL STOP (U+002E) character
/// 
/// 3.Remove all whitespace characters (5.14).
/// 
/// 4.If the first character of the resulting string is a period FULL STOP 
/// (U+002E) then prepend a zero
/// 
/// 5.If the string ends in one or more instances of PERCENT SIGN (U+0025) , 
/// remove the percent sign(s)
/// 
/// If percent signs were removed in step 5, divide the value of the returned 
/// number by 100 for each percent sign removed. If the resulting string is a 
/// valid xsd:float, then return the number corresponding to that string, 
/// according to the definition provided in XML Schema, Part 2, Section 3.2.4.
/// 
/// If the string is not a valid xsd:float then return an error.
///
/// See also: "N", "T", "DATEVALUE", "TIMEVALUE", "VALUE", 
#[inline]
pub fn numbervalue_<A: Text, B: Text>(x: A, decimal_separator: B) -> FnNumber2<A, B> {
    FnNumber2("NUMBERVALUE", x, decimal_separator)
}

/// Convert text to number, in a locale-independent way.
/// Syntax: NUMBERVALUE( X Text;[; DecimalSeparator Text][; GroupSeparator Text] )
///
/// Constraints:
/// LEN(DecimalSeparator) = 1, DecimalSeparator shall not appear in 
/// GroupSeparator
///
/// Semantics:
/// Converts given Text value X into Number. If X is a Reference, it is first 
/// dereferenced.
/// 
/// X is transformed according to the following rules:
/// 
/// 1.Starting from the beginning, remove all occurrences of the group 
/// separator before any decimal separator
/// 
/// 2.Starting from the beginning, replace the first occurrence in the text of 
/// the decimal separator character with the FULL STOP (U+002E) character
/// 
/// 3.Remove all whitespace characters (5.14).
/// 
/// 4.If the first character of the resulting string is a period FULL STOP 
/// (U+002E) then prepend a zero
/// 
/// 5.If the string ends in one or more instances of PERCENT SIGN (U+0025) , 
/// remove the percent sign(s)
/// 
/// If percent signs were removed in step 5, divide the value of the returned 
/// number by 100 for each percent sign removed. If the resulting string is a 
/// valid xsd:float, then return the number corresponding to that string, 
/// according to the definition provided in XML Schema, Part 2, Section 3.2.4.
/// 
/// If the string is not a valid xsd:float then return an error.
///
/// See also: "N", "T", "DATEVALUE", "TIMEVALUE", "VALUE", 
#[inline]
pub fn numbervalue__<A: Text, B: Text, C: Text>(x: A, decimal_separator: B, group_separator: C) -> FnNumber3<A, B, C> {
    FnNumber3("NUMBERVALUE", x, decimal_separator, group_separator)
}

/// Returns the row number(s) of a reference.
/// Syntax: ROW( )
///
/// Constraints:
/// AREAS(R) = 1
///
/// Semantics:
/// Returns the row number of a reference. If no parameter is given, the 
/// current cell is used. If a reference has multiple rows, an array of numbers 
/// is returned with all of the rows in the reference.
///
/// See also: "AREAS", "COLUMN", "SHEET", 
#[inline]
pub fn row() -> FnNumber0 {
    FnNumber0("ROW", )
}

/// Returns the row number(s) of a reference.
/// Syntax: ROW([ R Reference] )
///
/// Constraints:
/// AREAS(R) = 1
///
/// Semantics:
/// Returns the row number of a reference. If no parameter is given, the 
/// current cell is used. If a reference has multiple rows, an array of numbers 
/// is returned with all of the rows in the reference.
///
/// See also: "AREAS", "COLUMN", "SHEET", 
#[inline]
pub fn row_<A: Reference>(r: A) -> FnNumber1<A> {
    FnNumber1("ROW", r)
}

/// Returns the number of rows in a given range.
/// Syntax: ROWS( R Reference|Array; )
///
/// Constraints:
/// None
///
/// Semantics:
/// The result is not dependent on the cell content in the range.
///
/// See also: "COLUMNS", 
#[inline]
pub fn rows<A: ReferenceOrArray>(r: A) -> FnNumber1<A> {
    FnNumber1("ROWS", r)
}

/// Returns the sheet number of the reference or the string representing a 
/// sheet name.
/// Syntax: SHEET( )
///
/// Constraints:
/// R shall not contain a Source Location (5.8 References)
///
/// Semantics:
/// Returns the 1-based sheet number of the given reference or sheet name.
/// 
/// Hidden sheets are not excluded from the sheet count.
/// 
/// If no parameter is given, the result is the sheet number of the sheet 
/// containing the formula.
/// 
/// If a Reference is given it is not dereferenced.
/// 
/// If the reference encompasses more than one sheet, the result is the number 
/// of the first sheet in the range.
/// 
/// If a reference does not contain a sheet reference, the result is the sheet 
/// number of the sheet containing the formula.
/// 
/// If the function is not evaluated within a table cell, an error is returned.
///
/// See also: "COLUMN", "ROW", "SHEETS", 
#[inline]
pub fn sheet() -> FnNumber0 {
    FnNumber0("SHEET", )
}

/// Returns the sheet number of the reference or the string representing a 
/// sheet name.
/// Syntax: SHEET([ R Text|Reference] )
///
/// Constraints:
/// R shall not contain a Source Location (5.8 References)
///
/// Semantics:
/// Returns the 1-based sheet number of the given reference or sheet name.
/// 
/// Hidden sheets are not excluded from the sheet count.
/// 
/// If no parameter is given, the result is the sheet number of the sheet 
/// containing the formula.
/// 
/// If a Reference is given it is not dereferenced.
/// 
/// If the reference encompasses more than one sheet, the result is the number 
/// of the first sheet in the range.
/// 
/// If a reference does not contain a sheet reference, the result is the sheet 
/// number of the sheet containing the formula.
/// 
/// If the function is not evaluated within a table cell, an error is returned.
///
/// See also: "COLUMN", "ROW", "SHEETS", 
#[inline]
pub fn sheet_<A: TextOrReference>(r: A) -> FnNumber1<A> {
    FnNumber1("SHEET", r)
}

/// Returns the number of sheets in a reference or current document.
/// Syntax: SHEETS( )
///
/// Constraints:
/// R shall not contain a Source Location (5.8 References)
///
/// Semantics:
/// Returns the number of sheets in the given reference.
/// 
/// If no parameter is given, the number of sheets in the document is returned.
/// 
/// Hidden sheets are not excluded from the sheet count.
///
/// See also: "COLUMNS", "ROWS", "SHEET", 
#[inline]
pub fn sheets() -> FnNumber0 {
    FnNumber0("SHEETS", )
}

/// Returns the number of sheets in a reference or current document.
/// Syntax: SHEETS([ R Reference] )
///
/// Constraints:
/// R shall not contain a Source Location (5.8 References)
///
/// Semantics:
/// Returns the number of sheets in the given reference.
/// 
/// If no parameter is given, the number of sheets in the document is returned.
/// 
/// Hidden sheets are not excluded from the sheet count.
///
/// See also: "COLUMNS", "ROWS", "SHEET", 
#[inline]
pub fn sheets_<A: Reference>(r: A) -> FnNumber1<A> {
    FnNumber1("SHEETS", r)
}

/// Returns a number indicating the type of the provided value.
/// Syntax: TYPE( Value Any; )
///
/// Constraints:
/// None
///
/// Semantics:
/// Returns a number indicating the type of the value given:
/// 
/// Table 19 - TYPE
/// 
/// Value's Type
/// 
/// TYPE Return
/// 
/// Number
/// 
/// 1
/// 
/// Text
/// 
/// 2
/// 
/// Logical
/// 
/// 4
/// 
/// Error
/// 
/// 16
/// 
/// Array
/// 
/// 64
/// 
/// If a Reference is provided, the reference is first dereferenced, and any 
/// formulas are evaluated. This function does not propagate Error values.
///
/// Note:
/// Reliance on the return of 4 for TYPE will impair the interoperability of a 
/// document containing an expression that relies on that value.
///
/// See also: "ERROR.TYPE", 
#[inline]
pub fn type_<A: Any>(value: A) -> FnNumber1<A> {
    FnNumber1("TYPE", value)
}

/// Convert text to number.
/// Syntax: VALUE( X Text; )
///
/// Constraints:
/// None
///
/// Semantics:
/// Converts given text value X into Number. If X is a Reference, it is first 
/// dereferenced. It is implementation-defined what happens if VALUE is given 
/// neither a Text value nor a Reference to a Text value. If the Text has a 
/// date, time, or datetime format, it is converted into a serial Number. In 
/// many cases the conversion of a date or datetime format is locale-dependent.
/// 
/// If the supplied text X cannot be converted into a Number, an Error is 
/// returned.
/// 
/// Regardless of the current locale, an evaluator shall accept numbers 
/// matching this regular expression (which does not include a decimal point 
/// character) and convert it into a Number. If the value ends in %, it shall 
/// divide the number by 100:
/// 
/// [+-]? [0-9]+([eE][+-]?[0-9]+)?)%?
/// 
/// VALUE shall accept text representations of numbers in the current locale. 
/// In the en_US locale, an evaluator shall accept decimal numbers matching 
/// this regular expression and convert it into a Number (the leading “$” 
/// is ignored; commas are ignored if they match the rule of a thousands 
/// separator; if the value ends in %, it shall divide the number by 100):
/// 
/// [+-]?\$?([0-9]+(,[0-9]{3})*)?(\.[0-9]+)?(([eE][+-]?[0-9]+)|%)?
/// 
/// Evaluators shall accept fractional values matching the regular expression:
/// 
/// [+-]? [0-9]+ \ [0-9]+/[1-9][0-9]?
/// 
/// A leading minus sign shall be interpreted as identifying a negative number 
/// for the entire value. There is a space between the integer and the 
/// fractional portion; values between 0 and 1 can be represented by using 0 
/// for the integer part.
/// 
/// Evaluators shall support time values in at least the HH:MM and HH:MM:SS 
/// formats, where HH is a 1-2 digit value from 0 to 23, MM is a one- or 
/// two-digit value from 0 to 59, and SS is a one- or two-digit value from 0 to 
/// 59. The hour may be one or two digits when it is less than 10. VALUE 
/// converts time values into Numbers ranging from 0 to 1, which is percentage 
/// of day that has elapsed by that time. Thus, VALUE("2:00") is the same as 
/// 2/24. Evaluators should accept times with fractional seconds as well when 
/// expressed in the form HH:MM:SS.ssss...
/// 
/// Evaluators shall accept textual dates in [ISO8601] format (YYYY-MM-DD), 
/// converting them into serial numbers based on the current epoch. Evaluators 
/// shall, when running in the en_US locale, accept the format MM/DD/YYYY .
/// 
/// In addition, in locale en_US, evaluators shall support the following 
/// formats (where YYYY is a 4-digit year, YY a 2-digit year, MM a numerical 
/// month, DD a numerical day, mmm a 3-character abbreviated alphabetical name, 
/// and mmmmm a full name):
/// 
/// Table 20 - VALUE
/// 
/// Format
/// 
/// Example
/// 
/// Comment
/// 
/// MM/DD/YYYY
/// 
/// 5/21/2006
/// 
/// LOCALE-DEPENDENT; Long year format with slashes.
/// 
/// MM/DD/YY
/// 
/// 5/21/06
/// 
/// LOCALE-DEPENDENT; Short year format with slashes
/// 
/// MM-DD-YYYY
/// 
/// 5-21-2006
/// 
/// LOCALE-DEPENDENT; Long year format with dashes (short year may be 
/// supported, but it may also be used for years less than 100 .
/// 
/// mmm DD, YYYY
/// 
/// Oct 29, 2006
/// 
/// LOCALE-DEPENDENT; Short alphabetic month day, year.
///
/// Note:
/// mmm depends on the locale's language.
/// 
/// DD mmm YYYY
/// 
/// 29 Oct 2006
/// 
/// LOCALE-DEPENDENT; Short alphabetic day month year
/// 
/// mmmmm DD, YYYY
/// 
/// October 29, 2006
/// 
/// LOCALE-DEPENDENT; Long alphabetic month day, year
/// 
/// DD mmmmm YYYY
/// 
/// 29 October 2006
/// 
/// LOCALE-DEPENDENT; Long alphabetic day month year
/// 
/// Evaluators should support other locales. Many conversions will vary by 
/// locale, including the decimal point (comma or period), names of months, 
/// date formats (MM/DD vs. DD/MM), and so on. Dates in particular vary by 
/// locale.
/// 
/// Evaluators shall support the datetime format, which is a date followed by a 
/// time, using either the space character or the literal “T” character as 
/// the separator (the “T” is from ISO 8601). Evaluators shall support at 
/// least the ISO date format in a datetime format; they may support other date 
/// formats in a datetime format as well. Formats such as “YYYY-MM-DD 
/// HH:MM” and “YYYY-MM-DDTHH:MM:SS” (where “T” is the literal 
/// character T) shall be accepted. The result of accepting a datetime format 
/// shall be a representation of that specific time (without removing either 
/// the date or the time of day, unlike DATEVALUE or TIMEVALUE).
/// 
/// Evaluators may accept other formats that will convert to numbers, and those 
/// conversions may be locale-dependent, as long as they do not conflict with 
/// the above. Where no conversion is determined, an Error is returned.
///
/// See also: "N", "T", "DATEVALUE", "TIMEVALUE", "NUMBERVALUE", 
#[inline]
pub fn value<A: Text>(x: A) -> FnNumber1<A> {
    FnNumber1("VALUE", x)
}
