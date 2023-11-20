//! 
//! Information functions provide information about a data value, the 
//! spreadsheet, or underlying environment, including special functions for 
//! converting between data types.

use crate::*;
#[allow(unused_imports)]
use crate::info::*;

/// Returns the number of areas in a given list of references.
///
/// __Syntax__: 
/// ```ods
///     AREAS( R ReferenceList )
/// ```
///
/// __Constraints__:
/// None
///
/// __Semantics__:
/// Returns the number of areas in the reference list R.
///
/// __See also__: "Infix Operator Reference Concatenation", "INDEX", 
#[inline]
pub fn areas<A: Reference>(r: A) -> FnNumber1<A> {
    FnNumber1("AREAS", r)
}

/// Returns information about position, formatting or contents in a reference.
///
/// __Syntax__: 
/// ```ods
///     CELL( Info_Type Text )
/// ```
///
/// __Constraints__:
/// None
///
/// __Semantics__:
/// The parameters are
/// 
/// •Info_Type: the text string which specifies the type of information. 
/// Please refer to Table 17 - CELL.
/// 
/// •R : if R is a reference to a cell, it is the cell whose information will 
/// be returned; if R is a reference to a range, the top-left cell in the range 
/// is the selected one; if R is omitted, the current cell is used.
#[inline]
pub fn cell<>(info_type: CellInfo) -> FnAny1<CellInfo> {
    FnAny1("CELL", info_type)
}

/// Returns information about position, formatting or contents in a reference.
///
/// __Syntax__: 
/// ```ods
///     CELL( Info_Type Text; R Reference )
/// ```
///
/// __Constraints__:
/// None
///
/// __Semantics__:
/// The parameters are
/// 
/// •Info_Type: the text string which specifies the type of information. 
/// Please refer to Table 17 - CELL.
/// 
/// •R : if R is a reference to a cell, it is the cell whose information will 
/// be returned; if R is a reference to a range, the top-left cell in the range 
/// is the selected one; if R is omitted, the current cell is used.
#[inline]
pub fn cell_<A: Reference>(info_type: CellInfo, r: A) -> FnAny2<CellInfo, A> {
    FnAny2("CELL", info_type, r)
}

/// Returns the column number(s) of a reference.
///
/// __Syntax__: 
/// ```ods
///     COLUMN( )
/// ```
///
/// __Constraints__:
/// AREAS(R) = 1
///
/// __Semantics__:
/// Returns the column number of a reference, where “A” is 1, “B” is 2, 
/// and so on. If no parameter is given, the current cell is used. If a 
/// reference has multiple columns, an array of numbers is returned with all of 
/// the columns in the reference.
///
/// __See also__: "AREAS", "ROW", "SHEET", 
#[inline]
pub fn column() -> FnNumber0 {
    FnNumber0("COLUMN", )
}

/// Returns the column number(s) of a reference.
///
/// __Syntax__: 
/// ```ods
///     COLUMN( R Reference )
/// ```
///
/// __Constraints__:
/// AREAS(R) = 1
///
/// __Semantics__:
/// Returns the column number of a reference, where “A” is 1, “B” is 2, 
/// and so on. If no parameter is given, the current cell is used. If a 
/// reference has multiple columns, an array of numbers is returned with all of 
/// the columns in the reference.
///
/// __See also__: "AREAS", "ROW", "SHEET", 
#[inline]
pub fn column_<A: Reference>(r: A) -> FnNumber1<A> {
    FnNumber1("COLUMN", r)
}

/// Returns the number of columns in a given range.
///
/// __Syntax__: 
/// ```ods
///     COLUMNS( R Reference|Array )
/// ```
///
/// __Constraints__:
/// None
///
/// __Semantics__:
/// Returns the number of columns in the range or array specified. The result 
/// is not dependent on the cell content in the range.
///
/// __See also__: "ROWS", 
#[inline]
pub fn columns<A: ReferenceOrArray>(r: A) -> FnNumber1<A> {
    FnNumber1("COLUMNS", r)
}

/// Count the number of Numbers provided.
///
/// __Syntax__: 
/// ```ods
///     COUNT({ N NumberSequenceList}+ )
/// ```
///
/// __Constraints__:
/// One or more parameters.
///
/// __Semantics__:
/// Counts the numbers in the list N. Only numbers in references are counted; 
/// all other types are ignored. Errors are not propagated. It is 
/// implementation-defined what happens if 0 parameters are passed, but it 
/// should be an Error or 0.
///
/// __See also__: "COUNTA", 
#[inline]
pub fn count<A: Sequence>(n: A) -> FnNumber1<A> {
    FnNumber1("COUNT", n)
}

/// Count the number of non-empty values.
///
/// __Syntax__: 
/// ```ods
///     COUNTA({ AnyValue Any}+ )
/// ```
///
/// __Constraints__:
/// None.
///
/// __Semantics__:
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
/// __See also__: "COUNT", "ISBLANK", 
#[inline]
pub fn counta<A: Sequence>(any_value: A) -> FnNumber1<A> {
    FnNumber1("COUNTA", any_value)
}

/// Count the number of blank cells.
///
/// __Syntax__: 
/// ```ods
///     COUNTBLANK( R ReferenceList )
/// ```
///
/// __Constraints__:
/// None.
///
/// __Semantics__:
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
/// __See also__: "COUNT", "COUNTA", "COUNTIF", "ISBLANK", 
#[inline]
pub fn countblank<A: Reference>(r: A) -> FnNumber1<A> {
    FnNumber1("COUNTBLANK", r)
}

/// Count the number of cells in a range that meet a criteria.
///
/// __Syntax__: 
/// ```ods
///     COUNTIF( R ReferenceList; C Criterion )
/// ```
///
/// __Constraints__:
/// Does not accept constant values as the reference parameter.
///
/// __Semantics__:
/// Counts the number of cells in the reference range R that meet the Criterion 
/// C (4.11.8).
/// 
/// The values returned may vary depending upon the 
/// HOST-USE-REGULAR-EXPRESSIONS or HOST-USE-WILDCARDS or 
/// HOST-SEARCH-CRITERIA-MUST-APPLY-TO-WHOLE-CELL properties. 3.4
///
/// __See also__: "COUNT", "COUNTA", "COUNTBLANK", "COUNTIFS", "SUMIF", "Infix Operator \"=\"", "Infix Operator \"<>\"", "Infix Operator Ordered Comparison (\"<\", \"<=\", \">\", \">=\")", 
#[inline]
pub fn countif<A: Reference, B: Criterion>(r: A, c: B) -> FnNumber2<A, B> {
    FnNumber2("COUNTIF", r, c)
}

/// Returns Number representing the specific Error type.
///
/// __Syntax__: 
/// ```ods
///     ERROR.TYPE( E Error )
/// ```
///
/// __Constraints__:
/// None.
///
/// __Semantics__:
/// Returns a number representing what kind of Error has occurred. Note that 
/// unlike most functions, this function does not propagate Error values. 
/// Receiving a non-Error value returns an Error. In particular, 
/// ERROR.TYPE(NA()) returns 7, and ERROR.TYPE applied to a non-Error returns 
/// an Error.
///
/// __See also__: "NA", 
#[inline]
pub fn error_type<A: Any>(e: A) -> FnNumber1<A> {
    FnNumber1("ERROR.TYPE", e)
}

/// Returns formula at given reference as text.
///
/// __Syntax__: 
/// ```ods
///     FORMULA( X Reference )
/// ```
///
/// __Constraints__:
/// Reference X shall contain a formula.
///
/// __Semantics__:
/// Returns the formula in reference X as a string. The specific syntax of this 
/// returned string is implementation-defined. This function is intended to aid 
/// debugging by simplifying display of formulas in other cells. Error results 
/// of the referred formula cell are not propagated.
///
/// __See also__: "ISFORMULA", 
#[inline]
pub fn formula<A: Reference>(x: A) -> FnText1<A> {
    FnText1("FORMULA", x)
}

/// Returns information about the environment.
///
/// __Syntax__: 
/// ```ods
///     INFO( Category Text )
/// ```
///
/// __Constraints__:
/// Category shall be valid.
///
/// __Semantics__:
/// Returns information about the environment in the given category.
///
/// __See also__: "CELL", 
#[inline]
pub fn info<>(category: InfoInfo) -> FnAny1<InfoInfo> {
    FnAny1("INFO", category)
}

/// Return TRUE if the referenced cell is blank, else return FALSE.
///
/// __Syntax__: 
/// ```ods
///     ISBLANK( X Scalar )
/// ```
///
/// __Constraints__:
/// None
///
/// __Semantics__:
/// If X is of type Number, Text, or Logical, return FALSE. If X is a reference 
/// to a cell, examine the cell; if it is blank (has no value), return TRUE, 
/// but if it has a value, return FALSE. A cell with the empty string is not 
/// considered blank. This function does not propagate Error values.
///
/// __See also__: "ISNUMBER", "ISTEXT", 
#[inline]
pub fn isblank<A: Scalar>(x: A) -> FnLogical1<A> {
    FnLogical1("ISBLANK", x)
}

/// Return TRUE if the parameter has type Error and is not #N/A, else return 
/// FALSE.
///
/// __Syntax__: 
/// ```ods
///     ISERR( X Scalar )
/// ```
///
/// __Constraints__:
/// None
///
/// __Semantics__:
/// If X is of type Error, and ISNA(X) is not true, returns TRUE. Otherwise it 
/// returns FALSE. Note that this function returns FALSE if given #N/A; if this 
/// is not desired, use ISERROR 6.13.16. Note that this function does not 
/// propagate Error values.
/// 
/// ISERR(X) is the same as:
/// 
/// IF(ISNA(X),FALSE(),ISERROR(X))
///
/// __See also__: "ERROR.TYPE", "ISERROR", "ISNA", "ISNUMBER", "ISTEXT", "NA", 
#[inline]
pub fn iserr<A: Scalar>(x: A) -> FnLogical1<A> {
    FnLogical1("ISERR", x)
}

/// Return TRUE if the parameter has type Error, else return FALSE.
///
/// __Syntax__: 
/// ```ods
///     ISERROR( X Scalar )
/// ```
///
/// __Constraints__:
/// None
///
/// __Semantics__:
/// If X is of type Error, returns TRUE, else returns FALSE. Note that this 
/// function returns TRUE if given #N/A; if this is not desired, use ISERR 
/// 6.13.15. Note that this function does not propagate Error values.
///
/// __See also__: "ERROR.TYPE", "ISERR", "ISNA", "ISNUMBER", "ISTEXT", "NA", 
#[inline]
pub fn iserror<A: Scalar>(x: A) -> FnLogical1<A> {
    FnLogical1("ISERROR", x)
}

/// Return TRUE if the value is even, else return FALSE.
///
/// __Syntax__: 
/// ```ods
///     ISEVEN( X Number )
/// ```
///
/// __Constraints__:
/// None
///
/// __Semantics__:
/// First, compute X1 = TRUNC(X). Then, if X1 is even (a division by 2 has a 
/// remainder of 0), return TRUE, else return FALSE. The result is 
/// implementation-defined if given a Logical value; an evaluator may return 
/// either an Error or the result of converting the Logical value to a Number 
/// (per Conversion to Number 6.3.5 ).
///
/// __See also__: "ISODD", "TRUNC", 
#[inline]
pub fn iseven<A: Number>(x: A) -> FnLogical1<A> {
    FnLogical1("ISEVEN", x)
}

/// Return TRUE if the reference refers to a formula, else return FALSE.
///
/// __Syntax__: 
/// ```ods
///     ISFORMULA( X Reference )
/// ```
///
/// __Constraints__:
/// None
///
/// __Semantics__:
/// If X refers to a cell whose value is computed by a formula, return TRUE, 
/// else return FALSE. A formula itself may compute a constant; in that case it 
/// will still return TRUE since it is still a formula. Passing a 
/// non-reference, or a reference to more than one cell, is 
/// implementation-defined. This function does not propagate Error values.
///
/// __See also__: "ISTEXT", "ISNUMBER", 
#[inline]
pub fn isformula<A: Reference>(x: A) -> FnLogical1<A> {
    FnLogical1("ISFORMULA", x)
}

/// Return TRUE if the parameter has type Logical, else return FALSE.
///
/// __Syntax__: 
/// ```ods
///     ISLOGICAL( X Scalar )
/// ```
///
/// __Constraints__:
/// None
///
/// __Semantics__:
/// If X is of type Logical, returns TRUE, else FALSE. Evaluators that do not 
/// have a distinct Logical type will return the same value ISNUMBER(X) would 
/// return. This function does not propagate Error values.
///
/// __See also__: "ISTEXT", "ISNUMBER", 
#[inline]
pub fn islogical<A: Scalar>(x: A) -> FnLogical1<A> {
    FnLogical1("ISLOGICAL", x)
}

/// Return TRUE if the parameter has type Error and is #N/A, else return FALSE.
///
/// __Syntax__: 
/// ```ods
///     ISNA( X Scalar )
/// ```
///
/// __Constraints__:
/// None
///
/// __Semantics__:
/// If X is #N/A, return TRUE, else return FALSE. Note that if X is a 
/// reference, the value being referenced is considered. This function does not 
/// propagate Error values.
///
/// __See also__: "ERROR.TYPE", "ISERROR", "ISERR", "ISNUMBER", "ISTEXT", "NA", 
#[inline]
pub fn isna<A: Scalar>(x: A) -> FnLogical1<A> {
    FnLogical1("ISNA", x)
}

/// Return TRUE if the parameter does not have type Text, else return FALSE.
///
/// __Syntax__: 
/// ```ods
///     ISNONTEXT( X Scalar )
/// ```
///
/// __Constraints__:
/// None
///
/// __Semantics__:
/// If X is of type Text, ISNONTEXT returns FALSE, else TRUE. If X is a 
/// reference, it examines what X references. References to empty cells are not 
/// considered text, so for reference to an empty cell ISNONTEXT will return 
/// TRUE. Empty Cell 4.7 This function does not propagate Error values.
/// 
/// ISNONTEXT(X) is equivalent to NOT(ISTEXT(X))
///
/// __See also__: "ISNUMBER", "ISLOGICAL", "ISTEXT", "NOT", 
#[inline]
pub fn isnontext<A: Scalar>(x: A) -> FnLogical1<A> {
    FnLogical1("ISNONTEXT", x)
}

/// Return TRUE if the parameter has type Number, else return FALSE.
///
/// __Syntax__: 
/// ```ods
///     ISNUMBER( X Scalar )
/// ```
///
/// __Constraints__:
/// None
///
/// __Semantics__:
/// If X is of type Number, returns TRUE, else FALSE. Evaluators need not have 
/// a distinguished Logical type; in such evaluators, ISNUMBER(TRUE()) is TRUE. 
/// This function does not propagate Error values.
///
/// __See also__: "ISTEXT", "ISLOGICAL", 
#[inline]
pub fn isnumber<A: Scalar>(x: A) -> FnLogical1<A> {
    FnLogical1("ISNUMBER", x)
}

/// Return TRUE if the value is even, else return FALSE.
///
/// __Syntax__: 
/// ```ods
///     ISODD( X Number )
/// ```
///
/// __Constraints__:
/// None
///
/// __Semantics__:
/// First, compute X1 = TRUNC(X). Then, if X1 is odd (a division by 2 has a 
/// remainder of 1), return TRUE, else return FALSE. The result is 
/// implementation-defined if given a Logical value; an evaluator may return 
/// either an Error or the result of converting the Logical value to a Number 
/// (per Conversion to Number 6.3.5 ).
///
/// __See also__: "ISEVEN", "TRUNC", 
#[inline]
pub fn isodd<A: Number>(x: A) -> FnLogical1<A> {
    FnLogical1("ISODD", x)
}

/// Return TRUE if the parameter is of type reference, else return FALSE.
///
/// __Syntax__: 
/// ```ods
///     ISREF( X Any )
/// ```
///
/// __Constraints__:
/// None
///
/// __Semantics__:
/// If X is of type Reference or ReferenceList, return TRUE, else return FALSE. 
/// Note that unlike nearly all other functions, when given a reference this 
/// function does not then examine the value being referenced. Some functions 
/// and operators return references, and thus ISREF will return TRUE when given 
/// their results. X may be a ReferenceList, in which case ISREF returns TRUE. 
/// This function does not propagate Error values.
///
/// __See also__: "ISNUMBER", "ISTEXT", 
#[inline]
pub fn isref<A: Any>(x: A) -> FnLogical1<A> {
    FnLogical1("ISREF", x)
}

/// Return TRUE if the parameter has type Text, else return FALSE.
/// 
/// ISTEXT(X) is equivalent to NOT(ISNONTEXT(X)).
///
/// __Syntax__: 
/// ```ods
///     ISTEXT( X Scalar )
/// ```
///
/// __Constraints__:
/// None
///
/// __Semantics__:
/// If X is of type Text, returns TRUE, else FALSE. References to empty cells 
/// are NOT considered Text. If X is a reference, examines what X references. 
/// References to empty cells are NOT considered Text, so a reference to an 
/// empty cell will return FALSE. Empty Cell 4.7 This function does not 
/// propagate Error values.
///
/// __See also__: "ISNONTEXT", "ISNUMBER", "ISLOGICAL", 
#[inline]
pub fn istext<A: Scalar>(x: A) -> FnLogical1<A> {
    FnLogical1("ISTEXT", x)
}

/// Return the number of a value.
///
/// __Syntax__: 
/// ```ods
///     N( X Any )
/// ```
///
/// __Constraints__:
/// None
///
/// __Semantics__:
/// If X is a Reference, it is first dereferenced to a scalar. Then its type is 
/// examined. If it is of type Number, it is returned. If it is of type 
/// Logical, 1 is returned if TRUE else 0 is returned. It is 
/// implementation-defined what happens if it is provided a Text value.
///
/// __See also__: "T", "VALUE", 
#[inline]
pub fn n<A: Any>(x: A) -> FnNumber1<A> {
    FnNumber1("N", x)
}

/// Return the constant Error value #N/A.
///
/// __Syntax__: 
/// ```ods
///     NA( )
/// ```
///
/// __Constraints__:
/// Shall have 0 parameters
///
/// __Semantics__:
/// This function takes no arguments and returns the Error #N/A.
///
/// __See also__: "ERROR.TYPE", "ISERROR", 
#[inline]
pub fn na() -> FnAny0 {
    FnAny0("NA", )
}

/// Convert text to number, in a locale-independent way.
///
/// __Syntax__: 
/// ```ods
///     NUMBERVALUE( X Text )
/// ```
///
/// __Constraints__:
/// LEN(DecimalSeparator) = 1, DecimalSeparator shall not appear in 
/// GroupSeparator
///
/// __Semantics__:
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
/// __See also__: "N", "T", "DATEVALUE", "TIMEVALUE", "VALUE", 
#[inline]
pub fn numbervalue<A: Text>(x: A) -> FnNumber1<A> {
    FnNumber1("NUMBERVALUE", x)
}

/// Convert text to number, in a locale-independent way.
///
/// __Syntax__: 
/// ```ods
///     NUMBERVALUE( X Text; DecimalSeparator Text )
/// ```
///
/// __Constraints__:
/// LEN(DecimalSeparator) = 1, DecimalSeparator shall not appear in 
/// GroupSeparator
///
/// __Semantics__:
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
/// __See also__: "N", "T", "DATEVALUE", "TIMEVALUE", "VALUE", 
#[inline]
pub fn numbervalue_<A: Text, B: Text>(x: A, decimal_separator: B) -> FnNumber2<A, B> {
    FnNumber2("NUMBERVALUE", x, decimal_separator)
}

/// Convert text to number, in a locale-independent way.
///
/// __Syntax__: 
/// ```ods
///     NUMBERVALUE( X Text; DecimalSeparator Text; GroupSeparator Text )
/// ```
///
/// __Constraints__:
/// LEN(DecimalSeparator) = 1, DecimalSeparator shall not appear in 
/// GroupSeparator
///
/// __Semantics__:
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
/// __See also__: "N", "T", "DATEVALUE", "TIMEVALUE", "VALUE", 
#[inline]
pub fn numbervalue__<A: Text, B: Text, C: Text>(x: A, decimal_separator: B, group_separator: C) -> FnNumber3<A, B, C> {
    FnNumber3("NUMBERVALUE", x, decimal_separator, group_separator)
}

/// Returns the row number(s) of a reference.
///
/// __Syntax__: 
/// ```ods
///     ROW( )
/// ```
///
/// __Constraints__:
/// AREAS(R) = 1
///
/// __Semantics__:
/// Returns the row number of a reference. If no parameter is given, the 
/// current cell is used. If a reference has multiple rows, an array of numbers 
/// is returned with all of the rows in the reference.
///
/// __See also__: "AREAS", "COLUMN", "SHEET", 
#[inline]
pub fn row() -> FnNumber0 {
    FnNumber0("ROW", )
}

/// Returns the row number(s) of a reference.
///
/// __Syntax__: 
/// ```ods
///     ROW( R Reference )
/// ```
///
/// __Constraints__:
/// AREAS(R) = 1
///
/// __Semantics__:
/// Returns the row number of a reference. If no parameter is given, the 
/// current cell is used. If a reference has multiple rows, an array of numbers 
/// is returned with all of the rows in the reference.
///
/// __See also__: "AREAS", "COLUMN", "SHEET", 
#[inline]
pub fn row_<A: Reference>(r: A) -> FnNumber1<A> {
    FnNumber1("ROW", r)
}

/// Returns the number of rows in a given range.
///
/// __Syntax__: 
/// ```ods
///     ROWS( R Reference|Array )
/// ```
///
/// __Constraints__:
/// None
///
/// __Semantics__:
/// The result is not dependent on the cell content in the range.
///
/// __See also__: "COLUMNS", 
#[inline]
pub fn rows<A: ReferenceOrArray>(r: A) -> FnNumber1<A> {
    FnNumber1("ROWS", r)
}

/// Returns the sheet number of the reference or the string representing a 
/// sheet name.
///
/// __Syntax__: 
/// ```ods
///     SHEET( )
/// ```
///
/// __Constraints__:
/// R shall not contain a Source Location (5.8 References)
///
/// __Semantics__:
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
/// __See also__: "COLUMN", "ROW", "SHEETS", 
#[inline]
pub fn sheet() -> FnNumber0 {
    FnNumber0("SHEET", )
}

/// Returns the sheet number of the reference or the string representing a 
/// sheet name.
///
/// __Syntax__: 
/// ```ods
///     SHEET( R Text|Reference )
/// ```
///
/// __Constraints__:
/// R shall not contain a Source Location (5.8 References)
///
/// __Semantics__:
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
/// __See also__: "COLUMN", "ROW", "SHEETS", 
#[inline]
pub fn sheet_<A: TextOrReference>(r: A) -> FnNumber1<A> {
    FnNumber1("SHEET", r)
}

/// Returns the number of sheets in a reference or current document.
///
/// __Syntax__: 
/// ```ods
///     SHEETS( )
/// ```
///
/// __Constraints__:
/// R shall not contain a Source Location (5.8 References)
///
/// __Semantics__:
/// Returns the number of sheets in the given reference.
/// 
/// If no parameter is given, the number of sheets in the document is returned.
/// 
/// Hidden sheets are not excluded from the sheet count.
///
/// __See also__: "COLUMNS", "ROWS", "SHEET", 
#[inline]
pub fn sheets() -> FnNumber0 {
    FnNumber0("SHEETS", )
}

/// Returns the number of sheets in a reference or current document.
///
/// __Syntax__: 
/// ```ods
///     SHEETS( R Reference )
/// ```
///
/// __Constraints__:
/// R shall not contain a Source Location (5.8 References)
///
/// __Semantics__:
/// Returns the number of sheets in the given reference.
/// 
/// If no parameter is given, the number of sheets in the document is returned.
/// 
/// Hidden sheets are not excluded from the sheet count.
///
/// __See also__: "COLUMNS", "ROWS", "SHEET", 
#[inline]
pub fn sheets_<A: Reference>(r: A) -> FnNumber1<A> {
    FnNumber1("SHEETS", r)
}

/// Returns a number indicating the type of the provided value.
///
/// __Syntax__: 
/// ```ods
///     TYPE( Value Any )
/// ```
///
/// __Constraints__:
/// None
///
/// __Semantics__:
/// Returns a number indicating the type of the value given:
/// 
/// If a Reference is provided, the reference is first dereferenced, and any 
/// formulas are evaluated. This function does not propagate Error values.
///
/// __Note__:
/// Reliance on the return of 4 for TYPE will impair the interoperability of a 
/// document containing an expression that relies on that value.
///
/// __See also__: "ERROR.TYPE", 
#[inline]
pub fn type_<A: Any>(value: A) -> FnNumber1<A> {
    FnNumber1("TYPE", value)
}

/// Convert text to number.
///
/// __Syntax__: 
/// ```ods
///     VALUE( X Text )
/// ```
///
/// __Constraints__:
/// None
///
/// __Semantics__:
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
/// __See also__: "N", "T", "DATEVALUE", "TIMEVALUE", "NUMBERVALUE", 
#[inline]
pub fn value<A: Text>(x: A) -> FnNumber1<A> {
    FnNumber1("VALUE", x)
}
